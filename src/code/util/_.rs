// devela/src/code/util/_.rs
//
#![doc = crate::_DOC_CODE_UTIL!()] // public
#![doc = crate::_doc!(modules: crate::code; util: assert, cfg, debug, synth, token)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//!
//! This module collects small, cross-cutting facilities for authoring,
//! checking, inspecting, and transforming Rust code.
//!
//! [`assert`][mod@assert] checks assumptions, [`cfg`][mod@cfg] controls compilation,
//! and [`debug`] assists diagnostics. [`synth`] constructs and composes Rust code,
//! while [`token`] operates on the token, fragment, identifier,
//! and literal material from which macros work.
//!
//! Facilities with a clearer domain or semantic home belong there instead;
//! `util` is not a general catch-all namespace.
//
// # Implementation notes
//
// Several declarative macros are defined hidden with a `·` suffix and
// publicly re-exported without it, allowing them to remain importable
// from the crate's flattened surface. E.g. `capture_last!`, `is!`, `whilst!`.
// See: https://github.com/rust-lang/rust/pull/52234#issuecomment-976702997
//
// # Warning regarding macro expansion
//
// Some attributes (#[doc = …], #[rustfmt::skip], and other tooling-related ones) cause the
// compiler to inspect or expand tokens earlier than normal, before the crate's macro resolution
// graph is fully fixed. At that point, macros that are only introduced indirectly (for example
// via helper macros, exported macros, or re-exports) may not yet be visible, even if they would
// exist after full expansion. Because macro_rules! resolution depends on phase ordering, this
// can lead to "resolution is stuck" errors where the compiler cannot prove which macro applies.
// Defining the macro directly in the crate root avoids this, because it is visible in all phases.
//
// # Documentation for declarative macros
//
// - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/decl-macros.html)
// - [Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html)
// - [Specification](https://doc.rust-lang.org/reference/macro-ambiguity.html)

// BOOTSTRAP: needed by doc attributes before `mods_in!`/`mods_out!` expansion.
mod doclink; // doclink!
#[path = "synth/_.rs"] // BOOTSTRAP: provides `mods_in!` and `mods_out!`.
pub mod synth; // Code synthesis and macro composition

synth::mods_in! {
    #[cfg(test)]
    mod_ _test;

    pub mod_ assert; // Assertion utilities
    pub mod_ cfg; // Conditional compilation and configuration
    pub mod_ debug; // Debugging and diagnostic helpers
    pub mod_ token; // Macro token, fragment, and identifier utilities

    mod is; // is!
    mod lets; // lets!
    mod whilst; // whilst!
}
synth::mods_out! { // _mods, _reexports, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            doclink::doclink,
            is::is,
            lets::lets,
            whilst::whilst,
        };
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
        #[doc(inline)]
        pub use super::{
            // assert::const_assert,
            // debug::cdbg,
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
            super::{
                assert::_hidden::*,
                synth::_hidden::*,
            },
            devela_macros::__macro_derive_helpers,
        };
    }
}
