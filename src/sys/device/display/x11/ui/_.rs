// devela/src/sys/device/display/x11/ui/_.rs
//
//!
//

crate::mods_in! {
    mod surface; // XSurfaceUi
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            surface::*,
        };
    }
}
