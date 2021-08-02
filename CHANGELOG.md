# Changelog

## Unreleased - ReleaseDate
### Changed
- Bors is no longer used for merging PRs ([#20](https://github.com/toku-sa-n/accessor/pull/20)).
- Tests on CI are executed on stable Rust, not the nightly one ([#21](https://github.com/toku-sa-n/accessor/pull/21)).
- `rustfmt.toml` is deleted so that `cargo fmt` works on stable Rust ([#23](https://github.com/toku-sa-n/accessor/pull/23)).

### Fixed
- Clippy warnings are fixed ([#19](https://github.com/toku-sa-n/accessor/pull/19)).

## 0.3.0 - 2021-01-29
### Added
- Additional crate level documentation is added.

### Changed
- `Array::new` and `Single::new` now panic when the requirements are not fulfilled.
- Previous `Array::new` and `Single::new` are renamed to `Array::try_new` and `Single::try_new` respectively.

## 0.2.0 - 2021-01-26
### Changed
- The return type of `Mapper::map` is changed from `usize` to `NonZeroUsize`.
- Safety notes of methods are edited.

## 0.1.0 - 2021-01-24
### Added
- Initial version.
