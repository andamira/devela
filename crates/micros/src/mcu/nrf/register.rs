//
//! Defines [`NrfReg32`].
//

use crate::Ptr;

#[doc = crate::_tags!(hw)]
/// A 32-bit Nordic memory-mapped register address.
#[doc = crate::_doc_meta!{
    location("mcu/nrf", struct NrfReg32),
    test_size_of(NrfReg32 = 4|32; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NrfReg32(u32);

#[rustfmt::skip]
impl NrfReg32 {
    /// Creates a register address. Does not validate the address.
    #[must_use]
    pub const fn new(addr: u32) -> Self { Self(addr) }

    /// Returns its address.
    #[must_use]
    pub const fn addr(self) -> u32 { self.0 }

    /// Returns a pointer to the register.
    #[must_use]
    pub const fn as_ptr(self) -> *const u32 {
        Ptr::without_provenance(self.0 as usize)
    }

    /// Returns a mutable pointer to the register.
    #[must_use]
    pub const fn as_mut_ptr(self) -> *mut u32 {
        Ptr::without_provenance_mut(self.0 as usize)
    }
}

#[cfg(feature = "unsafe_mmio")]
impl NrfReg32 {
    /// Performs a volatile register read.
    ///
    /// # Safety
    /// The register must exist, be readable, and permit the read's side effects.
    #[must_use]
    pub unsafe fn read(self) -> u32 {
        unsafe { Ptr::read_volatile(self.as_ptr()) }
    }

    /// Performs a volatile register write.
    ///
    /// # Safety
    /// The register must exist, be writable, and accept this value.
    pub unsafe fn write(self, value: u32) {
        unsafe { Ptr::write_volatile(self.as_mut_ptr(), value) }
    }
}
