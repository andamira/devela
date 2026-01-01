// devela_base_std::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()]
//

mod errors;

pub mod source;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::errors::*;
    }
    _pub_mods {
        pub use super::source::*;
    }
}
