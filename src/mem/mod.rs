// devela::mem
//
//! Memory management, extends
//! `std::`[`mem`][std::mem].
//

/* contains always compiled items */

mod aligned;
mod always;
mod reexports_core;
mod size;
mod storage;
mod r#trait;

#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub(crate) use {aligned::*, always::*, r#trait::*, reexports_core::*, size::*, storage::*};

/* feature-gated */

#[cfg(feature = "mem")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {aligned::*, always::*, fns::*, r#trait::*, reexports_core::*, size::*, storage::*};

#[cfg(feature = "mem")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{aligned::*, always::*, r#trait::*, reexports_core::*, size::*, storage::*};
}
