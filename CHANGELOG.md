# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Error forwarding for SPI

### Added
- SPI `FullDuplex` trait implementation for reverse compatibility.
- Serial non-blocking `Write` trait implementation for reverse compatibility.
- Unit tests.


## [0.10.0] - 2023-06-25

### Changed
- Adapted to `embedded-hal` version `1.0.0-alpha.10`


[Unreleased]: https://github.com/ryankurte/embedded-hal-compat/compare/v0.10.0...HEAD
[0.10.0]: https://github.com/ryankurte/embedded-hal-compat/releases/tag/v0.10.0
