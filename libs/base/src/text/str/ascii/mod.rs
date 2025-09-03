// devela_base::text::ascii
//
#![doc = crate::_DOC_TEXT_ASCII!()]
#![doc = crate::_doc!(extends: ascii)]
#![doc = crate::_doc!(modules: crate::text; ascii)]
#![doc = crate::_doc!(newline)]
//!
//

mod wrapper;

crate::structural_mods! { // _mods
    _mods {
        pub use super::wrapper::Ascii;
    }
}
