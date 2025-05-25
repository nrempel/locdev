# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2024-01-15

### Fixed

- **BREAKING**: Fixed hostname collision detection bug that caused false positives
  - Previously adding `host` would fail if `localhost` existed
  - Now only exact hostname matches prevent additions
- **BREAKING**: Fixed remove function being too aggressive
  - Previously removing `host` would also remove `localhost` and `myhost`
  - Now only exact IP+hostname combinations are removed
- Fixed file formatting issues that could create double newlines
- Fixed inconsistent async usage throughout codebase

### Changed

- Renamed project from `locdev` to `hostie`
- Updated to Rust edition 2024
- Removed unnecessary tokio dependency (now fully synchronous)
- Improved code quality with clippy fixes
- Modernized GitHub Actions workflows
- Added comprehensive integration tests (24 total tests)
- Improved cross-platform support (Windows hosts file path)
- Enhanced clap CLI configuration following modern best practices

### Added

- Cross-platform hosts file path detection (Windows/Unix)
- Environment variable support for testing (`HOSTIE_HOSTS_FILE`)
- Comprehensive test suite covering all edge cases and bug scenarios
- Better error handling and user feedback
- Whitespace handling for various hosts file formats

## [0.1.2] - 2024-01-XX

### Added

- Initial release as `locdev`
- Add entries to hosts file
- Remove entries from hosts file  
- List current entries in hosts file
- Protection for system entries (localhost, broadcasthost)
