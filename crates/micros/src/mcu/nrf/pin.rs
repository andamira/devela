//
//! Defines [`NrfPin`].
//

use crate::NrfPort;

#[doc = crate::_tags!(hw io)]
/// nRF52840 GPIO pin identified by its port and bit position.
#[doc = crate::_doc_meta!{
    location("mcu/nrf", struct NrfPin),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NrfPin {
    port: NrfPort,
    bit: u8,
}

#[rustfmt::skip]
impl NrfPin {
    /// Creates a pin in the implemented range for its port.
    ///
    /// # Panics
    /// Panics for an out-of-range pin number.
    #[must_use]
    pub const fn new(port: NrfPort, bit: u8) -> Self {
        assert!(bit < port.pins(), "invalid nRF GPIO pin");
        Self { port, bit }
    }

    /// Returns the GPIO port.
    #[must_use]
    pub const fn port(self) -> NrfPort { self.port }

    /// Returns the bit position within the port.
    #[must_use]
    pub const fn bit(self) -> u8 { self.bit }

    /// Returns the pin's bit mask.
    #[must_use]
    pub const fn mask(self) -> u32 { 1 << self.bit }
}

#[cfg(feature = "unsafe_mmio")]
impl NrfPin {
    /// Enables GPIO output, preserving the output latch.
    ///
    /// # Safety
    /// The pin must belong to the active nRF52840, be free of conflicting
    /// peripheral ownership, and be electrically safe to drive. No concurrent
    /// accesses may modify the same pin configuration.
    pub unsafe fn set_output(self) {
        unsafe { self.port.dirset().write(self.mask()) }
    }

    /// Configures this GPIO output initially low (avoids a high-going glitch).
    ///
    /// # Safety
    /// The pin must belong to the active nRF52840 and may be driven low.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.set_output();
        }
    }

    /// Configures this GPIO output initially high (avoids a low-going glitch).
    ///
    /// # Safety
    /// The pin must belong to the active nRF52840 and may be driven high.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.set_output();
        }
    }

    /// Sets the output latch high using an atomic bit-set register.
    ///
    /// # Safety
    /// GPIO ownership and electrical drive constraints must be respected.
    pub unsafe fn set_high(self) {
        unsafe { self.port.outset().write(self.mask()) }
    }

    /// Sets the output latch low using an atomic bit-clear register.
    ///
    /// # Safety
    /// GPIO ownership and electrical drive constraints must be respected.
    pub unsafe fn set_low(self) {
        unsafe { self.port.outclr().write(self.mask()) }
    }

    /// Toggles the current output latch through a read and a separate write.
    ///
    /// Not an atomic operation with respect to other writers.
    ///
    /// # Safety
    /// GPIO ownership and electrical drive constraints must be respected.
    pub unsafe fn toggle(self) {
        unsafe {
            if self.port.out().read() & self.mask() != 0 {
                self.set_low();
            } else {
                self.set_high();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NrfPin, NrfPort};
    #[test]
    fn port_boundaries() {
        assert_eq!(NrfPin::new(NrfPort::P0, 31).mask(), 1 << 31);
        assert_eq!(NrfPin::new(NrfPort::P1, 15).bit(), 15);
    }
    #[test]
    #[should_panic]
    fn rejects_invalid_p1_bit() {
        NrfPin::new(NrfPort::P1, 16);
    }
}
