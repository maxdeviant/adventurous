# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `traverse` method to `Input`
- Added `From<&str>` and `From<String>` implementations for `Input`
- Added `test_part_one_solution!` and `test_part_two_solution!` for testing solutions against the correct answer

### Changed

- `Solve::Answer` now requires `Debug` and `PartialEq` implementations in addition to `Display`

## [0.1.0] - 2023-12-02

### Added

- Added `Solve` trait for implementing puzzle solvers
- Added `run` function for solving a puzzle
- Added `raw` method to `Input` for retrieving the raw input
- Added `lines` method to `Input` for iterating over the lines in the input
- Added `manhattan_distance` for computing the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points

### Changed

- Changed the contents of `Input` to be private
- Changed `CharCounts` trait to use `usize`s instead of `i32`s

## [0.0.3] - 2018-12-02

### Added

- Added `CharCounts` trait for counting characters in a string

## [0.0.2] - 2018-12-01

### Added

- Added documentation

## [0.0.1] - 2018-12-01

### Added

- Added `Input` struct for reading puzzle input

[unreleased]: https://github.com/maxdeviant/adventurous/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/maxdeviant/adventurous/compare/v0.0.3...v0.1.0
[0.0.3]: https://github.com/maxdeviant/adventurous/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/maxdeviant/adventurous/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/maxdeviant/adventurous/compare/90b1174...v0.0.1
