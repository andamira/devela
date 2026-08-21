// devela/src/code/util/synth/mod.rs
//
//! Code synthesis and macro composition.
//

// mod _reexport_core;

mod r#const; // CONST!
mod derive; // macro_apply_alias!, macro_derive_alias!
mod impl_trait; // impl_trait!
mod methods; // methods_as_fns!
#[cfg(feature = "_unroll")]
mod unroll; // unroll! TODO

crate::structural_mods! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            r#const::CONST,
            derive::{macro_apply_alias, macro_derive_alias},
            impl_trait::impl_trait,
            methods::methods_as_fns,
        };
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
    }
    _reexports {
        // pub use super::_reexport_core::*;
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
