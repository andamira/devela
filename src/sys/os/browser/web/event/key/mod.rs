// devela::sys::os::browser::web::event::key
//
//!
//

mod ffi; // impl:KeyFfi
mod media; // impl:KeyMedia
mod pad; // impl:KeyPad

mod location; // WebKeyLocation, impl:KeyMod,KeyMods

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            location::*,
        };
    }
}
