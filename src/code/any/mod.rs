// devela::code::any
//
//! Dynamic typing and reflection, extends `std::`[`any`].
//!
//! [`any`]: std::any
//

/* modules */

// always compiled, non public
mod ext;
mod reexports;

/* re-exports */

// always compiled, non public
#[allow(unused_imports)]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
