// devela::string
//
//! Owned strings, extends [`std::string`].
//

/* always compiled for internal use */

/* only compiled with the `ops` feature */

#[cfg(feature = "string")]
mod macros; // private

#[cfg(feature = "string")]
mod array_string;
#[cfg(feature = "string")]
mod egc;
#[cfg(feature = "string")]
mod error;
#[cfg(feature = "string")]
mod ext;
#[cfg(feature = "string")]
mod non_nul;

/* re-exports */

#[cfg(feature = "string")]
pub use all::*;
#[cfg(feature = "string")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{array_string::*, egc::all::*, error::*, ext::*, non_nul::*};
}
