use anyhow::{bail, Context, Result};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use terminal_size::{terminal_size, Height, Width};
use textwrap::wrap;

// This probably depends on the user's prompt size
const SPACE_AROUND: usize = 2;

fn main() -> Result<()> {
    let mut args = std::env::args().peekable();
    let _command = args.next();
    args.next_if(|x| x.as_str() == "firstpage");

    let mut child = Command::new("cargo")
        .env("CARGO_TERM_COLOR", "always")
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .context("could not start cargo command")?;
    let mut output = BufReader::new(child.stderr.take().unwrap());

    let mut buf = String::new();
    while output.read_line(&mut buf)? > 0 {
        if !buf.starts_with(' ') {
            break;
        }

        eprint!("{}", buf);

        buf.clear();
    }
    eprintln!();

    let (width, height) = if let Some((Width(width), Height(height))) = terminal_size() {
        (width as usize, height as usize)
    } else {
        bail!("could not get terminal size");
    };

    let mut count = 0;
    while output.read_line(&mut buf)? > 0 {
        let lines = wrap(buf.trim_end(), width);
        count += lines.len();

        if count > height - SPACE_AROUND {
            break;
        }

        for line in lines {
            eprintln!("{}", line);
        }

        buf.clear();
    }

    if let Ok(status) = child.wait() {
        std::process::exit(status.code().unwrap_or_default());
    } else {
        Ok(())
    }
}
