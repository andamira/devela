//
//! Defines [`AvrUsartTx`].
//

use crate::{AvrUsart, DiagLevel, DiagOut, Infallible, TextOut};

#[doc = crate::_tags!(hw io)]
/// An exclusively used blocking AVR USART transmitter.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct AvrUsartTx),
    test_size_of(AvrUsartTx = 12|96; niche !Option),
}]
/// This wraps an [`AvrUsart`] after configuring its transmitter,
/// allowing subsequent writes through a safe interface.
///
/// The wrapper represents logical exclusive access only.
/// Since [`AvrUsart`] itself is copyable, the caller creating this value
/// must ensure that no copied handle accesses the same USART concurrently.
#[derive(Debug)]
pub struct AvrUsartTx {
    usart: AvrUsart,
}

impl AvrUsartTx {
    /// Configures `usart` for transmit-only asynchronous 8N1 operation.
    ///
    /// # Panics
    /// Panics if `clock_hz` or `baud` is zero,
    /// or if the requested rate cannot be represented by the AVR baud generator.
    ///
    /// # Safety
    /// `usart` must describe a USART on the active device.
    ///
    /// The caller must also ensure that the same USART is not accessed
    /// through another handle while the returned transmitter is in use.
    #[must_use]
    pub unsafe fn configure_8n1(usart: AvrUsart, clock_hz: u32, baud: u32) -> Self {
        unsafe { usart.configure_tx_8n1(clock_hz, baud) };
        Self { usart }
    }

    /// Queues one byte for transmission, waiting until the USART can accept it.
    pub fn write_byte_blocking(&mut self, byte: u8) {
        unsafe { self.usart.write_byte_blocking(byte) };
    }
    /// Queues all bytes for transmission, waiting for space as needed.
    pub fn write_bytes_blocking(&mut self, bytes: &[u8]) {
        unsafe { self.usart.write_bytes_blocking(bytes) };
    }

    /// Releases the transmitter wrapper and returns its raw USART descriptor.
    #[must_use]
    pub const fn into_inner(self) -> AvrUsart {
        self.usart
    }
}

impl TextOut for AvrUsartTx {
    type Error = Infallible;

    fn write_text(&mut self, text: &str) -> Result<(), Self::Error> {
        self.write_bytes_blocking(text.as_bytes());
        Ok(())
    }
}

impl DiagOut for AvrUsartTx {
    type Error = Infallible;

    fn diag(&mut self, level: DiagLevel, text: &str) -> Result<(), Self::Error> {
        self.write_bytes_blocking(b"[");
        self.write_bytes_blocking(level.as_str().as_bytes());
        self.write_bytes_blocking(b"] ");
        self.write_bytes_blocking(text.as_bytes());
        self.write_bytes_blocking(b"\r\n");
        Ok(())
    }
}
