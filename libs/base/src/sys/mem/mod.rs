// devela_base::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//

mod borrow;
mod cswap; // cswap!
mod pin;
mod ptr;
mod reexports;
pub mod cell;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            borrow::_all::*,
            cswap::*,
            pin::_all::*,
            ptr::_all::*,
            reexports::*,
        };
    }
    _pub_mods {
        pub use super::{
            cell::_all::*,
        };
    }
}
