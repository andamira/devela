// devela/src/sys/device/display/x11/ui/mod.rs
//
//!
//

mod surface; // XSurfaceUi

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            surface::*,
        };
    }
}
