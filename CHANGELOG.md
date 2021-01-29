# Changelog

## Unreleased - ReleaseDate
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
