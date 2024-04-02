// devela::code::any
//
//! Dynamic typing and reflection, extends `std::`[`any`].
//!
//! [`any`]: std::any
//

/* always compiled */

mod ext;
mod reexports;
#[allow(unused_imports)]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
