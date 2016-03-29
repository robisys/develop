 
#![crate_name="rp_sys"]
#![crate_type = "lib"]

extern crate sdl2;
extern crate libc;
extern crate sdl2_sys as sys;

#[macro_use]
extern crate bitflags;

use libc::{c_int, c_char};
use std::ffi::CString;
use std::path::Path;
use sdl2::surface::Surface;
use sdl2::render::Texture;
use sdl2::render::Renderer;
use sdl2::rwops::RWops;
use sdl2::version::Version;
use sdl2::get_error;
use sdl2::SdlResult;


// Setup linking for all targets.
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_image")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_image")]
    extern {}
}

#[cfg(any(target_os="windows", target_os="linux", target_os="freebsd"))]
mod others {
    #[link(name="SDL2_image")]
    extern {}
}

#[allow(non_camel_case_types, dead_code)]
mod ffi;

/// InitFlags are passed to init() to control which subsystem
/// functionality to load.

pub fn quit() {
    //! Teardown the SDL2_Image subsystem
  //  unsafe { ffi::IMG_Quit(); }
}

pub fn get_linked_version() -> Version {
    //! Returns the version of the dynamically linked SDL_image library
   /* unsafe {
        Version::from_ll(*ffi::IMG_Linked_Version())
    }
    */
}
