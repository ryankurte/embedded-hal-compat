# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.11.1] - 2023-07-14

### Fixed
- Fixed implementation of `InputPin` requiring `OutputPin`. For input/output pins, the type must now be annotated (see documentation).

## [0.11.0] - 2023-07-04

### Changed
- Adapted to `embedded-hal` version `1.0.0-alpha.11`

## [0.10.1] - 2023-07-01

### Fixed
- Error forwarding for SPI

### Added
- SPI `FullDuplex` trait implementation for reverse compatibility.
- Serial non-blocking `Write` trait implementation for reverse compatibility.
- I2C and SPI `Transactional` trait implementations for reverse compatibility (needs `alloc`).
- Unit tests.

### Removed
- `mock` module, which contained only mock implementations for the documentation.


## [0.10.0] - 2023-06-25

### Changed
- Adapted to `embedded-hal` version `1.0.0-alpha.10`


[Unreleased]: https://github.com/ryankurte/embedded-hal-compat/compare/v0.11.1...HEAD
[0.11.1]: https://github.com/ryankurte/embedded-hal-compat/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/ryankurte/embedded-hal-compat/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/ryankurte/embedded-hal-compat/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/ryankurte/embedded-hal-compat/releases/tag/v0.10.0
