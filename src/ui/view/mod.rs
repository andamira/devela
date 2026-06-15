// devela/src/ui/view/mod.rs
//
#![doc = crate::_DOC_UI_VIEW!()] // public
#![doc = crate::_doc!(modules: crate::ui; view)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests;

mod form; // WIP UiViewForm: cell, document, graphic and message projection forms
mod layer; // UiLayer
mod profile; // WIP Presentation profiles for fitting logical views into output space
mod scale; // WIP Pixel, density, and text scaling units for view projection
mod view; // UiView, UiViewFlags

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            form::_all::*,
            layer::*,
            profile::_all::*,
            scale::_all::*,
            view::*,
        };
    }
}
