// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::_doc!(extends: str, string)]

mod reexports;
mod macros; // join!, str!

#[cfg(feature = "str")]
mod ext_str; // ExtStr

#[cfg(all(feature = "str", feature = "alloc"))]
mod ext_string;
#[cfg(feature = "_str_nonul")] // RETHINK
#[cfg_attr(nightly_doc, doc(cfg(feature = "_str_nonul")))]
mod nonul;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::reexports::*;
        pub use super::macros::*;

        #[cfg(feature = "str")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "str")))]
        pub use super::ext_str::*;
        #[cfg(all(feature = "str", feature = "alloc"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "str")))]
        pub use super::ext_string::*;

        #[cfg(feature = "_str_nonul")] // RETHINK
        pub use super::nonul::*;
    }
    _always {
        pub use super::reexports::*;
    }
}
