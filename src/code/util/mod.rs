// devela/src/code/util/mod.rs
//
#![doc = crate::_DOC_CODE_UTIL!()] // public
#![doc = crate::_doc!(modules: crate::code; util: assert, cfg, debug, synth, token)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//
// # Implementation notes
// Several macros are defined hidden, suffixed with `·`, an publicly re-exported unsuffixed.
// This makes them able to be imported from the root. E.g.: capture_last!, enumset!, set!…
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

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example; // EnumintI8Example

pub mod assert; // Assertion utilities
pub mod cfg; // Conditional compilation and configuration
pub mod debug; // Debugging and diagnostic helpers
pub mod synth; // Code synthesis and macro composition
pub mod token; // Macro token, fragment, and identifier utilities

mod doclink; // doclink!
mod enumset; // enumset!
mod is; // is!
mod items; // items!, sf!
mod lets; // lets!
mod maybe; // maybe!, maybe_slot!
mod structural; // structural_mods! TODO
mod type_count; // type_count!
mod use_as; // use_as!
mod whilst; // whilst!

structural::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            doclink::doclink,
            enumset::_all::*,
            is::is,
            items::{items, sf},
            lets::lets,
            maybe::{maybe, maybe_slot},
            structural::structural_mods,
            type_count::type_count,
            use_as::use_as,
            whilst::whilst,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::EnumintI8Example;
    }
    _pub_mods {
        pub use super::{
            assert::_all::*,
            cfg::_all::*,
            debug::_all::*,
            synth::_all::*,
            token::_all::*,
        };
    }
    _reexports {
        #[doc = crate::_tags!(construction code niche procedural_macro)]
        pub use devela_macros::enumint;
        #[doc(inline)]
        pub use super::{
            assert::const_assert,
            debug::cdbg,
            token::paste,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            doclink::_DOCLINK_CUSTOM_DOMAIN,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use {
            super::assert::_hidden::*,
            devela_macros::__macro_derive_helpers,
        };
    }
}
