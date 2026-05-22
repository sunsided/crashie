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

Crashie provides TCP and UDP echo functionalities. This comes in handy if you want to test resilient connection
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

The default status code for non-liveness paths is `204 No Content`. To make crashie return a
different status (e.g. for testing retry and circuit-breaker logic), use `CRASHIE_HTTP_STATUS`
or `--http-status`:

```bash
crashie --bind-http-echo 127.0.0.1:8080 --http-status 503
```

The liveness probe path continues to return `200 OK` regardless of the configured status,
so probes still succeed while other paths fail.

### Running on Kubernetes

A common use case for crashie is exercising a cluster's reaction to flaky pods — restart
policies, probe behavior, `kubectl port-forward` resilience, retry logic in upstream
services. The Docker image already exposes ports `80` (HTTP echo), `30000` (TCP echo) and
`40000` (UDP echo) and a liveness probe path; pair it with a `restartPolicy` of `Always`
and watch what happens:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: crashie
spec:
  replicas: 1
  selector:
    matchLabels:
      app: crashie
  template:
    metadata:
      labels:
        app: crashie
    spec:
      containers:
        - name: crashie
          image: sunside/crashie:latest
          env:
            # Crash after 30 ± 10 seconds with SIGINT or SIGTERM.
            - name: CRASHIE_SLEEP_DELAY
              value: "30"
            - name: CRASHIE_SLEEP_DELAY_STDDEV
              value: "10"
            - name: CRASHIE_SIGNALS
              value: "2,15"
            - name: CRASHIE_HTTP_LIVENESS_PROBE_PATH
              value: "/health/live"
          ports:
            - name: http
              containerPort: 80
            - name: tcp-echo
              containerPort: 30000
            - name: udp-echo
              containerPort: 40000
              protocol: UDP
          livenessProbe:
            httpGet:
              path: /health/live
              port: http
            periodSeconds: 5
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

### Prebuilt Binaries

Each tagged release publishes prebuilt binaries for Linux (x86_64, aarch64), macOS
(x86_64, aarch64) and Windows (x86_64) on the
[GitHub Releases](https://github.com/sunsided/crashie/releases) page. Download the
archive for your platform, extract, and run `crashie`.

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

## All Command-Line Options

Output of `crashie --help` (built with default features):

```plain
A Command-Line Utility that exits with a random exit code after a configurable delay

Usage: crashie [OPTIONS]

Options:
  -h, --help     Print help
  -V, --version  Print version

Delay (crash after):
  -d, --delay <SECONDS>               The sleep duration before exiting, in seconds [env: CRASHIE_SLEEP_DELAY=] [default: 9.0]
      --delay-stddev <SECONDS>        The standard deviation of the sleep duration, in seconds [env: CRASHIE_SLEEP_DELAY_STDDEV=] [default: 2.0]
      --delay-grace-period <SECONDS>  The duration, in seconds, to wait before starting the actual delay [env: CRASHIE_SLEEP_DELAY_GRACE_PERIOD=] [default: 1.0]

Echo Server:
      --bind-tcp-echo <SOCK_ADDR>  Provide TCP echo on the specified addresses [env: CRASHIE_BIND_TCP_ECHO=]
      --bind-udp-echo <SOCK_ADDR>  Provide UDP echo on the specified addresses [env: CRASHIE_BIND_UDP_ECHO=]

Echo Server (HTTP):
      --bind-http-echo <SOCK_ADDR>            Provide HTTP echo on the specified addresses [env: CRASHIE_BIND_HTTP_ECHO=]
      --http-liveness-probe-path <HTTP_PATH>  The request path on which to serve liveness probe results [env: CRASHIE_HTTP_LIVENESS_PROBE_PATH=] [default: /health/live]
      --http-status <STATUS>                  Default HTTP status code returned for non-liveness paths [env: CRASHIE_HTTP_STATUS=] [default: 204]

Exit Codes:
  -e, --exit-code <EXIT_CODES>  Exit with the specified code(s) [env: CRASHIE_EXIT_CODES=]
  -s, --signals <NUMBER>        Arbitrary signal (exit code 128+SIGNAL) [env: CRASHIE_SIGNALS=]

Exit Codes (POSIX):
      --sighup   Hang up controlling terminal or terminal [env: CRASHIE_SIGHUP=]
      --sigint   Interrupt from keyboard, Control-C [env: CRASHIE_SIGINT=]
      --sigquit  Quit from keyboard, Control-\ [env: CRASHIE_SIGQUIT=]
      --sigill   Illegal instruction [env: CRASHIE_SIGILL=]
      --sigabrt  Abnormal termination [env: CRASHIE_SIGABRT=]
      --sigfpe   Floating-point exception [env: CRASHIE_SIGFPE=]
      --sigkill  Forced process termination [env: CRASHIE_SIGKILL=]
      --sigusr1  Freely available to processes [env: CRASHIE_SIGUSR1=]
      --sigsegv  Invalid memory reference (Segmentation Fault) [env: CRASHIE_SIGSEGV=]
      --sigusr2  Freely available to processes [env: CRASHIE_SIGUSR2=]
      --sigpipe  Write to pipe with no readers [env: CRASHIE_SIGPIPE=]
      --sigalrm  Real-time clock [env: CRASHIE_SIGALRM=]
      --sigterm  Process termination [env: CRASHIE_SIGTERM=]

Exit Codes (non-POSIX):
      --sigtrap    Breakpoint for debugging [env: CRASHIE_SIGTRAP=]
      --sigiot     Equivalent to SIGABRT [env: CRASHIE_SIGIOT=]
      --sigbus     Bus error [env: CRASHIE_SIGBUS=]
      --sigstkflt  Coprocessor stack error [env: CRASHIE_SIGSTKFLT=]
      --sigchld    Child process stopped, terminated or got a signal if traced [env: CRASHIE_SIGCHLD=]
      --sigxcpu    CPU time limit exceeded [env: CRASHIE_SIGXCPU=]
      --sigxfsz    File size limit exceeded [env: CRASHIE_SIGXFSZ=]
      --sigvtalrm  Virtual timer clock [env: CRASHIE_SIGVTALRM=]
      --sigprof    Profile timer clock [env: CRASHIE_SIGPROF=]
      --sigio      I/O now possible [env: CRASHIE_SIGIO=]
      --sigpoll    Equivalent to SIGIO [env: CRASHIE_SIGPOLL=]
      --sigpwr     Power supply failure [env: CRASHIE_SIGPWR=]
      --sigsys     Bad system call [env: CRASHIE_SIGSYS=]
      --sigunused  Equivalent to SIGSYS [env: CRASHIE_SIGUNUSED=]
```
