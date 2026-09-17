//
//! Defines the [`Arch`] namespace.
//

crate::mods_in! {
    mod_ impls;
    mod define;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Arch,
        };
    }
}
