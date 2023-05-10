# Perfect Connect Four

A perfect connect four solver in Rust.

Inspired by this tutorial for writing a perfect and strong solver for connect four: <http://blog.gamesolver.org/solving-connect-four/01-introduction/>

This crate provides an efficient bitboard implementation of Connect Four and a `score` function which tells you how many turns the current player needs to **win**, **loose** or **draw** from this position.
