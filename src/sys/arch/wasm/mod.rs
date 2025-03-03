// devela::sys::arch::wasm
//
//! WASM architecture functionality.
//

mod namespace; // Wasm

#[cfg(target_arch = "wasm32")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
mod reexports; // ::core::arch::wasm32::*

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::namespace::*;
        #[cfg(target_arch = "wasm32")]
        pub use super::reexports::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
