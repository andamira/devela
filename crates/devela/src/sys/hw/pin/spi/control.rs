//
//! Defines [`SpiControl`], [`SpiController`].
//

use crate::SpiBusWrite;

#[doc = crate::_tags!(hw io protocol)]
/// Low-level control of an SPI controller.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/spi", trait SpiControl),
}]
/// This is an extension interface for MCU-specific and custom SPI implementations.
/// Application and device-driver code should normally use
/// higher-level capabilities such as [`SpiBusWrite`].
///
/// Implementors can be wrapped in [`SpiController`] to expose safe SPI capabilities
/// after the underlying hardware has been configured and exclusively acquired.
///
/// # Safety
/// Implementations must uphold the documented requirements of every unchecked operation
/// when invoked through an exclusively owned, correctly configured controller.
pub unsafe trait SpiControl {
    /// Error returned by controller operations.
    type Error;

    /// Writes one byte slice in order on the bus.
    ///
    /// # Safety
    /// `self` must represent exclusive access to a correctly
    /// configured SPI controller and its routed bus pins.
    unsafe fn write_unchecked(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
}

#[doc = crate::_tags!(hw io protocol)]
/// An exclusively owned, configured SPI controller.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/spi", struct SpiController),
}]
/// Wraps a low-level [`SpiControl`] implementation after its hardware,
/// pins, bus mode, and timing have been configured.
///
/// This type is intentionally neither [`Clone`] nor [`Copy`]. Its safe SPI capabilities
/// require mutable access, representing exclusive use of the underlying controller.
#[repr(transparent)]
#[derive(Debug)]
pub struct SpiController<C: SpiControl> {
    control: C,
}

impl<C: SpiControl> SpiController<C> {
    /// Creates a controller from an already configured low-level implementation.
    ///
    /// # Safety
    /// `control` must represent exclusive access to a correctly configured SPI
    /// controller and its routed bus pins. The same hardware resources must not
    /// be accessed through another handle while this value is alive.
    #[must_use]
    pub const unsafe fn new_unchecked(control: C) -> Self {
        Self { control }
    }

    /// Consumes the controller and returns its low-level implementation.
    #[must_use]
    pub fn into_inner(self) -> C {
        self.control
    }
}

impl<C: SpiControl> SpiBusWrite for SpiController<C> {
    type Error = C::Error;

    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        unsafe { self.control.write_unchecked(bytes) }
    }
}
