// devela_base_alloc::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//

mod alloc;
mod borrow;
// mod pin;
// mod ptr;
mod reexports;
// pub mod cell;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            alloc::_all::*,
            borrow::_all::*,
            // pin::_all::*,
            // ptr::_all::*,
            reexports::*,
        };
    }
    _pub_mods {
        // pub use super::{
        //     cell::_all::*,
        // };
    }
}
