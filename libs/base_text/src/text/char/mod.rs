// devela_base_text::text::char
//
#![doc = crate::_DOC_TEXT_EGC!()]
//

mod translit; // scalar_as_ascii_translit()

crate::structural_mods! { // _mods, _workspace_internals
    _mods {
        pub use super::{
            translit::_all::*,
        };
    }
    _workspace_internals {
        pub use super::{
            translit::_workspace_internals::*,
        };
    }
}
