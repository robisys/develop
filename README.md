# develop      
  (under construction)   
[![Build Status](https://secure.travis-ci.org/robisys/develop.svg?branch=master)](https://travis-ci.org/robisys/develop)

# rust-lang  
* cargo   [guide](http://doc.crates.io/guide.html)   
  cargo -V    
[cargo] (https://github.com/rust-lang/cargo)
* rust    [doku]   (https://www.rust-lang.org/documentation.html)    
  rust -V    
[rust](https://github.com/rust-lang/rust)  [ Contributing](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md)  

[Dokumentation ](https://doc.rust-lang.org/book/documentation.html) 
[ rustbyexample](http://rustbyexample.com/index.html) 
and the  Code  [github.com/rust-lang/rust-by-example](https://github.com/rust-lang/rust-by-example)

* Crate
[libc] (https://doc.rust-lang.org/libc/index.html) 
[std](https://doc.rust-lang.org/std/)
[core]    (https://doc.rust-lang.org/core/index.html)

* contribute libs  
  [libs](https://www.rust-lang.org/contribute-libs.html)   

* [Clippy](https://github.com/Manishearth/rust-clippy)

* for windows
[rust#building-on-windows] (https://github.com/rust-lang/rust#building-on-windows)  
 [msys2 ] (http://msys2.github.io/)     
# Downloads  
  [Downloads](https://www.rust-lang.org/downloads.html)

#  Rust compiler   
 Build stages [Contributing to the Rust compiler](https://gregchapple.com/contributing-to-the-rust-compiler/)       
  Building the Rust compiler from source involves four stages.
  * stage0    
Because the source for the Rust compiler is itself written in Rust, it means that we can't just compile the source into the latest compiler. Instead, we need to download an older version of the compiler from the internet and use that to build a new version of the compiler from the source tree.
stage0 downloads an older version of the compiler from the internet.
  * stage1    
Once we have a version of the compiler downloaded, we start to compile a new version from source. We use the compiler from stage0 to build the stage1 compiler.
The stage1 compiler contains all new language features & optimizations, but are not used in the compiler itself. To build the compiler with these new features & optimizations, we need to build from source again using this version of the compiler.
  * stage2  
Using the compiler from stage1 which contains all new language features & optimizations we build the stage2 compiler. At this point, we have the latest, most advanced & optimized version of the compiler.
The final (and optional) build step is to re-build the compiler once more using the compiler from stage2 to produce the stage3 compiler.
  * stage3   
The stage3 compiler should be bitwise identical to the stage2 compiler. We build it to ensure that we haven't introduced any new issues in the latest build.

# Cargo  
  The Rust community’s crate host   
[Cargo crates.io](https://crates.io/)


