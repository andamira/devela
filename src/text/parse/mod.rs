// devela::text::parse
//
//! String parsing without structured semantics.
//

mod _reexport_core; // SYMLINK to ../../libs/base_core/src/text/parse/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::text::ByteSearch;
    }
}
