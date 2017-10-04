# develop   


  (under construction)   
[![Build Status](https://secure.travis-ci.org/robisys/develop.svg?branch=master)](https://travis-ci.org/robisys/develop)

[Travis-ci](https://en.wikipedia.org/wiki/Travis_CI)

[Dev-doc](https://github.com/robisys/develop/blob/master/Dev-doc.md)

[travis-ci  gihub](https://github.com/travis-ci/travis-ci)

http://blog.tgrrtt.com/exploring-the-travisci-configuration-file

## Liesmich
[Liesmich](Liesmich.md)

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

[Book](https://doc.rust-lang.org/book/)

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

[API]   (https://en.wikipedia.org/wiki/Application_programming_interface)
[Windows_API] (https://en.wikipedia.org/wiki/Windows_API)

[ABI]  (https://en.wikipedia.org/wiki/Application_binary_interface)  
Windows (GNU ABI ) (.msi) 	 Windows (MSVC ABI ) (.msi)    
There are two prominent ABIs in use on Windows: the native (MSVC) ABI used by Visual Studio, and the GNU ABI used by the GCC toolchain. Which version of Rust you need depends largely on what C/C++ libraries you want to interoperate with: for interop with software produced by Visual Studio use the MSVC build of Rust; for interop with GNU software built using the MinGW/MSYS2 toolchain use the GNU build.

MSVC builds of Rust additionally require an installation of Visual Studio 2013 (or later) so rustc can use its linker. Make sure to check the "C++ tools" option. No additional software installation is necessary for basic use of the GNU build.

Rust's support for the GNU ABI is more mature, and is recommended for typical uses. 



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


