// devela::os
//
//! OS-specific, extends [`std::os`].
//

mod print;

#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;

#[cfg(feature = "term")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "term")))]
pub mod term;

/* re-exports */

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::print::*;

    #[cfg(feature = "linux")]
    #[doc(inline)]
    pub use super::linux::all::*;

    #[cfg(feature = "term")]
    #[doc(inline)]
    pub use super::term::*;
}
