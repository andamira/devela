//! Android host environment.
//!
//! Low-level Android OS concepts and native interfaces live here. Portable
//! application lifecycle, normalized events, and media/rendering models belong
//! in their corresponding `run`, `ui`, and `media` modules.

crate::mods_in! {
    // mod_ activity;
    // mod_ asset;
    mod define;
    // mod_ ffi;
    // mod_ input;
    // mod_ log;

}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // activity::_all::*,
            // asset::_all::*,
            define::Android,
            // ffi::_all::*,
            // input::_all::*,
            // log::_all::*,
        };
    }
}
