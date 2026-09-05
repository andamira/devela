// devela/src/sys/os/browser/web/ui/_.rs
//
//! UI presenters implemented for the web.
//

crate::mods_in! {
    mod canvas; // WebCanvasUi
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            canvas::WebCanvasUi,
        };
    }
}
