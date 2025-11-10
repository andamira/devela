// devela_base_core::data::list::of

mod one; // Oneof
// mod _wip_all; // Allof

crate::structural_mods! { // _mods, _workspace_internals
    _mods {
        pub use super::{
            one::Oneof,
            // _wip_all::*,
        };
    }
    _workspace_internals {
        pub use super::one::impl_oneof;
    }
}
