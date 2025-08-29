// devela::sys::arch::wasm
//
//! WASM architecture functionality.
//

crate::mod_path!(_c "../../../../libs/base/src/sys/arch/wasm/reexports.rs");

mod namespace; // Wasm

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{namespace::*, _c::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
