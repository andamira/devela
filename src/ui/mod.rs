// devela::ui
//
#![doc = crate::_DOC_UI!()]
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui: back, front, layout);
}

// IMPROVE: feature-gate some
pub mod event; // Event[Button[State]|Key[State]|Kind|Mouse|Pointer[Type]|TimeStamp|Wheel], Key*
pub mod front;

#[cfg(ui··)]
crate::items! {
    mod error;
    pub mod back; // UiService*, UiCap*
}
#[cfg(feature = "layout")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "layout")))]
pub mod layout;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[cfg(ui··)]
        pub use super::error::*;
    }
    _pub_mods {
        pub use super::{event::_all::*, front::_all::*};

        #[cfg(ui··)]
        pub use super::back::_all::*;

        #[cfg(feature = "layout")]
        pub use super::layout::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_UI_MODULES;
    }
}
