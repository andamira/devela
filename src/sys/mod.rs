// devela::sys
//
#![doc = crate::_DOC_SYS!()]
#![doc = crate::_doc!(modules: crate; sys: arch, display, env, fs, hw, io, log, mem, net, os)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice, simd)]
//
// safety
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]

pub mod arch; // Arch, *asm, detect_*, m128* m256*
pub mod display; // {x11}
pub mod env; // App*, Arg*, Env
pub mod fs; // Fs, FsPath, PathExt
pub mod hw; // {audio, …}
pub mod io; // Io*
pub mod log;
pub mod mem; // Mem,
pub mod net; // Ip*, Socket*, Tcp*, Udp*
pub mod os; // Linux,

// WIPZONE
// #[cfg(feature = "std")]
// mod bench;

crate::structural_mods! { // _pub_mods, _hidden
    _pub_mods {
        pub use super::{
            arch::_all::*,
            display::_all::*,
            env::_all::*,
            fs::_all::*,
            hw::_all::*,
            io::_all::*,
            log::_all::*,
            mem::_all::*,
            net::_all::*,
            os::_all::*,
        };
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::bench::_all::*;
    }
    _hidden {
        pub use super::mem::_hidden::*;
    }
}
