// devela/src/sys/device/display/x11/ui/mod.rs
//
//!
//

mod surface; // XSurfaceUi

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            surface::*,
        };
    }
}
