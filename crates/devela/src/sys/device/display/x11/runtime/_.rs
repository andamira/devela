//

crate::mods_in! {
    mod frontend;
    mod present;
    #[cfg(feature = "image")]
    mod raster;
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            frontend::XFrontend,
            present::XPresent,
        };
        #[cfg(feature = "image")]
        pub use super::raster::XRasterRenderer;
    }
    _crate_internals {
        pub(crate) use super::{
            frontend::XFrameCtx, // XBackend
            present::XPresenter,
        };
    }
}
