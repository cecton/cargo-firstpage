cargo-first-page
================

Shows only the first page of rustc output.

Installation
------------

```
cargo install cargo-firstpage
```

Usage
-----

Prefix the cargo command by `firstpage`:

This will run `cargo check` but display only the first page:

```
cargo firstpage check
```

You can use it with cargo-watch like this:

```
cargo watch -x "firstpage check"
```
