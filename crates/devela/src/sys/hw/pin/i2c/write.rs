//
//! Defines [`I2cWrite`].
//

use crate::I2cAddr7;

#[doc = crate::_tags!(hw io protocol)]
/// Blocking write access to an I²C bus.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", trait I2cWrite),
}]
/// A call to [`write_slices`](#method.write_slices) represents one uninterrupted
/// I²C write transaction. The supplied slices are concatenated without
/// inserting STOP or repeated-START conditions between them.
///
/// An empty payload performs an address-only write transaction.
pub trait I2cWrite {
    /// Error returned by the bus implementation.
    type Error;

    /// Writes a sequence of byte slices as one uninterrupted transaction.
    ///
    /// Empty slices are ignored. If the complete payload is empty, only the
    /// write-address byte is transmitted before STOP.
    fn write_slices(&mut self, address: I2cAddr7, slices: &[&[u8]]) -> Result<(), Self::Error>;

    /// Writes one byte slice as a single transaction.
    fn write(&mut self, address: I2cAddr7, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write_slices(address, &[bytes])
    }

    /// Tests whether a target acknowledges its write address.
    fn probe(&mut self, address: I2cAddr7) -> Result<(), Self::Error> {
        self.write_slices(address, &[])
    }
}
