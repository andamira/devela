// devela::sys::os::term::grid
//
#![doc = crate::_DOC_SYS_OS_TERM_GRID!()] // public
#![doc = crate::_doc!(modules: crate::sys::os::term; grid)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

mod color; // TermColor[s|Kind|Mode]
mod error; // TermGridError
mod grid; // TermGrid
mod meta; // TermelMeta, TermOccupancy
mod style; // TermStyle[Ext]
mod termel; // Termel

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            color::*,
            error::*,
            grid::_all::*,
            meta::*,
            style::*,
            termel::*,
        };
    }
}
