use anyhow::{bail, Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use terminal_size::{terminal_size, Height, Width};
use textwrap::wrap;

const PROMPT_SIZE: Option<&str> = option_env!("PROMPT_SIZE");
const DEFAULT_PROMPT_SIZE: usize = 1;

fn main() -> Result<()> {
    let mut args = std::env::args().peekable();
    let _command = args.next();
    args.next_if(|x| x.as_str() == "firstpage");

    // Query width and height of the terminal
    let (width, height) = if let Some((Width(width), Height(height))) = terminal_size() {
        (width as usize, height as usize)
    } else {
        bail!("could not get terminal size");
    };

    // Spawn the process, ensuring cargo will output colors
    let mut child = Command::new("cargo")
        .env("CARGO_TERM_COLOR", "always")
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .context("could not start cargo command")?;
    let mut output = BufReader::new(child.stderr.take().unwrap());
    let mut buf = String::new();

    // Skip the Download/Compiling/Finished/Running lines of cargo
    while output.read_line(&mut buf)? > 0 {
        if !buf.starts_with(' ') {
            break;
        }
        eprint!("{}", buf);
        buf.clear();
    }
    eprintln!();

    // Read and display the lines immediately until the limit is reached
    let mut count = 0;
    let space_around = 1 + PROMPT_SIZE
        .map(std::str::FromStr::from_str)
        .map(Result::ok)
        .flatten()
        .unwrap_or(DEFAULT_PROMPT_SIZE);
    while output.read_line(&mut buf)? > 0 {
        let lines = wrap(buf.trim_end(), width);
        count += lines.len();
        if count > height - space_around {
            break;
        }
        for line in lines {
            eprintln!("{}", line);
        }
        buf.clear();
    }

    // Discard the rest
    let mut sink = std::io::sink();
    let _ = std::io::copy(&mut output, &mut sink);

    std::process::exit(child.wait()?.code().unwrap_or_default())
}
