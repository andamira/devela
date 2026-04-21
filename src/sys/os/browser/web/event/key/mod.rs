// devela::sys::os::browser::web::event::key
//
//!
//

// impls
mod ffi;
mod media;
mod pad;

mod location; // WebKeyLocation

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            location::*,
        };
    }
}
