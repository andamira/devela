// devela/src/ui/view/form/_.rs
//
#![doc = crate::_DOC_UI_VIEW_FORM!()] // private
#![doc = crate::_doc!(modules: crate::ui::view; form)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod cell; // WIP
    // mod document; // WIP
    // mod graphic; // WIP
    // mod message; // WIP
    mod view; // UiViewForm
}
crate::mods_out! { // _mods
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
