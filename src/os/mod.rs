// devela::io
//
//! OS-specific, extends [`std::os`].
//

pub(crate) mod all {
    #[doc(inline)]
    pub use super::linux::all::*;
}

#[cfg_attr(feature = "nightly", doc(cfg(target_os = "linux")))]
pub mod linux;
