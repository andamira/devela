//
//! Defines [`Ssd13xxI2c`].
//

use crate::{I2cAddr7, I2cWrite, Ssd13xxWrite};

const COMMAND_CONTROL: &[u8] = &[0x00];
const DATA_CONTROL: &[u8] = &[0x40];

#[doc = crate::_tags!(hw io protocol)]
/// An SSD13xx interface over I²C.
#[doc = crate::_doc_meta!{
    location("device/display", struct Ssd13xxI2c),
    #[cfg(target_pointer_size = "32")]
    test_size_of(Ssd13xxI2c = 8|64; niche Option),
}]
pub struct Ssd13xxI2c<'a, I> {
    i2c: &'a mut I,
    address: I2cAddr7,
}

impl<'a, I> Ssd13xxI2c<'a, I> {
    /// Creates an SSD13xx I²C interface.
    #[must_use]
    pub const fn new(i2c: &'a mut I, address: I2cAddr7) -> Self {
        Self { i2c, address }
    }
    /// Returns the target I²C address.
    #[must_use]
    pub const fn address(&self) -> I2cAddr7 {
        self.address
    }
}

impl<I: I2cWrite> Ssd13xxWrite for Ssd13xxI2c<'_, I> {
    type Error = I::Error;

    fn write_commands(&mut self, commands: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write_slices(self.address, &[COMMAND_CONTROL, commands])
    }
    fn write_data(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write_slices(self.address, &[DATA_CONTROL, data])
    }
}
