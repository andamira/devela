// devela::sys::device
//
#![doc = crate::_DOC_SYS_DEVICE!()] // public
#![doc = crate::_doc!(modules: crate::sys; device: audio, display)] // gpu, midi
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_SYS_DEVICE!()]
//!
//

pub mod audio; // {alsa}
pub mod display; // {x11}
// pub mod midi; //

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            audio::_all::*,
            display::_all::*,
            // midi::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            audio::_crate_internals::*,
            display::_crate_internals::*,
        };
    }
}
