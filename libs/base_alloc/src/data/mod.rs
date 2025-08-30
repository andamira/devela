// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()]
//

// mod bit; // TODO
mod sort;

pub mod list;
pub mod key;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::sort::_all::*;
    }
    _pub_mods {
        pub use super::{list::_all::*, key::_all::*};
    }
}
