// devela::io
//
//! OS-specific, extends [`std::os`].
//

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(any(target_os = "linux", doc))]
    pub use super::linux::all::*;
}

#[cfg(any(target_os = "linux", doc))]
#[cfg_attr(feature = "nightly", doc(cfg(target_os = "linux")))]
pub mod linux;
