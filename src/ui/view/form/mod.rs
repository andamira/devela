// devela/src/ui/view/form/mod.rs
//
#![doc = crate::_DOC_UI_VIEW_FORM!()] // private
#![doc = crate::_doc!(modules: crate::ui::view; form)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

// mod cell; // WIP
// mod document; // WIP
// mod graphic; // WIP
// mod message; // WIP
mod view; // UiViewForm

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // cell::_all::*,
            // document::_all::*,
            // graphic::_all::*,
            // message::_all::*,
            view::*,
        };
    }
}
