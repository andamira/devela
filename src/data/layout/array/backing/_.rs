// devela/src/data/layout/array/backing/_.rs
//
//! Backing-storage implementations for [`Array`][crate::Array].
//!
//! Each implementation adapts construction, physical-storage access,
//! logical element access, and reborrowing to a particular backing type
//! while preserving the same layout invariant.
//

use crate::{ArrayLayout, MismatchedCapacity, is};

crate::mods_in! {
    mod fixed; // Array implementations over fixed native arrays
    mod slice; // Array implementations over shared and exclusive slices

    #[cfg(feature = "alloc")]
    mod boxed;
    #[cfg(feature = "alloc")]
    mod vec;
}

/// Validates that backing storage covers every position addressed by a layout.
pub(super) const fn validate_storage_len<const RANK: usize>(
    storage_len: usize,
    layout: ArrayLayout<RANK>,
) -> Result<(), MismatchedCapacity> {
    let required = layout.required_storage_len();
    if storage_len < required {
        Err(MismatchedCapacity::too_small(storage_len, required))
    } else {
        Ok(())
    }
}
