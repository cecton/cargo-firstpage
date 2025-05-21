[![Latest Version](https://img.shields.io/crates/v/cargo-firstpage.svg)](https://crates.io/crates/cargo-firstpage)
![License](https://img.shields.io/crates/l/cargo-firstpage)
[![LOC](https://tokei.rs/b1/github/cecton/cargo-firstpage)](https://github.com/cecton/cargo-firstpage)
[![Dependency Status](https://deps.rs/repo/github/cecton/cargo-firstpage/status.svg)](https://deps.rs/repo/github/cecton/cargo-firstpage)

# cargo-firstpage

Show only the first page of `rustc` output.

## Installation

```bash
cargo install cargo-firstpage
```

## Usage

Add `firstpage` before any `cargo` command:

### Examples

This runs `cargo check` but only shows the first page of output:

```bash
cargo firstpage check
```

This runs `cargo run -p myproject`:

```bash
cargo firstpage run -p myproject
```

All arguments after `firstpage` are passed directly to `cargo` without
exception. `cargo-firstpage` does not have any option is and not
parametrizable.

### Using with `cargo-watch`

You can use it like this:

```bash
cargo watch -x "firstpage check"
```

If you want to see just the first page of the build output while keeping your
program’s error messages:

```bash
cargo watch -x "firstpage build" -x "run"
```

For TUI (text user interface) programs, use this in one terminal:

```bash
CARGO_TERM_COLOR=always cargo watch \
    -x "firstpage build 2>/tmp/logs" \
    -x "run 2>>/tmp/logs"
```

And this in another terminal:

```bash
fail -F /tmp/logs
```

This setup keeps your program's stderr output scrolling while only showing the
first page of build messages.

## Limitations

In most cases, the useful error messages appear at the end of the output.
However, sometimes `rustc` shows important information at the beginning, like
how to disable a warning.

## Contributing

This project does not accept issues, pull requests, or other contributions.
Forks are welcome — feel free to use, modify, and build on it as needed.

## Related Tools

- [ograc](https://crates.io/crates/ograc): Shows diagnostic messages in reverse
  order, so important ones appear first. You can scroll up to see less
  important ones.
- [cargo-cut-diagnostics](https://github.com/SabrinaJewson/cargo-cut-diagnostics):
  Similar to `cargo-firstpage`, but keeps the progress bar and offers more
  options. The author claims it processes cargo output more accurately. Try
  this if you have issues with `cargo-firstpage`.
- [bacon](https://crates.io/crates/bacon): Combines features of `cargo-watch`
  with a small TUI that makes it easier to filter and read logs. Worth checking
  out if you want to improve the readability and scrolling of `rustc` output.
