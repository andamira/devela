// devela::sys::os::term::grid
//
#![doc = crate::_DOC_SYS_OS_TERM_GRID!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::term; grid)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests;

mod color; // TermColor[s|Kind|Mode]
mod definition; // TermGrid
mod error; // TermGridError
mod meta; // TermelMeta, TermOccupancy
mod style; // TermStyle[Ext]
mod termel; // Termel

mod impls; // impls for TermGrid

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            color::*,
            definition::*,
            error::*,
            meta::*,
            style::*,
            termel::*,
        };
    }
}
