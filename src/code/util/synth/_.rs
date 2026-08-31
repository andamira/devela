// devela/src/code/util/synth/mod.rs
//
#![doc = crate::_DOC_CODE_UTIL_SYNTH!()] // public
#![doc = crate::_doc!(modules: crate::code::util; synth)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//!
//! Synthesis operates on Rust structure: generated items, implementations,
//! methods, derives, and repeated expansions. Lower-level manipulation of macro
//! input material belongs to [`token`][super::token].
//

// BOOTSTRAP: defines `mods_out!`.
mod structural; // mods_out!, structural_mods!

mods_in! {
    mod r#const; // CONST!
    mod_ derive; // macro_apply_alias!, macro_derive_alias!
    mod impl_trait; // impl_trait!
    mod items; // items!
    mod maybe; // maybe!, maybe_slot!
    mod methods; // methods_as_fns!
    #[cfg(feature = "_unroll")]
    mod unroll; // unroll!
    mod use_as; // use_as!
}
structural::mods_out! { // _mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            r#const::CONST,
            derive::{macro_apply_alias, macro_derive_alias},
            impl_trait::impl_trait,
            items::items,
            maybe::{maybe, maybe_slot},
            methods::methods_as_fns,
            structural::{structural_mods, mods_out},
            use_as::use_as,
        };
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
    }
    _reexports {
        #[doc = crate::_tags!(code procedural_macro)]
        pub use super::derive::{
            macro_apply, macro_derive, macro_derive_with,
        };
        #[doc = crate::_tags!(code procedural_macro)]
        pub use devela_macros::{
            field_of,
            mods_in,
            repeat,
        };
    }
    _hidden {
        pub use super::derive::_hidden::*;
    }
}
