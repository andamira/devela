//
//! Defines [`SamPin`].
//

use crate::SamPort;

#[doc = crate::_tags!(hw io)]
/// A SAM GPIO pin identified by its PIO port and bit position.
#[doc = crate::_doc_meta!{
    location("mcu/sam", struct SamPin),
    test_size_of(SamPin = 8|64; niche !Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SamPin {
    port: SamPort,
    bit: u8,
}

#[rustfmt::skip]
impl SamPin {
    /// Creates a SAM GPIO pin from a port and bit position.
    ///
    /// # Panics
    /// Panics if `bit` is greater than 31.
    #[must_use]
    pub const fn new(port: SamPort, bit: u8) -> Self {
        assert!(bit < 32, "SAM PIO bit must be in 0..32");
        Self { port, bit }
    }

    /// Returns its PIO port.
    #[must_use]
    pub const fn port(self) -> SamPort { self.port }

    /// Returns its bit position within the port.
    #[must_use]
    pub const fn bit(self) -> u8 { self.bit }

    /// Returns its one-bit mask within the port.
    #[must_use]
    pub const fn mask(self) -> u32 { 1 << self.bit }
}

#[cfg(feature = "unsafe_mmio")]
impl SamPin {
    /// Configures this pin as an output, preserving its output latch.
    ///
    /// The internal pull-up is disabled and control is given to the PIO
    /// controller.
    ///
    /// # Safety
    /// The pin and its port must belong to the active device,
    /// and the caller must ensure exclusive control of this pin.
    pub unsafe fn set_output(self) {
        let mask = self.mask();
        unsafe {
            self.port.pudr().write(mask);
            self.port.oer().write(mask);
            self.port.per().write(mask);
        }
    }
    /// Configures this pin as an output initially driven low.
    ///
    /// # Safety
    /// The pin and its port must belong to the active device,
    /// and the caller must ensure exclusive control of this pin.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.set_output();
        }
    }
    /// Configures this pin as an output initially driven high.
    ///
    /// # Safety
    /// The pin and its port must belong to the active device,
    /// and the caller must ensure exclusive control of this pin.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.set_output();
        }
    }

    /// Drives this PIO output high.
    ///
    /// # Safety
    /// The pin must belong to the active device, be under PIO control,
    /// be configured as an output, and not be concurrently controlled.
    pub unsafe fn set_high(self) {
        unsafe { self.port.sodr().write(self.mask()) }
    }
    /// Drives this PIO output low.
    ///
    /// # Safety
    /// The pin must belong to the active device, be under PIO control,
    /// be configured as an output, and not be concurrently controlled.
    pub unsafe fn set_low(self) {
        unsafe { self.port.codr().write(self.mask()) }
    }
}
