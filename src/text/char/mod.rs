// devela::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
// #![doc = crate::_doc!(extends: char)]
// #![doc = crate::_doc!(modules: crate::text; char)]
// #![doc = crate::_doc!(newline)]
//

// with re-exports
crate::mod_path!(_c "../../../libs/base/src/text/char/reexports.rs");
mod definitions;

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use devela_base::text::Char;
        pub use super::{_c::*, definitions::*};
    }
    _always {
        pub use super::_c::*;
    }
}
