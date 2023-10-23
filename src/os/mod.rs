// devela::os
//
//! OS-specific, extends `std::`[`os`][std::os].
//

mod print;

/* public modules */

#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;

#[cfg(feature = "term")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "term")))]
pub mod term;

/* re-exports */

#[doc(inline)]
pub use print::*;

#[doc(no_inline)]
#[cfg(feature = "linux")]
pub use linux::all::*;

#[doc(no_inline)]
#[cfg(feature = "term")]
pub use term::all::*;

pub(crate) mod all {
    pub use super::print::*;

    #[cfg(feature = "linux")]
    pub use super::linux::all::*;

    #[cfg(feature = "term")]
    pub use super::term::all::*;
}
