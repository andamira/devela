// devela::text::parse
//
//! String parsing without structured semantics.
//

crate::mod_path!(_c "../../libs/base/src/text/parse/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use devela_base::text::ByteSearch;

        #[doc(inline)]
        pub use super::_c::*;
    }
}
