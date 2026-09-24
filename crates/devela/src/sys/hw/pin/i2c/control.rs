//
//! Defines [`I2cControl`], [`I2cController`].
//

use crate::{I2cAddr7, I2cBusWrite};

#[doc = crate::_tags!(hw io protocol)]
/// Low-level control of an I²C controller.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", trait I2cControl),
}]
/// This is an extension interface for MCU-specific and custom I²C
/// implementations. Application and device-driver code should
/// normally use higher-level capabilities such as [`I2cBusWrite`].
///
/// Implementors can be wrapped in [`I2cController`] to expose safe I²C
/// capabilities after the underlying hardware has been
/// configured and exclusively acquired.
///
/// # Safety
/// Implementations must uphold the documented requirements of every
/// unchecked operation when invoked through an exclusively owned,
/// correctly configured controller.
pub unsafe trait I2cControl {
    /// Error returned by controller operations.
    type Error;

    /// Writes a sequence of byte slices as one uninterrupted transaction.
    ///
    /// Empty slices are ignored. If the complete payload is empty, only the
    /// write-address byte is transmitted before STOP.
    ///
    /// # Safety
    /// `self` must represent exclusive access to a correctly configured
    /// I²C controller.
    unsafe fn write_slices_unchecked(
        &mut self,
        address: I2cAddr7,
        slices: &[&[u8]],
    ) -> Result<(), Self::Error>;
}

#[doc = crate::_tags!(hw io protocol)]
/// An exclusively owned, configured I²C controller.
#[doc = crate::_doc_meta!{
    location("sys/hw/pin/i2c", struct I2cController),
}]
/// Wraps a low-level [`I2cControl`] implementation after its hardware,
/// pins, and bus timing have been configured.
///
/// This type is intentionally neither [`Clone`] nor [`Copy`]. Its safe I²C
/// capabilities require mutable access, representing exclusive use of the
/// underlying controller.
#[repr(transparent)]
#[derive(Debug)]
pub struct I2cController<C: I2cControl> {
    control: C,
}

impl<C: I2cControl> I2cController<C> {
    /// Creates a controller from an already configured low-level implementation.
    ///
    /// # Safety
    /// `control` must represent exclusive access to a correctly configured
    /// I²C controller. The same hardware resources must not be accessed
    /// through another handle while this value is alive.
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

impl<C: I2cControl> I2cBusWrite for I2cController<C> {
    type Error = C::Error;

    fn write_slices(&mut self, address: I2cAddr7, slices: &[&[u8]]) -> Result<(), Self::Error> {
        unsafe { self.control.write_slices_unchecked(address, slices) }
    }
}
