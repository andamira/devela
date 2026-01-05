// devela::sys::mem::ptr::fat
//
//! Fat pointers.
//
// https://github.com/storycraft/unsized-stack
// Vendored from [unsized-stack](https://crates.io/crates/unsized-stack/0.2.0)
//
// MODIFICATIONS:
// - refactored, documented, namespaced.

// WAIT: [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
// use core::ptr::{metadata, from_raw_parts};

#[doc = crate::_TAG_MEM!()]
/// Represents a fat pointer with separate data and metadata pointers.
#[doc = crate::_doc_location!("sys/mem")]
///
#[doc = crate::_doc!(vendor: "unsized-stack")]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FatPtr {
    ptr: *const (),
    metadata: *const (),
}

impl FatPtr {
    /// Creates a new `FatPtr` from a data pointer and metadata.
    pub const fn new<T>(ptr: *const T, metadata: *const ()) -> Self {
        Self { ptr: ptr as _, metadata }
    }

    /// Returns the raw pointer part of the fat pointer.
    pub const fn ptr(&self) -> *const () {
        self.ptr
    }

    /// Returns the metadata part of the fat pointer.
    pub const fn metadata(&self) -> *const () {
        self.metadata
    }

    /* */

    /// Checks if a type `T` is a valid dynamically sized type (DST).
    // MAYBE make private
    pub const fn check_valid<T: ?Sized>() {
        if size_of::<*const T>() != size_of::<FatPtr>() {
            panic!("Type is not valid DST");
        }
    }

    /// Composes a fat pointer `*const T` from a `FatPtr` structure.
    pub const fn to_raw_ptr<T: ?Sized>(fat_ptr: FatPtr) -> *const T {
        Self::check_valid::<T>();

        // SAFETY: relying on unspecified fat pointer representation
        unsafe { FatPtrRepr { fat_ptr }.ptr_const }
    }
    /// Decomposes a fat pointer `*const T` into a `FatPtr` structure.
    pub const fn from_raw_ptr<T: ?Sized>(fat_ptr: *const T) -> FatPtr {
        Self::check_valid::<T>();

        // SAFETY: relying on unspecified fat pointer representation
        unsafe { FatPtrRepr { ptr_const: fat_ptr }.fat_ptr }
    }

    // WAIT: [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
    // /// Composes a fat pointer `*const T` from a `FatPtr` using stable APIs.
    // pub const fn compose2<T: ?Sized>(fat_ptr: FatPtr) -> *const T {
    //     unsafe { from_raw_parts(fat_ptr.ptr, fat_ptr.metadata as _) }
    // }
    // /// Decomposes a fat pointer `*const T` into a `FatPtr` using stable APIs.
    // pub const fn decompose2<T: ?Sized>(ptr: *const T) -> FatPtr {
    //     FatPtr {
    //         ptr: ptr as *const (),
    //         metadata: metadata(ptr) as *const (),
    //     }
    // }
}

/// A union that represents either a raw or fat pointer.
#[repr(C)]
union FatPtrRepr<T: ?Sized> {
    pub ptr_const: *const T,
    pub fat_ptr: FatPtr,
}

#[cfg(test)]
mod tests {
    use super::FatPtr;

    #[test]
    fn from_raw_ptr() {
        let slice: &[usize] = &[0_usize];
        let fat_ptr = FatPtr::from_raw_ptr(slice as *const [usize]);
        assert_eq!(fat_ptr.metadata(), 1 as *const ());
    }
}
