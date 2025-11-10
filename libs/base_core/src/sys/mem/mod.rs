// devela_base_core::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//

mod align; // CacheAlign, MemAligned
mod arena; // ArenaBytes
mod borrow;
mod cswap; // cswap!
mod namespace; // Mem
mod pin;
mod ptr;
mod reexports;
mod slice; // Slice

pub mod cell;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            align::_all::*,
            arena::_all::*,
            borrow::_all::*,
            cswap::*,
            namespace::*,
            pin::_all::*,
            ptr::_all::*,
            reexports::*,
            slice::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            cell::_all::*,
        };
    }
}
