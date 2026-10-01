//
//! Defines [`SpiBusWrite`].
//

#[doc = crate::_tags!(hw io protocol)]
/// Blocking byte-write access to an SPI bus.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/spi", trait SpiBusWrite),
}]
/// This models the bus operation itself. It intentionally
/// does not define SPI mode, bus frequency, or device selection;
/// those belong to configuration and device/transaction layers.
///
/// Implementations may split one logical write into multiple
/// hardware-sized transfers and may leave the clock idle between them.
/// Returning `Ok(())` means all bytes have been transmitted.
pub trait SpiBusWrite {
    /// Error returned by the bus implementation.
    type Error;

    /// Writes one byte slice in order on the bus.
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
}
