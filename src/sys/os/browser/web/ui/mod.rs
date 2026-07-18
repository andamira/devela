// devela/src/sys/os/browser/web/ui/mod.rs
//
//! UI presenters implemented for the web.
//

mod canvas; // WebCanvasUi

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            canvas::*,
        };
    }
}
