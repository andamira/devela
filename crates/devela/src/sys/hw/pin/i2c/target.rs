//
//! Defines [`I2cTarget`].
//

use crate::{I2cAddr7, I2cBusRead, I2cBusWrite, I2cCmdData};

#[cfg(target_pointer_width = "16")]
crate::test_size_of!(const I2cTarget<()> = 3|24; niche Option);

#[doc = crate::_tags!(hw io protocol)]
/// An I²C bus target with a bound address.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", enum I2cTarget),
    #[cfg(target_pointer_width = "16")]
    test_size_of(I2cTarget<()> = 3|24; niche Option),
    #[cfg(target_pointer_width = "32")]
    test_size_of(I2cTarget<()> = 8|64; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(I2cTarget<()> = 16|128; niche Option),
}]
pub struct I2cTarget<'a, B: ?Sized> {
    bus: &'a mut B,
    address: I2cAddr7,
}

impl<'a, B: ?Sized> I2cTarget<'a, B> {
    /// Binds `address` to the given I²C bus.
    #[must_use]
    pub const fn new(bus: &'a mut B, address: I2cAddr7) -> Self {
        Self { bus, address }
    }
    /// Returns the bound I²C target address.
    #[must_use]
    pub const fn address(&self) -> I2cAddr7 {
        self.address
    }
    /// Wraps this target with command/data control-byte framing.
    #[must_use]
    pub const fn cmd_data(self, cmd: u8, data: u8) -> I2cCmdData<'a, B> {
        I2cCmdData::new(self, cmd, data)
    }
}

impl<B: I2cBusRead + ?Sized> I2cTarget<'_, B> {
    /// Reads bytes from this target.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), B::Error> {
        self.bus.read(self.address, buffer)
    }
    /// Writes bytes, issues a repeated START, then reads from this target.
    pub fn write_read(&mut self, write: &[u8], read: &mut [u8]) -> Result<(), B::Error> {
        self.bus.write_read(self.address, write, read)
    }
}

impl<B: I2cBusWrite + ?Sized> I2cTarget<'_, B> {
    /// Writes one byte slice to this target as a single transaction.
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), B::Error> {
        self.bus.write(self.address, bytes)
    }
    /// Writes byte slices to this target as one uninterrupted transaction.
    pub fn write_slices(&mut self, slices: &[&[u8]]) -> Result<(), B::Error> {
        self.bus.write_slices(self.address, slices)
    }
    /// Tests whether this target acknowledges its write address.
    pub fn probe(&mut self) -> Result<(), B::Error> {
        self.bus.probe(self.address)
    }
}
