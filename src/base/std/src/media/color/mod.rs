// devela_base_std::media::color
//
#![doc = crate::_DOC_MEDIA_COLOR!()] // public
#![doc = crate::_doc!(modules: crate::media; color)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(base_safe_color, forbid(unsafe_code))]

mod gamma; // Gamma

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            gamma::*,
        };
    }
}
