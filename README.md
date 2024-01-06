# 💥 crashie — a little failure in a box

Crashie is a Command-Line Utility that exits with a random exit code after a configurable delay. Use it when you
want to test restart behaviors or anything that requires an application to fail.

```plain
Sleeping for 12.72 seconds, then exiting with code 130
Exiting with code 130
```

## Usage Example

If you would like to randomly fail with a SIGINT (code `130`) or SIGKILL (`137`) after 10 ± 2 seconds, run:

```bash
crashie --sigint --sigkill --delay=10 --delay-stddev=2
crashie --signals=2,3 --delay=10 --delay-stddev=2
echo $?
```

Alternatively, provide options using environment variables:

```bash
CRASHIE_SIGNALS=2,3 CRASHIE_SLEEP_DELAY=10 CRASHIE_SLEEP_DELAY_STDDEV=2 crashie
```

Crashie provides TCP and UDP echo functionalities. This comes in handy if you wand to test resilient connection
logic, port forwarding (notably Kubernetes' `kubectl port-forward`) or similar aspects.

To bind crashie to TCP sockets, use the `CRASHIE_BIND_TCP_ECHO` environment variable or run e.g.

```bash
crashie --bind-tcp-echo 127.0.0.1:8080
```

Likewise, UDP echo is supported. For that, use the `CRASHIE_BIND_UDP_ECHO` environment variable or run e.g.

```bash
crashie --bind-udp-echo 127.0.0.1:8080
```

On Linux, you can test the echo behavior e.g. using netcat (`nc 127.0.0.1 8080` for TCP or `nc -u 127.0.0.1 8080` for UDP).

To simplify work with HTTP connections, you can also bind an HTTP "echo". For that, use the `CRASHIE_BIND_HTTP_ECHO`
environment variable or run e.g.

```bash
crashie --bind-http-echo 127.0.0.1:8080
```

You can test the connection e.g. with curl (`curl -v localhost:8080`). As of now, the server always ignores the request
specifics and responds with `204 No Content`.

To support cases where responses must be `200 OK` exactly - e.g. for liveness probes in ingress checks - you
can provide the `CRASHIE_HTTP_LIVENESS_PROBE_PATH` or `--http-liveness-probe-path` argument:

```bash
crashie --bind-http-echo 127.0.0.1:8080 --http-liveness-probe-path /.health/livez
```

In this situation, calls to `curl -v localhost:8080` result in a `204 No Content`:

```
* processing: localhost:8080
*   Trying [::1]:8080...
* Connected to localhost (::1) port 8080
> GET / HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.2.1
> Accept: */*
>
< HTTP/1.1 204 No Content
< Server: crashie/0.3.0
< Date: Sat, 06 Jan 2024 14:44:53 GMT
< Content-Length: 0
< Cache-Control: no-cache, no-store
<
* Connection #0 to host localhost left intact
```

... while `curl -v localhost:8080/.health/livez` results in a `200 OK`:

```
* processing: localhost:8080/.health/livez
*   Trying [::1]:8080...
* Connected to localhost (127.0.0.1) port 8080
> GET /.health/livez HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/8.2.1
> Accept: */*
>
< HTTP/1.1 200 OK
< Server: crashie/0.3.0
< Date: Sat, 06 Jan 2024 14:44:59 GMT
< Content-Length: 0
< Cache-Control: no-cache, no-store
<
* Connection #0 to host localhost left intact
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
