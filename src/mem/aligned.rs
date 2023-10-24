// devela::aligned
//
//! Always available for internal use.
//

use core::mem::align_of;

// Marker trait to prevent downstream implementations of the `MemAligned` trait.
mod private {
    pub trait Sealed {}
}
impl<S, L> private::Sealed for (S, L) {}

/// Marker trait used to check the memory alignment between two types.
pub trait MemAligned: private::Sealed {
    #[doc(hidden)]
    fn check();
}

// #[cfg(feature = "full_const_generics")]
// impl<S, L> MemAligned for (S, L)
// where
//     [(); align_of::<L>() - align_of::<S>()]: Sized,
// {
//     fn check() {}
// }
// #[cfg(not(feature = "full_const_generics"))]
impl<S, L> MemAligned for (S, L) {
    fn check() {
        assert!(
            align_of::<S>() <= align_of::<L>(),
            "Enforce alignment >{} (requires {})",
            align_of::<L>(),
            align_of::<S>()
        );
    }
}
