use crate::{AvrReg8, AvrUsart};

/// # Semantic registers API
#[rustfmt::skip]
impl AvrUsart {
    /// Returns the status and operating-flags register
    /// ([`UCSRnA`](#method.ucsra_reg)).
    #[must_use]
    pub const fn status_reg(self) -> AvrReg8 { self.ucsra_reg() }

    /// Returns the transmitter, receiver, and interrupt control register
    /// ([`UCSRnB`](#method.ucsrb_reg)).
    #[must_use]
    pub const fn control_reg(self) -> AvrReg8 { self.ucsrb_reg() }

    /// Returns the operating-mode and frame-format control register
    /// ([`UCSRnC`](#method.ucsrc_reg)).
    #[must_use]
    pub const fn frame_reg(self) -> AvrReg8 { self.ucsrc_reg() }

    /// Returns the low baud-rate register
    /// ([`UBRRnL`](#method.ubrrl_reg)).
    #[must_use]
    pub const fn baud_low_reg(self) -> AvrReg8 { self.ubrrl_reg() }

    /// Returns the high baud-rate register
    /// ([`UBRRnH`](#method.ubrrh_reg)).
    #[must_use]
    pub const fn baud_high_reg(self) -> AvrReg8 { self.ubrrh_reg() }

    /// Returns the transmit and receive data register
    /// ([`UDRn`](#method.udr_reg)).
    #[must_use]
    pub const fn data_reg(self) -> AvrReg8 { self.udr_reg() }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrUsart {
    /// Returns the USART control and status register A (`UCSRnA`).
    #[must_use]
    pub const fn ucsra_reg(self) -> AvrReg8 { self.ucsra }

    /// Returns the USART control and status register B (`UCSRnB`).
    #[must_use]
    pub const fn ucsrb_reg(self) -> AvrReg8 { self.ucsrb }

    /// Returns the USART control and status register C (`UCSRnC`).
    #[must_use]
    pub const fn ucsrc_reg(self) -> AvrReg8 { self.ucsrc }

    /// Returns the low USART baud-rate register (`UBRRnL`).
    #[must_use]
    pub const fn ubrrl_reg(self) -> AvrReg8 { self.ubrrl }

    /// Returns the high USART baud-rate register (`UBRRnH`).
    #[must_use]
    pub const fn ubrrh_reg(self) -> AvrReg8 { self.ubrrh }

    /// Returns the USART data register (`UDRn`).
    #[must_use]
    pub const fn udr_reg(self) -> AvrReg8 { self.udr }
}
