# Unused Changelog

## [0.1.0] - 2021-06-25

### Added

 - The `Unused` type, its `new` function, and implementations of `Clone`,
   `Copy`, `Debug`, `Default`, `Eq`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`,
   `Send`, and `Sync`.
 - A test for the `Debug` implementation of `Unused`.
 - Documentation and information about the crate.
 - `producer` and `multiple_unused` examples.
 - MIT and Apache 2.0 licenses.
 - The `std` feature, which provides implementations for `RefUnwindSafe` and
   `UnwindSafe` in environments where the standard library is supported.
 - The `default` feature, which enables the `std` feature.

### Changed

 - The crate description, to `Allows for unused generic parameters that do not act like they are owned.`

## [0.0.0] - 2021-06-23

### Added

 - Claimed the `unused` crate name by publishing an empty version.

