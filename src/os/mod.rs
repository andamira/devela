// devela::os
//
//! OS-specific, extends [`std::os`].
//

/* always compiled for internal use */

/* only compiled with the `os` feature */

// use a feature instead of OS detection, so it can be used from `no_std`.
#[cfg(all(feature = "linux", feature = "os"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;

#[cfg(feature = "os")]
mod print;
#[cfg(feature = "os")]
pub mod term;

/* re-exports */

#[cfg(feature = "os")]
pub use all::*;
#[cfg(feature = "os")]
pub(crate) mod all {
    #[cfg(feature = "linux")]
    #[doc(inline)]
    pub use super::linux::all::*;

    #[doc(inline)]
    pub use super::{print::*, term::*};
}
