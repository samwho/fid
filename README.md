# fid

Note: this is mostly a plaything at the moment. It's not actually a
replacement for the `file` tool yet.

The plan is to create an extremely simple API for identifying a file
by its contents, that can be extended by anyone.

Each "identifier" is a function with the following signature:

```rust
pub(crate) type Identifier = fn(&mut BufReader<File>) -> Option<String>;
```

If the identifier doesn't think the file is of the type it is looking for, it
is to return `None`. Otherwise, it returns a string suitable for displaying
to the user of the program.

Identifiers live in `src/identifiers`. Have a look at existing identifiers to
get an idea for how they are laid out.

## Usage

```
$ cargo run -- src/main.rs
Rust source file
$ cargo run -- Cargo.toml
TOML file
```