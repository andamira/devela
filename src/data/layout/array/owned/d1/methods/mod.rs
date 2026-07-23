// devela/src/data/layout/array/owned/d1/methods/mod.rs
//
//! 1-dimensional array methods
//

mod bare;
mod general;

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod boxed;
