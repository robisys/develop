//! Show and Edit Cargo's Manifest Files

#![cfg_attr(test, allow(dead_code))]
#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#[macro_use]
extern crate quick_error;
extern crate toml;

mod manifest;
mod dependency;
mod target;

pub use dependency::Dependency;
pub use manifest::Manifest;
pub use target::Target;
