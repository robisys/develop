//  wist main.rs
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]
// 
//! this file: lib.rs
//!
//! Show and Edit Cargo's Manifest Files
//!
//!
//! # Example 1
//!
//!```rust
//!  fn mydoc () {};
//!
//!  mydoc();
//!
//!
//!```
//! # Example 2
//!
//!```
//! fn main() {
//!
//!
//!
//!
//!
//!    }
//!```
//!
//! `cargo add`

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate docopt;
extern crate rustc_serialize;
extern crate pad;
extern crate toml;
#[macro_use]
extern crate quick_error;

use std::error::Error;
use std::process;
use std::io::{self, Write};

extern crate cargo_edit;
use cargo_edit::Manifest;

mod wist;
mod wist_error;
mod tree;

use wist::list_section;
use wist_error::ListError;
use tree::parse_lock_file as list_tree;

static USAGE: &'static str = r"
my cargo wist
Usage:
    cargo wist [--dev|--build] [options]
    cargo wist --tree
    cargo wist (-h|--help)
    cargo wist --version

Options:
    --manifest-path=<path>  Path to the manifest to list dependencies of.
    --tree                  List dependencies recursively as tree.
    -h --help               Show this help page.
    --version               Show version.

Display a crate's dependencies using its Cargo.toml file.
";

/// Docopts input args.
#[derive(Debug, RustcDecodable)]
struct Args {
    /// dev-dependency
    flag_dev: bool,
    /// build-dependency
    flag_build: bool,
    /// Render tree of dependencies
    flag_tree: bool,
    /// `Cargo.toml` path
    flag_manifest_path: Option<String>,
    /// `--version`
    flag_version: bool,
}

impl Args {
    /// Get dependency section
    fn get_section(&self) -> &'static str {
        if self.flag_dev {
            "dev-dependencies"
        } else if self.flag_build {
            "build-dependencies"
        } else {
            "dependencies"
        }
    }
}

fn handle_list(args: &Args) -> Result<String, Box<Error>> {
    if args.flag_tree {
        let manifest = try!(Manifest::open_lock_file(&args.flag_manifest_path
                                                          .as_ref()
                                                          .map(|s| &s[..])));
        list_tree(&manifest)
    } else {
        let manifest = try!(Manifest::open(&args.flag_manifest_path.as_ref().map(|s| &s[..])));
        list_section(&manifest, args.get_section())
            .or_else(|err| match err {
                ListError::SectionMissing(..) => Ok("".into()),
                _ => Err(err),
            })
    }
    .map_err(From::from)
}

fn main() {
    let args = docopt::Docopt::new(USAGE)
                   .and_then(|d| d.decode::<Args>())
                   .unwrap_or_else(|err| err.exit());

    if args.flag_version {
        println!("cargo-list version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    match handle_list(&args) {
        Ok(list) => {
            println!("{}", list);
        }
        Err(err) => {
            write!(io::stderr(), "{}\n", err).unwrap();
            process::exit(1);
        }
    }
}
