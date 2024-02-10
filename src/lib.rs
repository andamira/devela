// devela::lib
//
//! A cohesive development foundation.
//

// warnings:
#![warn(missing_docs, clippy::all)]
#![cfg_attr(not(feature = "deps"), allow(rustdoc::broken_intra_doc_links))]
#![allow(
    clippy::module_inception, // allow modules with the same name as its parent
    clippy::wrong_self_convention, // allow `is_` methods having an owned self
)]
// nightly:
// - WAIT: [doc_cfg](https://github.com/rust-lang/rust/issues/43781)
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg))]
// - WAIT: [coroutines](https://github.com/rust-lang/rust/issues/43122)
#![cfg_attr(
    feature = "nightly_coro",
    feature(coroutines, coroutine_trait, iter_from_coroutine)
)]
// environment:
#![cfg_attr(not(feature = "std"), no_std)]
// safety:
#![cfg_attr(feature = "safe", forbid(unsafe_code))]

// safeguard environment:
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
//
// safeguard safety:
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", // includes all below:
        feature = "unsafe_array", feature = "unsafe_async", feature = "unsafe_const",
        feature = "unsafe_dyn", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str",
    )
))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");
// but you can enable `safe_*` features to prevent `unsafe` use in specific modules.

pub mod code;
pub mod data;
pub mod gfx;
pub mod io;
pub mod mem;
pub mod num;
pub mod os;
pub mod result;
pub mod text;
pub mod time;
pub mod ui;
pub mod work;

/// Additional dependencies.
pub mod _deps;

/// Additional documentation.
pub mod _docs {
    /// Documented examples.
    #[rustfmt::skip] // rustfmt doesn't work in here
    #[cfg(any(doc, test))]
    pub mod examples;

    /// Documented features.
    pub mod features {
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./_docs/features.md")]
    }
}

/// All the items are re-exported here.
///
/// Note that any item tagged with [`dep`] can also be enabled by
/// manually enabling the associated optional dependency.
pub mod all {
    #[allow(unused_imports)]
    #[doc(inline)]
    pub use super::{
        code::all::*, data::all::*, gfx::all::*, io::all::*, mem::all::*, num::all::*, os::all::*,
        result::all::*, text::all::*, time::all::*, ui::all::*, work::all::*,
    };
}
// and from the root
#[doc(no_inline)]
pub use all::*;
