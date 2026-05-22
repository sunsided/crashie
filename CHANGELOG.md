# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- README now includes a Kubernetes Deployment example demonstrating the intended
  use case (restart-policy and probe testing).
- GitHub Actions release workflow that publishes prebuilt binaries for Linux
  (x86_64, aarch64), macOS (x86_64, aarch64) and Windows (x86_64) on tag push.

### Changed

- Upgraded `rand` from `0.8.5` to `0.9.3` and `rand_distr` from `0.4.3` to `0.5`.

### Fixed

- Minor README and source-comment typos.

## [0.4.0] - 2024-01-06

### Added

- Added `CRASHIE_SLEEP_DELAY_GRACE_PERIOD` / `--delay-grace-period` option to provide a minimum value of seconds
  to wait before crashing.

## [0.3.0] - 2024-01-06

### Added

- Added echo support for TCP connections via the `CRASHIE_BIND_TCP_ECHO` / `--bind-tcp-echo` option.
- Added echo support for UDP datagrams via the `CRASHIE_BIND_UDP_ECHO` / `--bind-udp-echo` option.
- Added "echo" support for HTTP requests via the `CRASHIE_BIND_HTTP_ECHO` / `--bind-http-echo` option. The server
  will always respond with a `204 No Content`.
- Added the `CRASHIE_HTTP_LIVENESS_PROBE_PATH` / `--http-liveness-probe-path` options to allow returning of
  `200 OK` on a specific path.
- Command-line options are now shown in named sections when using the `-h` / `--help` flag.

## [0.2.0] - 2024-01-07

### Added

- Added environment variable support.
- README, CHANGELOG and LICENSE are now bundled with the Docker image.

### Changed

- When no exit code is provided as an argument, one is randomly selected.
- A default standard deviation of two seconds is now used.
- The default Docker behavior is now the same as with the CLI. To show the help, pass `--help`.

## [0.1.0] - 2024-01-04

### Added

- Added support for the `-e`/`-exit-code`, `-s`/`--signal` and specialized `--sigint`, ... options.
- Added support for `--delay` and `--delay-std`.

### Internal

- 🎉 Initial release.

[Unreleased]: https://github.com/sunsided/crashie/compare/0.4.0...HEAD
[0.4.0]: https://github.com/sunsided/crashie/releases/tag/0.4.0
[0.3.0]: https://github.com/sunsided/crashie/releases/tag/0.3.0
[0.2.0]: https://github.com/sunsided/crashie/releases/tag/0.2.0
[0.1.0]: https://github.com/sunsided/crashie/releases/tag/0.1.0
