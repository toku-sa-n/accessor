# Changelog

## [Unreleased]
### Fixed
- `no_run` attribute is added to the code example in `README.md` to avoid a compile error ([#38]).
- `Copy` trait bound is removed from `array::Generic` and `single::Generic ([#37]).

## [0.3.2] - 2021-08-04
### Added
- These structs are added ([#28]).
  - `single::Generic`
  - `array::Generic`
- These type aliases are added ([#28]).
  - `single::ReadWrite`
  - `single::ReadOnly`
  - `single::WriteOnly`
  - `array::ReadWrite`
  - `array::ReadOnly`
  - `array::WriteOnly`
- License and Contribution sections are added to README ([#27]).

### Changed
- An empty `dependencies` section in `Cargo.toml` is removed ([#31]).
- Changelog is improved ([#32]).
- `todo!` is used instead of `unimplemented!` in the documentations. `todo!` indicates that the code should be implemented ([#33]).
- The crate-level documentation is imported from `README.md` ([#34]).

## Deprecated
- `single::Single` in favor of `single::ReadWrite` ([#28]).
- `array::Array` in favor of `array::ReadWrite` ([#28]).

### Fixed
- A wrong lint name which is enabled on CI was fixed ([#29]).
- Clippy's lint errors are fixed ([#29]).

## [0.3.1] - 2021-08-03
### Changed
- Bors is no longer used for merging PRs ([#20]).
- Tests on CI are executed on stable Rust, not the nightly one ([#21]).
- Multiple lints that are allowed by default are now denied ([#22]).
- `rustfmt.toml` is deleted so that `cargo fmt` works on stable Rust ([#23]).

### Deprecated
- Methods that are not ended with "volatile" like `Single::read` in favor of methods ending with "volatile" like `Single::read_volatile` ([#24]).

### Fixed
- Clippy warnings are fixed ([#19]).

## [0.3.0] - 2021-01-29
### Added
- Additional crate level documentation is added.

### Changed
- `Array::new` and `Single::new` now panic when the requirements are not fulfilled.
- Previous `Array::new` and `Single::new` are renamed to `Array::try_new` and `Single::try_new` respectively.

## [0.2.0] - 2021-01-26
### Changed
- The return type of `Mapper::map` is changed from `usize` to `NonZeroUsize`.
- Safety notes of methods are edited.

## [0.1.0] - 2021-01-24
### Added
- Initial version.

[#38]: https://github.com/toku-sa-n/accessor/pull/38
[#37]: https://github.com/toku-sa-n/accessor/pull/37
[#34]: https://github.com/toku-sa-n/accessor/pull/34
[#33]: https://github.com/toku-sa-n/accessor/pull/33
[#32]: https://github.com/toku-sa-n/accessor/pull/32
[#31]: https://github.com/toku-sa-n/accessor/pull/31
[#29]: https://github.com/toku-sa-n/accessor/pull/29
[#28]: https://github.com/toku-sa-n/accessor/pull/28
[#27]: https://github.com/toku-sa-n/accessor/pull/27
[#24]: https://github.com/toku-sa-n/accessor/pull/24
[#23]: https://github.com/toku-sa-n/accessor/pull/23
[#22]: https://github.com/toku-sa-n/accessor/pull/22
[#21]: https://github.com/toku-sa-n/accessor/pull/21
[#20]: https://github.com/toku-sa-n/accessor/pull/20
[#19]: https://github.com/toku-sa-n/accessor/pull/19

[Unreleased]: https://github.com/toku-sa-n/accessor/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/toku-sa-n/accessor/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/toku-sa-n/accessor/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/toku-sa-n/accessor/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/toku-sa-n/accessor/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/toku-sa-n/accessor/releases/tag/v0.1.0
