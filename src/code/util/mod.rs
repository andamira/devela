// devela::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()] // public
#![doc = crate::_doc!(modules: crate::code; util)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: hint)]
//
// # Implementation notes
// Several macros are defined hidden, prefixed with `_`, an publicly re-exported unprefixed.
// This makes them able to be imported from the root. E.g.: bitfield, capture_last, enumset…
// See: https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997
//
// # Warning regarding macro expansion
// Some attributes (#[doc = …], #[rustfmt::skip], and other tooling-related ones) cause the
// compiler to inspect or expand tokens earlier than normal, before the crate's macro resolution
// graph is fully fixed. At that point, macros that are only introduced indirectly (for example
// via helper macros, exported macros, or re-exports) may not yet be visible, even if they would
// exist after full expansion. Because macro_rules! resolution depends on phase ordering, this
// can lead to "resolution is stuck" errors where the compiler cannot prove which macro applies.
// Defining the macro directly in the crate root avoids this, because it is visible in all phases.
//
// # Documentation for declarative macros
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

mod _reexport_core;

#[cfg(test)]
mod tests;

mod asserts; // (assertion macros)
mod capture; // capture_[first|last|tail_tuple]!
mod cdbg; // cdbg!
mod cfg_if; // cfg_if!
mod r#const; // CONST!
mod debug; // , compile_warning!, fn_name!
mod deprecate; // deprecate_feature!
mod doclink; // doclink!
mod enumset; // enumset!
mod ident; // ident_const_index!
mod impl_trait; // impl_trait!
mod include; // include_from!, mod_from!, mod_path!
mod is; // is!
mod items; // items!, sf!
mod lets; // lets!
mod maybe; // maybe!
mod methods; // methods_as_fns!
mod paste; // paste!, (wrapped for docs)
mod structural; // structural_mods!
mod type_count; // type_count!
mod unlikely; // cold_path, likely, unlikely
mod whilst; // whilst!
mod write; // write_at!

#[cfg(feature = "_unroll")]
mod unroll; // unroll!

structural::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            asserts::{assert_eq_all, assert_approx_eq_all, const_assert},
            capture::{capture_first, capture_last, capture_tail_tuple},
            cdbg::*,
            cfg_if::cfg_if,
            r#const::CONST,
            debug::{compile_warn, fn_name},
            deprecate::deprecate_feature,
            doclink::doclink,
            enumset::enumset,
            ident::ident_const_index,
            impl_trait::impl_trait,
            include::{include_from, mod_from, mod_path},
            is::is,
            items::{items, sf},
            lets::lets,
            maybe::maybe,
            methods::methods_as_fns,
            paste::paste,
            structural::structural_mods,
            type_count::type_count,
            unlikely::{cold_path, likely, unlikely},
            whilst::whilst,
            write::write_at,
        };
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;

        #[doc = crate::_tags!(code procedural_macro)]
        #[rustfmt::skip]
        pub use devela_base_macros::{
            cif, compile, compile_attr,
            ident_total, ident_total_unique, ident_unique,
            coalesce, field_of,
            compile_doc,
            repeat,
        };
        #[doc(inline)]
        #[doc = crate::_tags!(code procedural_macro)]
        #[cfg(feature = "devela_macros")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "devela_macros")))]
        pub use devela_macros::{
            enumint,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            doclink::_DOCLINK_CUSTOM_DOMAIN,
        };
    }
    _hidden {
        pub use super::{
            paste::__paste,
        };
    }
}
