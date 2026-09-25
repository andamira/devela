//
//! Defines [`SamPort`].
//

use crate::SamReg32;

#[doc = crate::_tags!(hw io)]
/// A SAM GPIO port backed by a Parallel I/O Controller (PIO).
#[doc = crate::_doc_meta!{
    location("mcu/sam", struct SamPort),
    test_size_of(SamPort = 4|32; niche !Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SamPort {
    base: u32,
}

#[rustfmt::skip]
impl SamPort {
    /// Creates a SAM GPIO port from its PIO base address.
    #[must_use]
    pub const fn new(base: u32) -> Self { Self { base } }

    /// Returns its PIO base address.
    #[must_use]
    pub const fn base(self) -> u32 { self.base }

    /// Returns the PIO Enable Register (`PIO_PER`).
    pub(super) const fn per(self) -> SamReg32 { SamReg32::new(self.base) }

    /// Returns the Output Enable Register (`PIO_OER`).
    pub(super) const fn oer(self) -> SamReg32 { SamReg32::new(self.base + 0x10) }

    /// Returns the Set Output Data Register (`PIO_SODR`).
    pub(super) const fn sodr(self) -> SamReg32 { SamReg32::new(self.base + 0x30) }

    /// Returns the Clear Output Data Register (`PIO_CODR`).
    pub(super) const fn codr(self) -> SamReg32 { SamReg32::new(self.base + 0x34) }

    /// Returns the Pull-Up Disable Register (`PIO_PUDR`).
    pub(super) const fn pudr(self) -> SamReg32 { SamReg32::new(self.base + 0x60) }
}
