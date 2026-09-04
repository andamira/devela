// devela/src/sys/mem/alloc/storage/_.rs
//
//! The [`Storage`] trait allows data structures to abstract over how data is stored,
//! enabling specialization by storage strategy (e.g. stack vs heap).
//!
//! It is already implemented for the [`Bare`] and [`Boxed`] type markers,
//! which wraps their data in a [`BareBox`] and a [`Box`], respectively.
//

crate::mods_in! {
    mod bare;
    #[cfg(feature = "alloc")]
    mod boxed;
    mod traits;
}
crate::mods_out! {
    _mods {
        pub use super::{
            bare::{Bare, BareBox},
            traits::Storage
        };
        #[cfg(feature = "alloc")]
        pub use super::{
            boxed::Boxed,
        };
    }
}
