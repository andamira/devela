// devela::sys::os::browser::web::api
//
//! The web API Javascript ←→ Rust bridge interface.
//

// impl web APIs:
#[cfg(not(feature = "safe_sys"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
crate::items! {
    #[cfg(feature = "event")]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
    mod events; // → events
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
    mod history; // → history, location
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
    mod workers; // → workers
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "unsafe_ffi", target_arch = "wasm32"))))]
    mod canvas; // → canvas
}

mod namespace; // Web, → permission, performance

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
        };
    }
}
