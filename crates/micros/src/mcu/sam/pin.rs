//
//! Defines [`SamPin`].
//

use crate::SamPort;

#[doc = crate::_tags!(hw io)]
/// A SAM GPIO pin identified by its PIO port and bit position.
///
/// Provides low-level access to basic PIO output configuration
/// and the output-data latch.
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
    /// Configures this pin as a PIO-controlled output, preserving its output latch.
    ///
    /// The internal pull-up is disabled.
    ///
    /// # Safety
    /// This pin's port must describe the corresponding PIO registers on the active
    /// device. This pin's PIO configuration must not be concurrently modified.
    /// Driving the pin at its current output-latch level must be valid for the
    /// connected circuit.
    pub unsafe fn set_output(self) {
        let mask = self.mask();
        unsafe {
            self.port.pudr().write(mask);
            self.port.oer().write(mask);
            self.port.per().write(mask);
        }
    }

    /// Configures this pin as a PIO-controlled output initially driven low.
    ///
    /// # Safety
    /// This pin's port must describe the corresponding PIO registers on the active
    /// device. This pin's PIO configuration and output latch must not be concurrently
    /// modified. Driving the pin low must be valid for the connected circuit.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.set_output();
        }
    }

    /// Configures this pin as a PIO-controlled output initially driven high.
    ///
    /// # Safety
    /// This pin's port must describe the corresponding PIO registers on the active
    /// device. This pin's PIO configuration and output latch must not be concurrently
    /// modified. Driving the pin high must be valid for the connected circuit.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.set_output();
        }
    }

    /// Sets this pin's output latch high.
    ///
    /// When under PIO control and configured as an output, this drives the pin high.
    ///
    /// # Safety
    /// This pin's port must describe the corresponding PIO registers on the active
    /// device. This pin's output latch must not be concurrently modified. If under
    /// PIO control and configured as an output, driving the pin high must be valid
    /// for the connected circuit.
    pub unsafe fn set_high(self) {
        unsafe { self.port.sodr().write(self.mask()) }
    }

    /// Sets this pin's output latch low.
    ///
    /// When under PIO control and configured as an output, this drives the pin low.
    ///
    /// # Safety
    /// This pin's port must describe the corresponding PIO registers on the active
    /// device. This pin's output latch must not be concurrently modified. If under
    /// PIO control and configured as an output, driving the pin low must be valid
    /// for the connected circuit.
    pub unsafe fn set_low(self) {
        unsafe { self.port.codr().write(self.mask()) }
    }

    /// Toggles this pin's output latch.
    ///
    /// This is a compound operation: it reads the output latch, then performs
    /// a separate set or clear write. Unlike AVR GPIO toggling, this is not a
    /// single write-one-to-toggle register operation.
    ///
    /// When under PIO control and configured as an output,
    /// this drives the pin to the opposite level.
    ///
    /// # Safety
    /// This pin's port must describe the corresponding PIO registers on the active
    /// device. This pin's output latch must not be concurrently modified. If under
    /// PIO control and configured as an output, driving the opposite level must be
    /// valid for the connected circuit.
    pub unsafe fn toggle(self) {
        let mask = self.mask();
        unsafe {
            if self.port.odsr().read() & mask != 0 {
                self.port.codr().write(mask);
            } else {
                self.port.sodr().write(mask);
            }
        }
    }
}
