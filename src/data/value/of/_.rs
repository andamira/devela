// devela/src/data/value/of/_.rs

crate::mods_in! {
    mod one; // Oneof
    // mod _wip_all; // Allof
    // mod _wip_macro_one; // oneof!
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            one::Oneof,
            // _wip_all::*,
            // _wip_macro_one::oneof,
        };
    }
}
