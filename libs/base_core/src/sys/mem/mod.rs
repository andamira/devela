// devela_base_core::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//

mod _reexport; // SYMLINK from /src/sys/mem/_reexport_core.rs

mod align; // CacheAlign, MemAligned
mod arena; // ArenaBytes
mod borrow;
mod byte; // MaybeByte
mod cswap; // cswap!
mod namespace; // Mem
mod pin;
mod ptr;
mod size; // size_of_expr!, BitSized, ByteSized,
mod slice; // Slice

pub mod cell;

crate::structural_mods! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        pub use super::{
            align::_all::*,
            arena::_all::*,
            borrow::_all::*,
            byte::*,
            cswap::*,
            namespace::*,
            pin::_all::*,
            ptr::_all::*,
            size::_all::*,
            slice::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            cell::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
    _hidden {
        pub use super::{
            arena::_hidden::*,
            size::_hidden::*,
        };
    }
}
