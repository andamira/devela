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

/* contains always compiled items */

// ...

#[cfg(feature = "io")]
mod path;

/* re-exports */

#[cfg(feature = "io")]
mod reexports;

// re-exports private sub-modules
#[cfg(feature = "io")]
#[allow(unused_imports)]
pub use {path::all::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "io")]
    #[allow(unused_imports)]
    pub use super::{path::all::*, reexports::*};
}
