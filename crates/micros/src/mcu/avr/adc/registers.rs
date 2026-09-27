//
//! Defines [`AvrAdc`].
//

use crate::{AvrAdc, AvrReg8};

/// # Semantic registers API
#[rustfmt::skip]
impl AvrAdc {
    /// Returns the low conversion-result register
    /// ([`ADCL`](#method.adcl_reg)).
    #[must_use]
    pub const fn result_low_reg(self) -> AvrReg8 { self.adcl_reg() }

    /// Returns the high conversion-result register
    /// ([`ADCH`](#method.adch_reg)).
    #[must_use]
    pub const fn result_high_reg(self) -> AvrReg8 { self.adch_reg() }

    /// Returns the main control and status register
    /// ([`ADCSRA`](#method.adcsra_reg)).
    #[must_use]
    pub const fn control_status_reg(self) -> AvrReg8 { self.adcsra_reg() }

    /// Returns the auxiliary control and auto-trigger register
    /// ([`ADCSRB`](#method.adcsrb_reg)).
    #[must_use]
    pub const fn auxiliary_control_reg(self) -> AvrReg8 { self.adcsrb_reg() }

    /// Returns the reference, result-adjustment, and input multiplexer register
    /// ([`ADMUX`](#method.admux_reg)).
    #[must_use]
    pub const fn mux_reg(self) -> AvrReg8 { self.admux_reg() }

    /// Returns the digital-input disable register
    /// ([`DIDR0`](#method.didr0_reg)).
    #[must_use]
    pub const fn digital_input_disable_reg(self) -> AvrReg8 { self.didr0_reg() }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrAdc {
    /// Returns the ADC data register low byte (`ADCL`).
    #[must_use]
    pub const fn adcl_reg(self) -> AvrReg8 { self.adcl }

    /// Returns the ADC data register high byte (`ADCH`).
    #[must_use]
    pub const fn adch_reg(self) -> AvrReg8 { self.adch }

    /// Returns the ADC control and status register A (`ADCSRA`).
    #[must_use]
    pub const fn adcsra_reg(self) -> AvrReg8 { self.adcsra }

    /// Returns the ADC control and status register B (`ADCSRB`).
    #[must_use]
    pub const fn adcsrb_reg(self) -> AvrReg8 { self.adcsrb }

    /// Returns the ADC multiplexer selection register (`ADMUX`).
    #[must_use]
    pub const fn admux_reg(self) -> AvrReg8 { self.admux }

    /// Returns the digital input disable register 0 (`DIDR0`).
    #[must_use]
    pub const fn didr0_reg(self) -> AvrReg8 { self.didr0 }
}
