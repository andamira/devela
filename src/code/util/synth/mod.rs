// devela/src/code/util/synth/mod.rs
//
//! Code synthesis and macro composition.
//!
//! Synthesis operates on Rust structure: generated items, implementations,
//! methods, derives, and repeated expansions. Lower-level manipulation of macro
//! input material belongs to [`token`][super::token].
//

mod r#const; // CONST!
mod derive; // macro_apply_alias!, macro_derive_alias!
mod impl_trait; // impl_trait!
mod items; // items!
mod maybe; // maybe!, maybe_slot!
mod methods; // methods_as_fns!
mod structural; // structural_mods!
#[cfg(feature = "_unroll")]
mod unroll; // unroll!
mod use_as; // use_as!

structural::structural_mods! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            r#const::CONST,
            derive::{macro_apply_alias, macro_derive_alias},
            impl_trait::impl_trait,
            items::items,
            maybe::{maybe, maybe_slot},
            methods::methods_as_fns,
            structural::structural_mods,
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
            repeat,
        };
    }
}
