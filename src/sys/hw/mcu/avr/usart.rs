// devela/sys/hw/mcu/avr/usart.rs
//
//! Defines [`AvrUsart`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw io)]
/// An AVR USART described by its control, baud-rate, and data registers.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr", struct AvrUsart),
    test_size_of(AvrUsart = 12|96; niche !Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrUsart {
    ucsra: AvrReg8,
    ucsrb: AvrReg8,
    ucsrc: AvrReg8,
    ubrrl: AvrReg8,
    ubrrh: AvrReg8,
    udr: AvrReg8,
}

#[rustfmt::skip]
impl AvrUsart {
    /// Creates an AVR USART from its register data-space addresses.
    #[must_use]
    pub const fn new(
        ucsra: u16,
        ucsrb: u16,
        ucsrc: u16,
        ubrrl: u16,
        ubrrh: u16,
        udr: u16,
    ) -> Self {
        Self {
            ucsra: AvrReg8::new(ucsra),
            ucsrb: AvrReg8::new(ucsrb),
            ucsrc: AvrReg8::new(ucsrc),
            ubrrl: AvrReg8::new(ubrrl),
            ubrrh: AvrReg8::new(ubrrh),
            udr: AvrReg8::new(udr),
        }
    }

    /// Returns its USART control and status register A (`UCSRnA`).
    #[must_use]
    pub const fn ucsra_reg(self) -> AvrReg8 { self.ucsra }

    /// Returns its USART control and status register B (`UCSRnB`).
    #[must_use]
    pub const fn ucsrb_reg(self) -> AvrReg8 { self.ucsrb }

    /// Returns its USART control and status register C (`UCSRnC`).
    #[must_use]
    pub const fn ucsrc_reg(self) -> AvrReg8 { self.ucsrc }

    /// Returns the low USART baud-rate register (`UBRRnL`).
    #[must_use]
    pub const fn ubrrl_reg(self) -> AvrReg8 { self.ubrrl }

    /// Returns the high USART baud-rate register (`UBRRnH`).
    #[must_use]
    pub const fn ubrrh_reg(self) -> AvrReg8 { self.ubrrh }

    /// Returns its USART data register (`UDRn`).
    #[must_use]
    pub const fn udr_reg(self) -> AvrReg8 { self.udr }

    /// Returns all six registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 6] {
        [
            self.ucsra,
            self.ucsrb,
            self.ucsrc,
            self.ubrrl,
            self.ubrrh,
            self.udr,
        ]
    }
}
