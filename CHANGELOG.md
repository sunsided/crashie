# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.2.0]: https://github.com/sunsided/crashie/releases/tag/0.2.0
[0.1.0]: https://github.com/sunsided/crashie/releases/tag/0.1.0
