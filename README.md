# Rust-SDL2
Bindings for SDL2 in Rust

# Overview

Rust-SDL2 is a library for talking to SDL2 from Rust. Low-level C components are wrapped in Rust code to make them more idiomatic and abstract away inappropriate manual memory management.

Rust-SDL2 uses the MIT license.

# Requirements

* *Rust* - we currently compile against the *Master* branch. The releases on http://www.rust-lang.org tend to not work.
* *SDL 2.0 development libraries* - install through your favourite package management tool, or via http://www.libsdl.org/

# Installation

Clone this repo, run `rustc src/lib.rs`.

# When things go wrong
Rust, and Rust-SDL2, are both still heavily in development, and you may run into teething issues when using this. Before panicking, check that you're using the latest Master branch of Rust, check that you've updated Rust-SDL to the latest version, and run `make clean` and `./configure`. If that fails, please let us know on the issue tracker.
