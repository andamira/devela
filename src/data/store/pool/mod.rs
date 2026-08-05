// devela/src/data/store/pool/mod.rs
//
//! Reusable stores with stable handles and individual reclamation.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // pool!
mod iter; // PoolIter

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::pool,
            iter::PoolIter,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::{PoolExample, PoolHandleExample};
    }
}
