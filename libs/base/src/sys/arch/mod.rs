// devela_base::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()]
//

mod reexports;
mod wasm;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{reexports::*, wasm::*, };
    }
}
