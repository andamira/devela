// devela::str
//
//! Synchronization, extends [`core::sync`].
//

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}

pub mod atomic {
    //! Atomic types.
    //!
    //! It also reexports the [`Atomic`] type from the
    //! [`atomic`](https://docs.rs/atomic) crate,
    //! and some useful definitions from `libcore`.

    #[doc = "[`atomic`](https://docs.rs/atomic) crate's generic atomic wrapper type.\n\n---"]
    #[doc(inline)]
    pub use ::atomic::Atomic;

    /* Re-export some useful definitions from libcore */

    #[doc = "`::core::sync::`[`atomic`](core::sync::atomic) fence.\n\n---"]
    pub use core::sync::atomic::fence as atomic_fence;

    #[doc = "`::core::sync::`[`atomic`](core::sync::atomic) memory ordering.\n\n---"]
    pub use core::sync::atomic::Ordering as AtomicOrdering;
}
