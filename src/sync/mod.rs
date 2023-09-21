// devela::sync
//
//! Synchronization, extends [`core::sync`].
//

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}

pub mod atomic;
