// devela/src/sys/device/display/x11/runtime/_.rs

crate::mods_in! {
    mod frontend; // (XBackend), (XFrameCtx), XFrontend
    mod present; // XPresent, (XPresenter), XRasterRender
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            frontend::*,
            present::*,
        };
    }
}
