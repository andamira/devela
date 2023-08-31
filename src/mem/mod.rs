// devela::mem
//
//! Memory, extends [`core::mem`].
//

/// Reexported [`bytemuck`](https://docs.rs/bytemuck)'s crate types.
/// Gives small utilities for casting between plain data types.
#[cfg(feature = "bytemuck")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "bytemuck")))]
pub mod bytemuck {
    pub use ::bytemuck::{
        checked::{self, CheckedBitPattern},
        offset_of, AnyBitPattern, Contiguous, NoUninit, Pod, PodCastError, PodInOption,
        TransparentWrapper, Zeroable, ZeroableInOption,
    };

    #[cfg(feature = "alloc")]
    pub use ::bytemuck::allocation::{self, TransparentWrapperAlloc};
}
