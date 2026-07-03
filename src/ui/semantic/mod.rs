// devela/src/ui/sem/mod.rs
//
#![doc = crate::_DOC_UI_SEMANTIC!()] // private
#![doc = crate::_doc!(modules: crate::ui; semantic)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod action; // UiAction, UiActions
mod entry; // UiEntry
mod flags; // UiFlags
mod role; // UiRole
mod text; // UiText

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            action::*,
            entry::*,
            flags::*,
            role::*,
            text::*,
        };
    }
}
