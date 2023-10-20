// devela::slice
//
//! Slice, extends [`std::slice`].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "slice"))]
pub(crate) use always_fns::*;

/* only compiled with the `char` feature */

#[cfg(feature = "slice")]
mod ext;
#[cfg(feature = "slice")]
mod fns;

/* re-exports */

#[cfg(feature = "slice")]
pub use all::*;
#[cfg(feature = "slice")]
pub(crate) mod all {
    pub use super::{always_fns::*, ext::*, fns::*};
}
