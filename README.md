# wth-m8

This repository is an example of a dependency conflict (and a fix) that can occur in Rust using Cargo.

## The case
Suppose that you are writing a Rust program (`wth-m8`) that uses a library (`sub-wth`). Both `wth-m8` and `sub-wth` depends on the library [`ndarray`](https://github.com/rust-ndarray/ndarray).
