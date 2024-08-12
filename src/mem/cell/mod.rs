// devela::mem::cell
//
//! Shareable mutable containers, <small>extends:
//! `std::`[`cell`]`.`</small>
//!
//! [`cell`]: std::cell
//

mod option;
mod reexports;
#[allow(unused_imports)]
pub use {option::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{option::*, reexports::*};
}
