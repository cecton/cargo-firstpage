use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::Message;
use std::cmp;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use terminal_size::{terminal_size, Height, Width};
use textwrap::wrap;

// This probably depends on the user's prompt size
const SPACE_AROUND: usize = 4;

const HELP: &str = r#"A wrapper for Cargo that displays only the first page of errors.

USAGE:
    cargo firstpage -h|--help
    cargo firstpage [COMMAND] [args]...
"#;

/// A list of Cargo subcommands that show errors and support
/// `--message-format=json-diagnostic-rendered-ansi`.
///
/// Some Cargo subcommands can show diagnostics but do not support that option:
/// - `cargo fmt`
/// - `cargo install`
/// - `cargo package`
/// - `cargo publish`
///
/// Users are not likely to encouter diagnostics with these commands anyway however, so it's fine
/// to not attempt to cut off their output.
const SHOWS_ERRORS: &[&str] = &[
    "b", "bench", "build", "c", "check", "clippy", "d", "doc", "fix", "init", "miri", "r", "run",
    "rustc", "rustdoc", "t", "test",
];

const SPECIAL_ARGS: &[&str] = &["--help", "-h", "--version", "-V"];

fn main() -> Result<()> {
    let mut args = std::env::args().peekable();
    let _command = args.next();
    // Support running both as `cargo-firstpage` and as `cargo firstpage`.
    args.next_if(|x| x.as_str() == "firstpage");

    let subcommand_arg = args.next();
    let subcommand = match subcommand_arg.as_deref() {
        Some("--help" | "-h") | None => {
            print!("{}", HELP);
            return Ok(());
        }
        Some(subcommand) => subcommand,
    };

    let args = args.collect::<Vec<_>>();

    let (width, height) = if let Some((Width(width), Height(height))) = terminal_size() {
        (width as usize, height as usize)
    } else {
        bail!("could not get terminal size");
    };

    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));

    command.arg(subcommand);

    if SHOWS_ERRORS.contains(&subcommand) && args.iter().all(|arg| !SPECIAL_ARGS.contains(&&**arg))
    {
        command.arg("--message-format=json-diagnostic-rendered-ansi");
        // TODO: Piping stdout will not disable colors emitted by Cargo itself (as those are done
        // via stderr), but it can disable colors for programs that Cargo then spawns that use
        // stdout (such as the default test runner in `cargo test`). To solve this properly, we
        // would have to spawn these commands in a PTY and forward the PTY output.
        command.stdout(Stdio::piped());
    }

    command.args(args);

    let mut child = command.spawn().context("could not start cargo command")?;

    // `child.stdout` can be `None` if `shows_errors == false`.
    if let Some(stdout) = child.stdout.take() {
        let mut stdout = BufReader::new(stdout);

        let mut diagnostics = String::new();

        let mut buf = String::new();

        while stdout.read_line(&mut buf)? > 0 {
            match serde_json::from_str(&buf) {
                Ok(Message::CompilerMessage(msg)) => {
                    let rendered = msg
                        .message
                        .rendered
                        .context("rustc did not provide rendered message")?;
                    diagnostics.push_str(&rendered);
                }
                Ok(Message::BuildFinished(_)) => break,
                Ok(_) => {}
                Err(e) => {
                    bail!(anyhow!(e).context("could not deserialize Cargo message"));
                }
            }
            buf.clear();
        }

        // Now that Cargo has finished emitting its diagnostics, pipe the rest of stdout directly.
        io::copy(&mut stdout, &mut io::stdout().lock())?;

        // Removing trailing newlines avoids double-newlines at the end of the output.
        if diagnostics.ends_with('\n') {
            diagnostics.pop();
        }

        if !diagnostics.is_empty() {
            let lines = wrap(&diagnostics, width);
            let stderr = io::stderr();
            let mut stderr = stderr.lock();
            for line in &lines[..cmp::min(lines.len(), height - SPACE_AROUND)] {
                writeln!(stderr, "{}", line)?;
            }
        }
    }

    if let Ok(status) = child.wait() {
        std::process::exit(status.code().unwrap_or_default());
    } else {
        Ok(())
    }
}
