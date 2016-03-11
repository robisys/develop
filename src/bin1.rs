// RP 20160131 Test  bin1.rs
//
/// # k(l)eine Rust Dokumentation
/// 
/// RobiSys


pub fn rpdoc() {
//!
//! # Rust Grundlagen
//!
//! hier nun erste Information
//!
//! *  ['rust-doc'](https://www.rust-lang.org)    'The Rust Programming Language'
//!
//!    weiter
//!
//! *  ['Crates'](https://crates.io/)   'CARGO  Browse all Crates'
//!    weiter
//!
//! *  ['DOC'](https://doc.rust-lang.org/) 'Doc'
//!
//!    weiter
//!
//!
//! # Weitere Quellen
//!   
//! hier nun erste Information
//!
//! *  ['Rustbook'](http://rust-lang-de.github.io/rustbook-de/) 'Rustbook de'
//!    weiter
//!     
//! *  ['Rust nightly'](http://doc.rust-lang.org/nightly ) 'Rust Doku/'
//!  
//!    
//!    
//! # 3 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter
//!    [TOML](https://github.com/alexcrichton/toml-rs)
//!
//!    [Hyperium-mime ](https://github.com/hyperium/mime.rs)
//!
//!    [markdown-editor](https://github.com/jbt/markdown-editor)
//!
//!     [Markdown-Editor](https://jbt.github.io/markdown-editor)
//!
//!   [opensuse Cargo](http://software.opensuse.org/package/cargo)
//!
//!
//!  # 4 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter7865
//!
//!
//! # 5 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter
//!
//!
//! # 6 Unterpunkt
//! hier nun erste Information
//!
//! *  ['Mod1'](modules/) 'modules/'
//!    weiter
//!
//!
//! # 7 Tools

//!  
//!    [ VUE Markdown-Editor ](http://vuejs.org/guide/installation.html)
//!
//!
//!
// rpdoc ende
// }

//    #![doc(html_favicon_url = "https://www.rust-lang.org/favicon.ico") ]
//    #![crate_name = "stdd"]

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/nightly/",
       html_playground_url = "https://play.rust-lang.org/",
       issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut))))]
 
// rpdoc ende
}


 /*
 
#[cfg(unix)]
fn is_executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::prelude::*;
    metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
}
#[cfg(windows)]
fn is_executable(metadata: &fs::Metadata) -> bool {
    metadata.is_file()
}
  */

// #[doc(no_inline)] 
//pub use std;
//pub use std::env;
pub use std::option::Option;

    /// Person Struc
    ///
    ///hier dann weiteres
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}
    ///  Struc Rodel
    ///
    ///  weitere Info
pub struct Rodel {
    /// A Rodel must have a name, no matter how much Juliet may hate it
    ///
    /// weiter info für Rodel
    name1: String,
}


impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Example
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

impl Rodel {
    /// Returns a Rodel with the name1 given them
    ///
    /// # Arguments
    ///
    /// * `name1` - A Teststring slice that holds the name of the person
    ///
    /// # Example
    ///
    /// ```
    /// // You Test can have rust code between fences inside the comments
    /// // If you pass --test to Rustdoc, it will even test it for you!
    /// let Rodel = Rodel::new("name1");
    /// ```
    pub fn new(name1: &str) -> Rodel {
        Rodel {
            name1: name1.to_string(),
        }
    }

    /// Gives a Testfriendly hello!  TestTestTestTest
    ///
    /// Says "Hello, [name1]" to the `Rodel` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name1);
    }
}
    /// Argumente
    pub fn argumente () {
    
    //let args: Vec<String> = env::args().collect();
    
    //println!("My path is {}.", args[0] );
    println!("I got ");
         
    }

/// foote 
///
/// Function 
///  running 
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
 pub fn foote() {
 //  Inhalt der Foote
 //!  Doku
 //! # kochbuch
 //!
 println!("Foote");
 
 
 }
///
//




/// fn tante
///
///  main Prog
/// Returns a  the  given text -bla bla 
///
/// # Arguments
///
/// * `nam` - A Teststring slice that holds the name of the person
///
///  erklährung äüö ß ÜÖÄ
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = tante();
/// ```

pub fn tante() {
// tante

}      
///



include!("lib1.rs");

//include!("unit_test.rs");

// include ("webbrowser.rs") ;

/*
#[cfg(not(test))]
fn main () {
   println!("If you see this, the tests were not compiled nor ran!");
}
*/

#[cfg(test)]
mod test {
     // help
     fn dd() {
     println!("function  dd ");

     }
     
   #[test]
   fn d() {
   println!("d -- I got test d ");
    
   }

   #[test]
//   #[should_panic]
    fn gg() {
    println!("gg -- I got gg ");
    

    dd();
    
        

    }
}


/*
//  rp 20160130
// rustc --test main.rs


/// Constructs a new `Rc<T>`.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
pub fn new(value: T) -> Rc<T> {
    // implementation goes here
    //! Documentation innerhalb
    //! # Examples
    //!```
    //!
    //!```
    //
}


*/

pub fn docbsp1() {
// RP 20160210 Test
//
//! # diese k(l)eine Rust Dokumentation
//! 
//! RobiSys

//!
//! # Rust Grundlagen
//!
//! hier nun erste Information
//!
//! *  ['rust-doc'](https://www.rust-lang.org)    'The Rust Programming Language'
//!
//!    weiter
//!
//! *  ['Crates'](https://crates.io/)   'CARGO  Browse all Crates'
//!    weiter
//!
//! *  ['DOC'](https://doc.rust-lang.org/) 'Doc'
//
}


 /*

```
// This is a testable code block
```

```rust{.example}
// This is rust and also testable
```

```ignore
// This is not a testable code block
```
   */
    // This is a testable code block (4-space indent)

///```sh
/// # this is shell code and not tested
///```
   // */
/*

#[doc = "
Calculates the factorial of a number.

Given the input integer `n`, this function will calculate `n!` and return it.
"]

fn g() {
}



// Rustdoc will inline documentation of a `pub use` into this crate when the
// `pub use` reaches across crates, but this behavior can also be disabled.
#[doc(no_inline)]  
pub use std::option::Option;

/// A whizbang. Does stuff. (this line is the summary)
///
/// Whizbangs are ...
struct Whizbang;

 */
//

fn main() {
    // console.log("start");
    //! main

    let john = Person::new("John");

    john.hello();
    
    let jonas = Rodel::new("Johann");

    jonas.hello();
    
    //  console.log("stop");
    argumente();
    
}


//include!("webbrowser.rs");


/// # Documenting modules
///
/// Rust has another kind of doc comment, 
/// 
///  //!. This comment doesn't document the next item, but the enclosing item.
/// 
///  In other words:
///
/// mod foo {
/// 
///    //! This is documentation for the `foo` module.
/// 
///    //!
/// 
///    //! # Examples
///
/// 
///    // ...
///    
/// 
/// ...
/// }
/// 
/// This is where you'll see 
/// 
/// //! used most often: for module documentation. 
/// 
/// If you have a module in foo.rs, you'll often open its code and see this:
/// 
/// 
/// //! A module for using `foo`s.
/// 
/// //!
/// 
/// //! The `foo` module contains a lot of useful functionality blah blah blah
/// 
/// //
/// 
/// 


/// A lightweight logging facade.

///
/// A logging facade provides a single logging API that abstracts over the
/// actual logging implementation. Libraries can use the logging API provided
/// by this crate, and the consumer of those libraries can choose the logging
/// framework that is most suitable for its use case.
///
/// If no logging implementation is selected, the facade falls back to a "noop"
/// implementation that ignores all log messages. The overhead in this case
/// is very small - just an integer load, comparison and jump.
///
/// A log request consists of a target, a level, and a body. A target is a
/// string which defaults to the module path of the location of the log
/// request, though that default may be overridden. Logger implementations
/// typically use the target to filter requests based on some user
/// configuration.
///
/// # Use
///
/// ## In libraries
///
/// Libraries should link only to the `log` crate, and use the provided
/// macros to log whatever information will be useful to downstream consumers.
///
/// ### Examples
///
/// ```rust
/// # #![allow(unstable)]
/// #[macro_use]
/// extern crate log;
///
/// # #[derive(Debug)] pub struct Yak(String);
/// # impl Yak { fn shave(&self, _: u32) {} }
/// # fn find_a_razor() -> Result<u32, u32> { Ok(1) }
/// pub fn shave_the_yak(yak: &Yak) {
///     info!(target: "yak_events", "Commencing yak shaving for {:?}", yak);
///
///     loop {
///         match find_a_razor() {
///             Ok(razor) => {
///                 info!("Razor located: {}", razor);
///                 yak.shave(razor);
///                 break;
///             }
///             Err(err) => {
///                 warn!("Unable to locate a razor: {}, retrying", err);
///             }
///         }
///     }
/// }
/// # fn main() {}
///
/// ```
///
/// ## In executables
///
/// Executables should choose a logging framework and initialize it early in the
/// runtime of the program. Logging frameworks will typically include a
/// function to do this. Any log messages generated before the framework is
/// initialized will be ignored.
///
/// The executable itself may use the `log` crate to log as well.
///
/// ### Warning
///
/// The logging system may only be initialized once.
///
/// ### Examples
///
/// ```rust,ignore
/// #[macro_use]
/// extern crate log;
/// extern crate my_logger;
///
/// fn main() {
///     my_logger::init();
///
///     info!("starting up");
///
///     // ...
/// }
/// ```
///
/// # Logger implementations
///
/// Loggers implement the `Log` trait. Here's a very basic example that simply
/// logs all messages at the `Error`, `Warn` or `Info` levels to stdout:
///
/// ```rust
/// extern crate log;
///
/// use log::{LogRecord, LogLevel, LogMetadata};
///
/// struct SimpleLogger;
///
/// impl log::Log for SimpleLogger {
///     fn enabled(&self, metadata: &LogMetadata) -> bool {
///         metadata.level() <= LogLevel::Info
///     }
///
///     fn log(&self, record: &LogRecord) {
///         if self.enabled(record.metadata()) {
///             println!("{} - {}", record.level(), record.args());
///         }
///     }
/// }
///
/// # fn main() {}
/// ```
///
/// Loggers are installed by calling the `set_logger` function. It takes a
/// closure which is provided a `MaxLogLevel` token and returns a `Log` trait
/// object. The `MaxLogLevel` token controls the global maximum log level. The
/// logging facade uses this as an optimization to improve performance of log
/// messages at levels that are disabled. In the case of our example logger,
/// we'll want to set the maximum log level to `Info`, since we ignore any
/// `Debug` or `Trace` level log messages. A logging framework should provide a
/// function that wraps a call to `set_logger`, handling initialization of the
/// logger:
///
/// ```rust
/// # extern crate log;
/// # use log::{LogLevel, LogLevelFilter, SetLoggerError, LogMetadata};
/// # struct SimpleLogger;
/// # impl log::Log for SimpleLogger {
/// #   fn enabled(&self, _: &LogMetadata) -> bool { false }
/// #   fn log(&self, _: &log::LogRecord) {}
/// # }
/// # fn main() {}
/// # #[cfg(feature = "use_std")]
/// pub fn init() -> Result<(), SetLoggerError> {
///     log::set_logger(|max_log_level| {
///         max_log_level.set(LogLevelFilter::Info);
///         Box::new(SimpleLogger)
///     })
/// }
/// ```
///
/// # Use with `no_std`
///
/// To use the `log` crate without depending on `libstd`, you need to specify
/// `default-features = false` when specifying the dependency in `Cargo.toml`.
/// This makes no difference to libraries using `log` since the logging API
/// remains the same. However executables will need to use the `set_logger_raw`
/// function to initialize a logger and the `shutdown_logger_raw` function to
/// shut down the global logger before exiting:
///
/// ```rust
/// # extern crate log;
/// # use log::{LogLevel, LogLevelFilter, SetLoggerError, ShutdownLoggerError,
/// #           LogMetadata};
/// # struct SimpleLogger;
/// # impl log::Log for SimpleLogger {
/// #   fn enabled(&self, _: &LogMetadata) -> bool { false }
/// #   fn log(&self, _: &log::LogRecord) {}
/// # }
/// # impl SimpleLogger {
/// #   fn flush(&self) {}
/// # }
/// # fn main() {}
/// pub fn init() -> Result<(), SetLoggerError> {
///     unsafe {
///         log::set_logger_raw(|max_log_level| {
///             static LOGGER: SimpleLogger = SimpleLogger;
///             max_log_level.set(LogLevelFilter::Info);
///             &SimpleLogger
///         })
///     }
/// }
/// pub fn shutdown() -> Result<(), ShutdownLoggerError> {
///     log::shutdown_logger_raw().map(|logger| {
///         let logger = unsafe { &*(logger as *const SimpleLogger) };
///         logger.flush();
///     })
/// }
/// 
/// ```
pub fn rp_logger() {}





/*

#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://doc.rust-lang.org/log/")]
// 
#![warn(missing_docs)]
#![cfg_attr(feature = "nightly", feature(panic_handler))]

#![cfg_attr(not(feature = "use_std"), no_std)]

#[cfg(not(feature = "use_std"))]
extern crate core as std;

#[cfg(feature = "use_std")]
extern crate libc;

*/

