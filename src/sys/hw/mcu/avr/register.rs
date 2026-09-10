// devela/sys/hw/mcu/avr/register.rs
//
//! Defines [`AvrReg8`].
//

use crate::{Ptr, is};

#[doc = crate::_tags!(hw)]
/// An 8-bit AVR memory-mapped hardware register.
#[doc = crate::_doc_meta!{
    location("hw/mcu/avr", struct AvrReg8),
    test_size_of(AvrReg8 = 2|16; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrReg8(u16);

#[rustfmt::skip]
impl AvrReg8 {
    /// Creates an 8-bit AVR register at a data-space address.
    #[must_use]
    pub const fn new(data_addr: u16) -> Self { Self(data_addr) }

    /// Returns its data-space address.
    #[must_use]
    pub const fn addr(self) -> u16 { self.0 }

    /// Returns its I/O-space address, when one exists.
    #[must_use]
    pub const fn io_addr(self) -> Option<u8> {
        is! { self.0 >= 0x20 && self.0 <= 0x5f, Some((self.0 - 0x20) as u8), None }
    }
    /// Returns its memory-mapped immutable pointer.
    #[must_use]
    pub const fn as_ptr(self) -> *const u8 { Ptr::without_provenance(self.0 as usize) }

    /// Returns its memory-mapped mutable pointer.
    #[must_use]
    pub const fn as_mut_ptr(self) -> *mut u8 { Ptr::without_provenance_mut(self.0 as usize) }

    /// Performs a volatile read from this register.
    ///
    /// # Safety
    /// This must be a readable 8-bit register on the active device,
    /// and any hardware effects caused by reading it must be valid here.
    #[must_use]
    #[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
    pub unsafe fn read(self) -> u8 {
        unsafe { Ptr::read_volatile(self.as_ptr()) }
    }
    /// Performs a volatile write to this register.
    ///
    /// # Safety
    /// This must be a writable 8-bit register on the active device,
    /// and the write must obey that register's hardware semantics.
    #[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
    pub unsafe fn write(self, value: u8) {
        unsafe { Ptr::write_volatile(self.as_mut_ptr(), value) }
    }
}
