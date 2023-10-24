// devela::mem
//
//! Memory management, extends
//! `std::`[`mem`][std::mem].
//

/* contains always compiled items */

mod aligned;
mod always_fns;
mod reexports_core;
mod size;
mod storage;
mod r#trait;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub(crate) use {aligned::*, always_fns::*, r#trait::*, reexports_core::*, size::*, storage::*};

/* feature-gated */

// ...

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {aligned::*, always_fns::*, r#trait::*, reexports_core::*, size::*, storage::*};

// re-export external dependencies
#[doc(no_inline)]
#[cfg(any(feature = "bytemuck", all(feature = "mem", feature = "depend")))]
pub use ::depend::bytemuck;

#[cfg(feature = "mem")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        aligned::*, always_fns::*, r#trait::*, reexports_core::*, size::*, storage::*,
    };
}
