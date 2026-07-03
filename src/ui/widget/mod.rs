// devela/src/ui/widget/mod.rs
//
#![doc = crate::_DOC_UI_WIDGET!()] // public
#![doc = crate::_doc!(modules: crate::ui; widget)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod button; // UiButton
mod response; // UiResponse, UiResponseFlags

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            button::*,
            response::*,
        };
    }
}
