// devela::sys::arch::wasm
//
//! WASM architecture functionality.
//

crate::mod_path!(_c "../../../../libs/base/src/sys/arch/wasm/reexports.rs");

mod namespace; // Wasm

crate::structural_mods! { // _mods
    _mods {
        pub use super::{namespace::*, _c::*};
    }
}
