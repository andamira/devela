// devela::code::util
//
#![doc = crate::_DOC_CODE_UTIL!()]
//!
#![doc = crate::_doc!(extends: hint)]
//
// # Implementation notes
// Several macros are defined hidden, prefixed with `_`, an publicly re-exported unprefixed.
// This makes them able to be imported from the root. E.g.: bitfield, capture_last, enumset…
// See: https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997
//
// # Documentation for declarative macros
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

mod _reexport_core; // SYMLINK to /libs/base_core/src/code/util/_reexport.rs

mod _env; // __dbg!, _std_core!

mod cdbg; // cdbg!

#[cfg(feature = "_unroll")]
mod unroll; // unroll!

// WIPZONE
// #[cfg(all(feature = "std", feature = "dep_image"))]
// #[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "dep_image"))))]
// mod docima; // DocImage

devela_base_core::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            cdbg::*,
        };
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
        // WIPZONE
        // #[cfg(all(feature = "std", feature = "dep_image"))]
        // pub use super::docima::*;
    }
    _reexports {
        pub use super::_reexport_core::*;

        #[doc(inline)]
        // NOTE: in sync with /libs/base_core/src/code/util/mod.rs
        pub use devela_base_core::{ // IMPROVE path
            assert_eq_all, assert_approx_eq_all, const_assert,
            capture_first, capture_last, capture_tail_tuple,
            cfg_if,
            CONST,
            compile_warn, fn_name,
            deprecate_feature,
            doclink,
            enumset,
            ident_const_index,
            impl_trait,
            include_from, mod_from, mod_path,
            is,
            items, sf,
            lets,
            maybe,
            methods_as_fns,
            paste,
            structural_mods,
            type_count,
            whilst,
            write_at,
            // devela_code_macros:
            // cif, compile, compile_attr, compile_doc,
            // ident_total, ident_total_unique, ident_unique,
            // coalesce, field_of,
            // repeat,
        };
        #[doc(inline)]
        #[cfg(feature = "devela_macros")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "devela_macros")))]
        pub use devela_macros::{
            enumint,
        };
    }
    _crate_internals {
        pub(crate) use super::_env::*;
    }
}
