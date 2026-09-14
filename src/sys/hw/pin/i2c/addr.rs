// devela/sys/hw/pin/i2c/addr.rs
//
//! Defines [`I2cAddr7`].
//

#[doc = crate::_tags!(hw io protocol)]
/// A 7-bit I²C target address.
///
/// Stores the unshifted address value in `0..=0x7f`;
/// the transfer-direction bit is not part of the address.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", struct I2cAddr7),
    test_size_of(I2cAddr7 = 1|8; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct I2cAddr7(u8);

impl I2cAddr7 {
    /// Creates a 7-bit I²C address.
    ///
    /// # Panics
    /// Panics if `addr` is greater than `0x7f`.
    #[must_use]
    pub const fn new(addr: u8) -> Self {
        assert!(addr < 0x80, "I²C 7-bit address must fit in 7 bits");
        Self(addr)
    }

    /// Creates a 7-bit I²C address if `addr` fits in 7 bits.
    #[must_use]
    pub const fn new_checked(addr: u8) -> Option<Self> {
        if addr < 0x80 { Some(Self(addr)) } else { None }
    }

    /// Returns the unshifted 7-bit address value.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}
