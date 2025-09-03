// devela_base::text::str
//
#![doc = crate::_DOC_TEXT_STR!()]
//

mod ascii; // Ascii
mod namespace; // Str
mod reexports;
mod string_u; // StringU8, StringU16, StringU32, StringUsize

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ascii::*,
            namespace::Str,
            reexports::*,
            string_u::*,
        };
    }
}
