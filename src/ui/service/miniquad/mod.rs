// devela::ui::service::miniquad
//
//! `miniquad` Ui service.
//

mod reexports;
mod service; // MiniquadEventHandlerExt, MiniquadService
mod window; // MiniquadWindow

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{reexports::*, service::*, window::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
