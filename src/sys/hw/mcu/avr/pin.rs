// devela/sys/hw/mcu/avr/pin.rs
//
//! Defines [`AvrPin`].
//

use crate::AvrPort;

#[doc = crate::_tags!(hw io)]
/// An AVR GPIO pin identified by its port and bit position.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr", struct AvrPin),
    test_size_of(AvrPin = 7|56; niche !Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrPin {
    port: AvrPort,
    bit: u8,
}

#[rustfmt::skip]
impl AvrPin {
    /// Creates an AVR GPIO pin from a port and bit position.
    ///
    /// # Panics
    /// Panics if `bit` is greater than 7.
    #[must_use]
    pub const fn new(port: AvrPort, bit: u8) -> Self {
        assert!(bit < 8, "AVR GPIO bit must be in 0..8");
        Self { port, bit }
    }

    /// Returns its GPIO port.
    #[must_use]
    pub const fn port(self) -> AvrPort { self.port }

    /// Returns its bit position within the port.
    #[must_use]
    pub const fn bit(self) -> u8 { self.bit }

    /// Returns its one-bit mask within the port.
    #[must_use]
    pub const fn mask(self) -> u8 { 1 << self.bit }

    /// Returns its port and bit position.
    #[must_use]
    pub const fn into_parts(self) -> (AvrPort, u8) {
        (self.port, self.bit)
    }
}

#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrPin {
    /// Returns whether this pin is configured as an output.
    pub unsafe fn is_output(self) -> bool {
        unsafe { self.port.ddr_reg().read() & self.mask() != 0 }
    }
    /// Returns whether this pin is configured as an input.
    pub unsafe fn is_input(self) -> bool {
        unsafe { !self.is_output() }
    }

    /// Configures this pin as an output, preserving its `PORTx` latch.
    pub unsafe fn set_output(self) {
        let reg = self.port.ddr_reg();
        unsafe { reg.write(reg.read() | self.mask()) };
    }
    /// Configures this pin as an output initially driven low.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low(); // input becomes floating first if necessary
            self.set_output();
        }
    }
    /// Configures this pin as an output initially driven high.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high(); // input pull-up is the intermediate state
            self.set_output();
        }
    }

    /// Configures this pin as an input, preserving its `PORTx` latch.
    pub unsafe fn set_input(self) {
        let reg = self.port.ddr_reg();
        unsafe { reg.write(reg.read() & !self.mask()) };
    }
    /// Configures this pin as a floating input.
    pub unsafe fn set_input_floating(self) {
        unsafe {
            self.set_input();
            self.disable_pullup();
        }
    }
    /// Configures this pin as an input with its pull-up enabled.
    pub unsafe fn set_input_pullup(self) {
        unsafe {
            self.set_input();
            self.enable_pullup();
        }
    }

    /// Sets this pin's `PORTx` latch.
    ///
    /// As an output this drives the pin high.
    /// As an input this enables its pull-up resistor.
    pub unsafe fn set_high(self) {
        let reg = self.port.port_reg();
        unsafe { reg.write(reg.read() | self.mask()) };
    }
    /// Clears this pin's `PORTx` latch.
    ///
    /// As an output this drives the pin low.
    /// As an input this disables its pull-up resistor.
    pub unsafe fn set_low(self) {
        let reg = self.port.port_reg();
        unsafe { reg.write(reg.read() & !self.mask()) };
    }
    /// Toggles this pin's `PORTx` latch.
    pub unsafe fn toggle(self) {
        unsafe { self.port.pin_reg().write(self.mask()) };
    }

    /// Reads the current input level of this pin.
    pub unsafe fn is_high(self) -> bool {
        unsafe { self.port.pin_reg().read() & self.mask() != 0 }
    }
    /// Returns whether the current input level of this pin is low.
    pub unsafe fn is_low(self) -> bool {
        unsafe { !self.is_high() }
    }

    /// Enables this pin's pull-up resistor when configured as an input.
    pub unsafe fn enable_pullup(self) {
        unsafe { self.set_high() }
    }
    /// Disables this pin's pull-up resistor when configured as an input.
    pub unsafe fn disable_pullup(self) {
        unsafe { self.set_low() }
    }
}
