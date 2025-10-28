// devela_base_core::data::list::of

// mod all; // Allof
mod one; // Oneof

crate::structural_mods! { // _mods, _workspace_internals
    _mods {
        pub use super::{
            // all::Allof,
            one::Oneof,
        };
    }
    _workspace_internals {
        pub use super::one::impl_oneof;
    }
}
