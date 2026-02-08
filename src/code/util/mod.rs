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

mod _env; // __dbg!, __std!, _std_core!
mod _reexport_core; // SYMLINK to /crates/base/core/src/code/util/_reexport.rs

mod cdbg; // cdbg!

#[cfg(feature = "_unroll")]
mod unroll; // unroll!

devela_base_core::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            cdbg::*,
        };
        #[cfg(feature = "_unroll")]
        pub use super::unroll::_all::*;
    }
    _reexports {
        // includes devela_code_macros:
        // cif, compile, compile_attr, compile_doc,
        // ident_total, ident_total_unique, ident_unique,
        // coalesce, field_of,
        // repeat,
        pub use super::_reexport_core::*;

        #[doc(inline)]
        // NOTE: in sync with /src/base/core/src/code/util/mod.rs
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
            cold_path, likely, unlikely,
            whilst,
            write_at,
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
