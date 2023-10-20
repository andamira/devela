// devela::result
//
//! Result, extends [`std::result`].
//

/* always compiled for internal use */

/* only compiled with the `result` feature */

#[cfg(feature = "result")]
mod ext;
#[cfg(feature = "result")]
mod never;

/* re-exports */

#[cfg(feature = "result")]
pub use all::*;
#[cfg(feature = "result")]
pub(crate) mod all {
    pub use super::{ext::ResultExt, never::*};
}
