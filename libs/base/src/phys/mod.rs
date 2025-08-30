// devela_base::phys
//
#![doc = crate::_DOC_PHYS!()]
//

mod time;

crate::structural_mods! { // _mods
    _mods {
        pub use super::time::_all::*;
    }
}
