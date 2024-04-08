// devela::code::any
//
//! Dynamic typing and reflection, extends `std::`[`any`].
//!
//! [`any`]: std::any
//

mod ext;
mod reexports;
#[allow(unused_imports)]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
