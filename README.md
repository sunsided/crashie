# 💥 crashie — a little failure in a box

Crashie is a Command-Line Utility that exits with a random exit code after a configurable delay. Use it when you
want to test restart behaviors or anything that requires an application to fail.

```plain
Sleeping for 12.72 seconds, then exiting with code 130
Exiting with code 130
```

## Usage

### Usage Example

If you want to randomly fail with a SIGINT (code `130`) or SIGKILL (`137`) after 10 ± 2 seconds, run:

```bash
crashie --sigint --sigkill --delay=10 --delay-stddev=2
crashie --signals=2,3 --delay=10 --delay-stddev=2
echo $?
```

Alternatively, provide options using environment variables:

```bash
CRASHIE_SIGNALS=2,3 CRASHIE_SLEEP_DELAY=10 CRASHIE_SLEEP_DELAY_STDDEV=2 crashie
```

### Running via Docker

The application is available as the [sunside/crashie](https://hub.docker.com/r/sunside/crashie) Docker image.
To run crashie via Docker, use e.g.

```shell
docker run --rm sunside/crashie --help
```

Provide command-line arguments as if you were running it locally:

```bash
docker run --rm sunside/crashie --sigint --sigkill --delay=10 --delay-stddev=2
echo $?
```

Alternatively, provide configuration via environment variables:

```bash
docker run --rm \
  --env CRASHIE_SIGNALS=2,3 \
  --env CRASHIE_SLEEP_DELAY=10 \
  --env CRASHIE_SLEEP_DELAY_STDDEV=2 \
  sunside/crashie
echo $?
```

### Local Installation from crates.io

To install crashie from [crates.io](https://crates.io/crates/crashie), run

```shell
cargo install crashie
```

## Run from source

To get a documentation, run

```shell
cargo run -- --help
```
