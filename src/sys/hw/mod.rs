// devela::sys::hw
//
#![doc = crate::_DOC_SYS_HW!()]
//

mod audio; //

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            audio::_all::*,
        };
    }
}
