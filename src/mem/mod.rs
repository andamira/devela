// devela::mem
//
//! Memory management, extends
//! `std::`[`mem`][std::mem].
//

/* contains always compiled items */

mod always_fns;
mod storage;
#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub(crate) use {always_fns::*, storage::*};

/* feature-gated */

#[cfg(feature = "mem")]
mod reexports;
#[cfg(feature = "mem")]
mod size;
#[cfg(feature = "mem")]
mod r#trait;

// private sub-modules
#[cfg(feature = "mem")]
pub use {always_fns::*, r#trait::*, reexports::*, size::*, storage::*};

// external dependencies
#[doc(no_inline)]
#[cfg(any(feature = "bytemuck", all(feature = "mem", feature = "depend")))]
pub use ::depend::bytemuck;

#[cfg(feature = "mem")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always_fns::*, r#trait::*, reexports::*, size::*, storage::*};
}
