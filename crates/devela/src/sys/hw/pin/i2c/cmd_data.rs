//
//! Defines [`I2cCmdData`].
//

use crate::{CmdDataWrite, I2cBusWrite, I2cTarget};

#[cfg(target_pointer_width = "16")]
crate::test_size_of!(const I2cCmdData<()> = 5|40; niche Option);

#[doc = crate::_tags!(hw io protocol)]
/// Command/data framing over an addressed I²C target.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", enum I2cCmdData),
    #[cfg(target_pointer_width = "16")]
    test_size_of(I2cCmdData<()> = 5|40; niche Option),
    #[cfg(target_pointer_width = "32")]
    test_size_of(I2cCmdData<()> = 12|96; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(I2cCmdData<()> = 24|192; niche Option),
}]
/// Prefixes command and data transactions with their respective control byte.
pub struct I2cCmdData<'a, B: ?Sized> {
    target: I2cTarget<'a, B>,
    cmd: u8,
    data: u8,
}

impl<'a, B: ?Sized> I2cCmdData<'a, B> {
    /// Creates command/data framing over `target` using the given control-byte prefixes.
    #[must_use]
    pub const fn new(target: I2cTarget<'a, B>, cmd: u8, data: u8) -> Self {
        Self { target, cmd, data }
    }
}

impl<B: I2cBusWrite + ?Sized> CmdDataWrite for I2cCmdData<'_, B> {
    type Error = B::Error;

    fn write_cmd(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.target.write_slices(&[&[self.cmd], bytes])
    }

    fn write_data(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.target.write_slices(&[&[self.data], bytes])
    }
}
