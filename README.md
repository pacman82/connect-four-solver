# Perfect Connect Four

A perfect connect four solver in Rust.

Inspired by this tutorial for writing a perfect and strong solver for connect four: <http://blog.gamesolver.org/solving-connect-four/01-introduction/>

This crate provides an efficient bitboard implementation of Connect Four and a `score` function which tells you how many turns the current player needs to **win**, **loose** or **draw** from this position.

## Building CLI from source

This is both a crate which can be imported into Rust project and an executable which can be invoked from the command line.

In order to build and execute the executable you need the rust toolchain installed. You can do so by navigating to <http://rustup.rs> and following the instructions.

Check out the source from this project and run

```shell
cargo build --release
```

to build it. You will find the executable in the `/target/release` directory.

You can also build and execute in one go using

```shell
cargo run --release
```
