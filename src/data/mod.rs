// devela::data
//
//! Data handling and manipulation, <small>extends
//! `std::{`[`array`], [`collections`], [`hash`], [`iter`], [`vec`]`}`.
//! </small>
//!
//! [`array`]: mod@std::array
//! [`collections`]: std::collections
//! [`hash`]: std::hash
//! [`iter`]: std::iter
//! [`vec`]: mod@std::vec
//

// safety:
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

/* always compiled */

mod array;
mod error;
mod sort;
#[allow(unused_imports)]
pub use {array::*, error::*, sort::*};

pub mod hash;
pub mod iter;
#[doc(no_inline)]
pub use {hash::*, iter::*};

/* feature-gated */

#[cfg(feature = "data_bit")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data_bit")))]
mod bit;
#[cfg(feature = "data_bit")]
pub use bit::*;

#[cfg(feature = "data_collections")]
pub mod collections;
#[cfg(feature = "data_collections")]
pub use collections::*;

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_dyn")))]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
pub mod dst;
#[doc(no_inline)]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
pub use dst::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{array::*, error::*, hash::all::*, iter::all::*, sort::*};

    // feature-gated
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "data_bit")]
    pub use super::bit::all::*;
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "data_collections")]
    pub use super::collections::all::*;
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
    pub use super::dst::*;
}
