//
//! Defines [`Esp32C6SpiCmdData`].
//

use crate::{CmdDataWrite, Esp32C6Pin, EspSpi, SpiBusWrite, SpiController, Timeout, is};

#[doc = crate::_tags!(hw io protocol)]
/// ESP32-C6 SPI command/data transport with GPIO-controlled CS and D/C.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct Esp32C6SpiCmdData),
}]
/// Owns the logical use of one configured SPI controller together with its
/// chip-select and data/command pins.
///
/// This type is intentionally neither [`Clone`] nor [`Copy`]. Its safe
/// [`CmdDataWrite`] implementation relies on exclusive ownership established
/// when it is constructed.
#[derive(Debug)]
pub struct Esp32C6SpiCmdData {
    spi: SpiController<EspSpi>,
    cs: Esp32C6Pin,
    dc: Esp32C6Pin,
}

impl Esp32C6SpiCmdData {
    /// Creates a command/data transport around already configured resources.
    ///
    /// The CS pin is initialized inactive-high and D/C as command-low.
    ///
    /// # Safety
    /// `spi`, `cs`, and `dc` must belong to the active ESP32-C6 device and
    /// represent exclusive access to a correctly routed SPI bus and control
    /// pins. The same hardware resources must not be accessed through another
    /// handle while this value is alive.
    #[must_use]
    pub unsafe fn new_unchecked(
        spi: SpiController<EspSpi>,
        cs: Esp32C6Pin,
        dc: Esp32C6Pin,
    ) -> Self {
        unsafe {
            cs.set_output_high();
            dc.set_output_low();
        }
        Self { spi, cs, dc }
    }

    /// Writes one big-endian 16-bit value repeatedly as a single data stream.
    ///
    /// CS remains active across all internal 64-byte SPI controller chunks.
    /// This is useful for generated RGB565 spans that do not exist as one
    /// contiguous source slice.
    pub fn write_data_repeated_u16_be(&mut self, value: u16, count: usize) -> Result<(), Timeout> {
        let bytes = value.to_be_bytes();
        let mut buffer = [0u8; EspSpi::CPU_BUFFER_LEN];
        let mut i = 0;
        while i < buffer.len() {
            buffer[i] = bytes[0];
            buffer[i + 1] = bytes[1];
            i += 2;
        }
        unsafe {
            self.dc.set_high();
            self.cs.set_low();
            let mut remaining = count;
            while remaining != 0 {
                let pixels = remaining.min(buffer.len() / 2);
                if let Err(error) = self.spi.write(&buffer[..pixels * 2]) {
                    self.cs.set_high();
                    return Err(error);
                }
                remaining -= pixels;
            }
            self.cs.set_high();
        }
        Ok(())
    }

    fn transfer(&mut self, data: bool, bytes: &[u8]) -> Result<(), Timeout> {
        unsafe {
            is! { data, self.dc.set_high(), self.dc.set_low() }
            self.cs.set_low();
            let result = self.spi.write(bytes);
            self.cs.set_high();
            result
        }
    }
}

impl CmdDataWrite for Esp32C6SpiCmdData {
    type Error = Timeout;

    fn write_cmd(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.transfer(false, bytes)
    }
    fn write_data(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.transfer(true, bytes)
    }
}
