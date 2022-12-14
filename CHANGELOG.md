# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added a `port_name()` function to `kaleidoscope_focus::Focus`, which returns
  the name of the serial port it is connected to, if available.
- Updated the command-line tools to make use of it, and display the port name on
  the progress indicator.

## [0.1.0] - 2022-10-23

_Initial release._

[Unreleased]: https://github.com/keyboardio/kaleidoscope-focus.rs/commits/main
[0.1.0]: https://github.com/keyboardio/kaleidoscope-focus.rs/releases/tag/v0.1.0
