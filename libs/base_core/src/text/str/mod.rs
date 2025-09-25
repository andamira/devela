// devela_base_core::text::str
//
#![doc = crate::_DOC_TEXT_STR!()]
//

mod namespace; // Str
mod reexports;
mod string_nonul; // StringNonul
mod string_u; // StringU8, StringU16, StringU32, StringUsize

// WIPZONE
// mod _wip_sixbit;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::Str,
            reexports::*,
            string_nonul::*,
            string_u::*,
        };
        // WIPZONE
        // pub use super::_wip_sixbit::*;
    }
}
