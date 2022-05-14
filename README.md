# Rust Workspace

These are my solitions to [Project Euler's Problems](https://projecteuler.net/archives).

To build all the necesary crates, libraries and problems run:
```console
cargo build --release
```

To run a specific problem's executable run:
```console
cargo run -p e### --release
```
or if already built:
```console
./target/release/e###
```
replacing ### with the problem's id

To run unit tests of utility functions run:
```console
cargo test -p utils
```
