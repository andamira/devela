//
//! WASM architecture functionality.
//

crate::mods_in! {
    mod namespace; // Wasm
}
crate::mods_out! { // _mods, reexports
    _mods {
        pub use super::namespace::Wasm;
    }
    _reexports {
        pub use crate::boot::rust::sys::arch_wasm::*;
    }
}
