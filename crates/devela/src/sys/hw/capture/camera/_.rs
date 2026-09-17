//
//! Camera device interfaces.
//
// Low-level access to image capture hardware
// via backends such as V4L2, libcamera, and platform-specific drivers.
//

crate::mods_in! {
    // mod v4l;
}
crate::mods_out! { _mods
    _mods {
        pub use super::{
            v4l::_all::*,
        };
    }
}
