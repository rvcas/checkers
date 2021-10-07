# Checkers

Validates Checker's Moves

## How to use?

This was built in Rust using the structopt and anyhow crates.

### Running

First you'll need rust installed, the best way to do that is to use [rustup](https://rustup.rs)

- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Once you have Rust installed you can use `cargo` to run and build the project.

- `cargo run -- --help`
- `cargo run -- inputs/red.txt`

There are a few example input files that can be found in the `inputs/` folder.

The CLI also supports:

- passing many input files at once
  - `cargo run -- inputs/red.txt inputs/illegal_move.txt`
- printing the current player, move, and board
  - `cargo run -- inputs/white.txt --debug`

### Tests

Each test uses a file from the `inputs/` folder and they can be run using `cargo`.

- `cargo test`

You should see four passing tests.
