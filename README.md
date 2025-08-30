# Rust in 3 Hours — Mini Projects

A small collection of Rust mini-projects intended for quick, hands-on learning. Each subfolder is an independent Cargo project that focuses on a core Rust concept.

## Repo Layout

- `hello_rust`: Demonstrates ownership vs. borrowing by moving and borrowing `String` values.
- `errors_demo`: Simple file I/O with `Result` and the `?` operator for error handling.
- `async_demo`: Tiny async example using `tokio` to run two tasks concurrently.
- `simple_calc`: A minimal CLI calculator built with `clap`.

## Prerequisites

- Rust toolchain with Cargo (latest stable). Projects use Edition 2024.
  - Install via https://rustup.rs if you don’t have it.

## How to Run

Each project is standalone. Change into a directory and run with Cargo.

### hello_rust

```bash
cd hello_rust
cargo run
```

### errors_demo

```bash
cd errors_demo
cargo run
```

### async_demo

```bash
cd async_demo
cargo run
```

### simple_calc

Provide two numbers and an operator: `+`, `-`, `x`/`X`, `/`.

```bash
cd simple_calc
cargo run -- 4 + 7
cargo run -- 10 x 3
cargo run -- 9 / 2
```

## Build Release Binaries

From any project directory:

```bash
cargo build --release
```

The compiled binary will be in `target/release/` for that project.

## Notes

- Projects are not part of a Cargo workspace; they run independently.
- `simple_calc` is intentionally minimal and does not handle invalid operators beyond the listed ones.
- Feel free to extend any example or ask to convert these into a single Cargo workspace.
