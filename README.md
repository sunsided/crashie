# crashie — a little failure in a box

Crashie is a Command-Line Utility that exits with a random exit code after a configurable delay. Use when you
want to test restart behaviors or anything that requires an application to fail.

## Installation

To install crashie from [crates.io](https://crates.io/crates/crashie), run

```shell
cargo install crashie
```

## Example

If you want to randomly fail with a SIGINT (code 130) or SIGKILL (137) after 10 ± 2 seconds, run:

```
crashie --sigint --sigkill --delay=10 --delay-stddev=2
```

## Run from source

To get a documentation, run

```shell
cargo run -- --help
```
