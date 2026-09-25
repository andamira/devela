//
//! Defines [`SamReg32`].
//

use crate::Ptr;

#[doc = crate::_tags!(hw)]
/// A 32-bit SAM memory-mapped hardware register.
#[doc = crate::_doc_meta!{
    location("mcu/sam", struct SamReg32),
    test_size_of(SamReg32 = 4|32; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SamReg32(u32);

#[rustfmt::skip]
impl SamReg32 {
    /// Creates a 32-bit SAM register at a memory-mapped address.
    #[must_use]
    pub const fn new(addr: u32) -> Self { Self(addr) }

    /// Returns its memory-mapped address.
    #[must_use]
    pub const fn addr(self) -> u32 { self.0 }

    /// Returns its memory-mapped immutable pointer.
    #[must_use]
    pub const fn as_ptr(self) -> *const u32 {
        Ptr::without_provenance(self.0 as usize)
    }

    /// Returns its memory-mapped mutable pointer.
    #[must_use]
    pub const fn as_mut_ptr(self) -> *mut u32 {
        Ptr::without_provenance_mut(self.0 as usize)
    }
}

#[cfg(feature = "unsafe_mmio")]
impl SamReg32 {
    /// Performs a volatile read from this register.
    ///
    /// # Safety
    /// This must be a readable 32-bit register on the active device,
    /// and any hardware effects caused by reading it must be valid here.
    #[must_use]
    pub unsafe fn read(self) -> u32 {
        unsafe { Ptr::read_volatile(self.as_ptr()) }
    }

    /// Performs a volatile write to this register.
    ///
    /// # Safety
    /// This must be a writable 32-bit register on the active device,
    /// and the write must obey that register's hardware semantics.
    pub unsafe fn write(self, value: u32) {
        unsafe { Ptr::write_volatile(self.as_mut_ptr(), value) }
    }
}
