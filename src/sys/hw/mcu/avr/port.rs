// devela/sys/hw/mcu/avr/port.rs
//
//! Defines [`AvrPort`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw io)]
/// An AVR GPIO port described by its `PINx`, `DDRx`, and `PORTx` registers.
#[doc = crate::_doc_meta!{
    location("hw/mcu/avr", struct AvrPort),
    test_size_of(AvrPort = 6|48; niche !Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrPort {
    pin: AvrReg8,
    ddr: AvrReg8,
    port: AvrReg8,
}
#[rustfmt::skip]
impl AvrPort {
    /// Creates an AVR GPIO port from its `PINx`, `DDRx`, and `PORTx` data-space addresses.
    #[must_use]
    pub const fn new(pin: u16, ddr: u16, port: u16) -> Self {
        Self {
            pin: AvrReg8::new(pin),
            ddr: AvrReg8::new(ddr),
            port: AvrReg8::new(port),
        }
    }

    /// Returns its `PINx` register.
    #[must_use]
    pub const fn pin_reg(self) -> AvrReg8 { self.pin }

    /// Returns its `DDRx` register.
    #[must_use]
    pub const fn ddr_reg(self) -> AvrReg8 { self.ddr }

    /// Returns its `PORTx` register.
    #[must_use]
    pub const fn port_reg(self) -> AvrReg8 { self.port }

    /// Returns its `PINx`, `DDRx`, and `PORTx` registers, in that order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 3] { [self.pin, self.ddr, self.port] }
}
