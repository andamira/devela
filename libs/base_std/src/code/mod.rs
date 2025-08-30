// devela_base_std::code
//
#![doc = crate::_DOC_CODE!()]
//

pub mod error;
pub mod panic;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            error::_all::*,
            panic::_all::*,
        };
    }
}
