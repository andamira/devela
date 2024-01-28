// devela::io
//
//! I/O functionality, extends
//! `std::{`[`env`], [`fs`], [`io`], [`net`], [`path`]`}`.
//!
//! [`env`]: mod@std::env
//! [`fs`]: std::fs
//! [`io`]: std::io
//! [`net`]: std::net
//! [`path`]: std::path
//

#![allow(unused_imports)]

/* modules */

// feature-gated, non-public
#[cfg(feature = "io")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
mod path;
//
#[cfg(feature = "std")]
mod reexport_std;
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "error", any(feature = "no_std", feature = "std"))))
)]
#[cfg(feature = "no_std")]
mod reimplement_no_std;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "io")]
pub use path::all::*;
//
#[cfg(feature = "std")]
pub use reexport_std::*;
#[cfg(feature = "no_std")]
pub use reimplement_no_std::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use super::path::all::*;
    //
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::reexport_std::*;
    #[doc(inline)]
    #[cfg(feature = "no_std")]
    pub use super::reimplement_no_std::*;
}
