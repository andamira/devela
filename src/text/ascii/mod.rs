// devela::text::ascii
//
#![doc = crate::_DOC_TEXT_ASCII!()]
#![doc = crate::_doc!(extends: ascii)]
#![doc = crate::_doc!(modules: crate::text; ascii)]
#![doc = crate::_doc!(newline)]
//!
//

#[cfg(feature = "ascii")]
mod char;

crate::structural_mods! { // _mods
    _mods {
        pub use devela_base::text::str::Ascii;

        #[cfg(feature = "ascii")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "ascii")))]
        pub use super::char::AsciiChar;
    }
}
