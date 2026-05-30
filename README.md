# shishell

A simple Unix-style shell written in Rust, built as a learning project.

It runs a basic read-eval-print loop: it prints a prompt, reads a line, and
executes the command(s) you type by spawning child processes.

## Features

- Run external programs with arguments (e.g. `ls -la`)
- Pipe commands together with `|` (e.g. `ls | grep src | wc -l`)
- Built-in `cd` to change the working directory (defaults to `/`)
- Built-in `exit` to quit the shell

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Build & Run

```bash
cargo run
```

Or build a release binary:

```bash
cargo build --release
./target/release/shishell
```

## Usage

Once running, you'll see a `>` prompt:

```
>ls
>cat Cargo.toml | grep name
>cd src
>exit
```

## How it works

Each line is split on `|` into a pipeline of commands. For every command in the
pipeline, the shell wires the previous command's `stdout` into the next
command's `stdin`, and the final command writes to the terminal. The `cd` and
`exit` commands are handled internally rather than spawned as processes.

## Project layout

```
src/main.rs   # the entire shell: REPL, piping, and built-ins
Cargo.toml    # package manifest
```
