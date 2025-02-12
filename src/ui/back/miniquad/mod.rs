// devela::ui::back::miniquad
//
//! [`miniquad`][crate::_dep::miniquad] UI backends.
//

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod base;
mod reexports;
mod service; // MiniquadEventHandlerExt, MiniquadService
mod window; // MiniquadWindow

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{reexports::*, service::*, window::*};
        #[cfg(feature = "alloc")]
        pub use super::base::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
