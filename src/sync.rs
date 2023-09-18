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

    #[doc = "A generic atomic wrapper type.\n\n"]
    #[doc = "*Reexported from the [`atomic`](https://docs.rs/atomic)* crate.\n\n---"]
    #[doc(inline)]
    pub use ::atomic::Atomic;

    /* Re-export some useful definitions from libcore */

    #[doc = "An atomic fence.\n\n"]
    #[doc = "*Reexported from"]
    #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
    pub use core::sync::atomic::fence as atomic_fence;

    #[doc = "Atomic memory ordering.\n\n"]
    #[doc = "*Reexported from"]
    #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
    pub use core::sync::atomic::Ordering as AtomicOrdering;
}
