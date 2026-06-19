// devela/src/ui/text/mod.rs
//
#![doc = crate::_DOC_UI_TEXT!()] // public
#![doc = crate::_doc!(modules: crate::ui; text)]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]

mod input; // TextInput[Action|Config|Outcome|Reject|View]

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            input::_all::*,
        };
    }
}
