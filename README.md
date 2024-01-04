# crashie — a little failure in a box

Crashie is a Command-Line Utility that exits with a random exit code after a configurable delay. Use when you
want to test restart behaviors or anything that requires an application to fail.

## Usage

### Running via Docker

The application is available as the [sunside/crashie](https://hub.docker.com/r/sunside/crashie) Docker image.
To run crashie via Docker, use e.g.

```shell
docker run --rm sunside/crashie
```

### Local Installation

To install crashie from [crates.io](https://crates.io/crates/crashie), run

```shell
cargo install crashie
```

### Usage Example

If you want to randomly fail with a SIGINT (code `130`) or SIGKILL (`137`) after 10 ± 2 seconds, run:

```
crashie --sigint --sigkill --delay=10 --delay-stddev=2
```

## Run from source

To get a documentation, run

```shell
cargo run -- --help
```
