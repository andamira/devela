// devela::data::collections::array::d1::methods
//
//! 1-dimensional array methods
//

mod common;

mod bare;

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod boxed;
