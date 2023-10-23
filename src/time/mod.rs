// devela::time
//
//! Temporal quantification, extends [`std::time`].
//

/* always compiled for internal use */

/* only compiled with the `time` feature */

/* re-exports */

#[cfg(feature = "time")]
mod reexports;

#[cfg(feature = "time")]
pub use all::*;
#[cfg(feature = "time")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;
}
