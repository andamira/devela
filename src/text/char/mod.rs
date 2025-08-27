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
mod namespace;

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{_c::*, definitions::*, namespace::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_c::*;
    }
}
