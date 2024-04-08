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

mod array;
mod bit;
mod error;
mod sort;
#[allow(unused_imports)]
pub use {array::*, bit::*, error::*, sort::*};

pub mod collections;
pub mod hash;
pub mod iter;
#[doc(no_inline)]
pub use {collections::*, hash::*, iter::*};

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_dyn")))]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
pub mod dst;
#[doc(no_inline)]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
pub use dst::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        array::*, bit::all::*, collections::all::*, error::*, hash::all::*, iter::all::*, sort::*,
    };

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
    pub use super::dst::*;
}
