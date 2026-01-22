// devela_base_core::data::list::of

mod one; // Oneof
// mod _wip_all; // Allof
// mod _wip_macro_one; // oneof!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            one::Oneof,
            // _wip_all::*,
            // _wip_macro_one::oneof,
        };
    }
}
