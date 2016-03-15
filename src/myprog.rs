//  myprog.rs
#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]
// 
//! this file: myprog.rs
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

//
//
use std::env;
//use rpdoc;

#[cfg(unix)]
fn is_executable() {
   println!("is unix");
   }
#[cfg(windows)]
fn is_executable() {
   println!("is windows");
   }   
/// # pargs
/// hier diese Funktion   
pub fn pargs() {
//! # pargs inhalt
//!  weitere Info
    for (key,value) in env::vars_os() {
    println!("{:?}: {:?}", key, value);
    }
}

fn main() {
//! main
pargs();
is_executable();
println!("Hallo World");
}


/// # rpdoc1 start
///   erklaerung
//
pub fn  rpdoc1() {
//! # rpdoc1 
//! innerhalb

println!("Hallo World rpdoc ");

}  






pub fn zatest() {
//! # zatest
//! 
//!  it reall_works
//!
//! # Example
//!
//!```
//! // #[macro_use]
//! //extern crate assert_cli;
//! 
//! //assert_cli!("g ",&["list", "--dev","--"] => Success,r#"    "#).unwrap();
//!
//! println! ("eine Bemerkung");
//! 
//! //parg() ;
//! fn Ztest()  { 
//!
//! }
//!
//! //rpdoc1 ();
//!
//! Ztest();
//!```
//!
 println! ("eine Bemerkung zatest");


}

#[test]
fn its_really_works() {
  zatest();
}
