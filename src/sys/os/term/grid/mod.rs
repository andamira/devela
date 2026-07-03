// devela/src/sys/os/term/grid/mod.rs
//
#![doc = crate::_DOC_SYS_OS_TERM_GRID!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::term; grid)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod color; // TermColor[s|Kind|Mode]
mod define; // TermGrid
mod error; // TermGridError
mod meta; // TermelMeta, TermOccupancy
mod style; // TermStyle[Ext]
mod termel; // Termel

mod impls; // impls for TermGrid

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            color::*,
            define::*,
            error::*,
            meta::*,
            style::*,
            termel::*,
        };
    }
}
