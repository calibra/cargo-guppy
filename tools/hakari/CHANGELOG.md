# Changelog

## [0.2.0] - 2021-02-23

### Changed

- `hakari` now uses [`camino`](https://crates.io/crates/camino) `Utf8Path` and `Utf8PathBuf` wrappers. These wrappers
  provide type-level assertions that returned paths are valid UTF-8.
- Public dependency version bumps:
  - `proptest` updated to version 1 and the corresponding feature renamed to `proptest1`.

## [0.1.1] - 2021-02-04

### Added

* Experimental Windows support. There may still be bugs around path normalization: please [report them](https://github.com/facebookincubator/cargo-guppy/issues/new).

### Fixed

* Fixed Cargo.toml output for path dependencies.
* Return an error for non-Unicode paths instead of silently producing incorrect paths.

## [0.1.0] - 2021-02-03

Initial release.

[0.2.0]: https://github.com/facebookincubator/cargo-guppy/releases/tag/hakari-0.2.0
[0.1.1]: https://github.com/facebookincubator/cargo-guppy/releases/tag/hakari-0.1.1
[0.1.0]: https://github.com/facebookincubator/cargo-guppy/releases/tag/hakari-0.1.0
