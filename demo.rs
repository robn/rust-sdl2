extern crate sdl2;

use std::cast;

#[main]
pub fn main() {
    let ver = sdl2::ll::Struct_SDL_version { major: 0, minor: 0, patch: 0 };
    unsafe {
        sdl2::ll::SDL_GetVersion(cast::transmute(&ver));
    }
    print!("SDL {:u}.{:u}.{:u}\n", ver.major, ver.minor, ver.patch);
}
