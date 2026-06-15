// devela/src/ui/mod.rs
//
#![doc = crate::_DOC_UI!()] // public, root
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_QUO_UI!()]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui: event, frame, layout, route); // intent, view, widget
}

// mod error;
mod intent; // WIP
// #[cfg(feature = "layout")]
mod sem; // WIP
mod view; // WIP
mod widget; // WIP

#[cfg(feature = "event")]
pub mod event; // Event[Button[State]|Key[State]|Kind|Mouse|Pointer[Type]|TimeStamp|Wheel], Key*
pub mod frame; // UiFrame, UiId, UiKey, UiPhase, UiScope
pub mod layout; // Layout1d, LayoutReceipt, Lunit, Layout<Pos*|Ext*|Rec|Region|Stride*|>
pub mod route; // HitRoute, Route<Active|Capture|Focus|Hot>

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // error::*,
            intent::*,
            sem::*,
            view::*,
            widget::*,
        };
    }
    _pub_mods {
        pub use super::{
            frame::*,
            layout::*,
            route::*,
        };
        #[cfg(feature = "event")]
        pub use super::event::_all::*;
        // #[cfg(feature = "layout")]
        // pub use super::layout::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_UI_MODULES;
    }
}
