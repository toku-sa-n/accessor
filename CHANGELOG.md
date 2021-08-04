# Changelog

## Unreleased - ReleaseDate
### Added
- These structs are added ([#28](https://github.com/toku-sa-n/accessor/pull/28)).
  - `single::Generic`
  - `array::Generic`

- These type aliases are added ([#28](https://github.com/toku-sa-n/accessor/pull/28)).
  - `single::ReadWrite`
  - `single::ReadOnly`
  - `single::WriteOnly`
  - `array::ReadWrite`
  - `array::ReadOnly`
  - `array::WriteOnly`

### Changed
- License and Contribution sections are added to README ([#27](https://github.com/toku-sa-n/accessor/pull/27)).
- `single::Single` and `array::Array` are now deprecated in favor of `single::ReadWrite` and `array::ReadWrite` respectively ([#28](https://github.com/toku-sa-n/accessor/pull/28)).

### Fixed
- A wrong lint name which is enabled on CI was fixed ([#29](https://github.com/toku-sa-n/accessor/pull/29)).
- Clippy's lint errors are fixed ([#29](https://github.com/toku-sa-n/accessor/pull/29)).

## 0.3.1 - 2021-08-03
### Changed
- Bors is no longer used for merging PRs ([#20](https://github.com/toku-sa-n/accessor/pull/20)).
- Tests on CI are executed on stable Rust, not the nightly one ([#21](https://github.com/toku-sa-n/accessor/pull/21)).
- Multiple lints that are allowed by default are now denied ([#22](https://github.com/toku-sa-n/accessor/pull/22)).
- `rustfmt.toml` is deleted so that `cargo fmt` works on stable Rust ([#23](https://github.com/toku-sa-n/accessor/pull/23)).
- Methods that are not ended with "volatile" like `Single::read` are now deprecated. Use methods ending with "volatile" like `Single::read_volatile` ([#24](https://github.com/toku-sa-n/accessor/pull/24)).

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
