// devela::io
//
//! OS-specific, extends [`std::os`].
//

pub(crate) mod all {
    #[doc(inline)]
    pub use super::linux::all::*;

    #[doc(inline)]
    pub use super::{linux::all::*, terminal::*};

    #[doc(inline)]
    pub use super::macros::*;
}

// do not block for non-linux oses so it can be used from no_std.
#[cfg_attr(feature = "nightly", doc(cfg(target_os = "linux")))]
pub mod linux;
pub mod terminal;

#[doc(inline)]
pub use terminal::*;

// `os_` functions and macros derive to specific OS implementations.
// For now, only linux is supported.

#[cfg_attr(feature = "nightly", doc(cfg(target_os = "linux")))]
mod macros;
pub use macros::*;
