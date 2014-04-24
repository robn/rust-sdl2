#![crate_id="sdl2#0.0.1"]
#![comment = "SDL2 bindings"]
#![license = "MIT"]
#![crate_type = "lib"]

#![feature(globs)]

extern crate libc;

pub use sdl2::*;

pub mod sdl2;
