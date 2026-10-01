//
//! Defines [`I2cBusRead`].
//

use crate::{I2cAddr7, I2cBusWrite};

#[doc = crate::_tags!(hw io protocol)]
/// Blocking read access to an I²C bus.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", trait I2cBusRead),
}]
/// Extends [`I2cBusWrite`] with read transactions and the common
/// repeated-START write→read transaction used by register-oriented devices.
pub trait I2cBusRead: I2cBusWrite {
    /// Reads bytes from one I²C target.
    ///
    /// An empty buffer performs no transaction.
    fn read(&mut self, address: I2cAddr7, buffer: &mut [u8]) -> Result<(), Self::Error>;

    /// Writes `write`, issues a repeated START, then reads into `read`.
    ///
    /// No STOP condition is inserted between the write and read phases.
    ///
    /// An empty write buffer is equivalent to [`read`](Self::read).
    /// An empty read buffer is equivalent to [`I2cBusWrite::write`].
    fn write_read(
        &mut self,
        address: I2cAddr7,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error>;
}
