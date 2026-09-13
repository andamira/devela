// devela/sys/hw/mcu/avr/timer/one/registers.rs

use crate::{AvrReg8, AvrTimer1};

/// # Semantic registers API
#[rustfmt::skip]
impl AvrTimer1 {
    /// Returns the interrupt mask register
    /// ([`TIMSK1`][Self::timsk1_reg]).
    #[must_use]
    pub const fn interrupt_mask_reg(self) -> AvrReg8 {
        self.timsk1_reg()
    }

    /// Returns the interrupt flag register
    /// ([`TIFR1`][Self::tifr1_reg]).
    #[must_use]
    pub const fn interrupt_flag_reg(self) -> AvrReg8 {
        self.tifr1_reg()
    }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrTimer1 {
    /// Returns the Timer/Counter1 control register A (`TCCR1A`).
    #[must_use]
    pub const fn tccr1a_reg(self) -> AvrReg8 { self.tccr1a }

    /// Returns the Timer/Counter1 control register B (`TCCR1B`).
    #[must_use]
    pub const fn tccr1b_reg(self) -> AvrReg8 { self.tccr1b }

    /// Returns the Timer/Counter1 control register C (`TCCR1C`).
    #[must_use]
    pub const fn tccr1c_reg(self) -> AvrReg8 { self.tccr1c }

    /// Returns the low Timer/Counter1 counter register (`TCNT1L`).
    #[must_use]
    pub const fn tcnt1l_reg(self) -> AvrReg8 { self.tcnt1l }

    /// Returns the high Timer/Counter1 counter register (`TCNT1H`).
    #[must_use]
    pub const fn tcnt1h_reg(self) -> AvrReg8 { self.tcnt1h }

    /// Returns the low input capture register (`ICR1L`).
    #[must_use]
    pub const fn icr1l_reg(self) -> AvrReg8 { self.icr1l }

    /// Returns the high input capture register (`ICR1H`).
    #[must_use]
    pub const fn icr1h_reg(self) -> AvrReg8 { self.icr1h }

    /// Returns the low output compare register A (`OCR1AL`).
    #[must_use]
    pub const fn ocr1al_reg(self) -> AvrReg8 { self.ocr1al }

    /// Returns the high output compare register A (`OCR1AH`).
    #[must_use]
    pub const fn ocr1ah_reg(self) -> AvrReg8 { self.ocr1ah }

    /// Returns the low output compare register B (`OCR1BL`).
    #[must_use]
    pub const fn ocr1bl_reg(self) -> AvrReg8 { self.ocr1bl }

    /// Returns the high output compare register B (`OCR1BH`).
    #[must_use]
    pub const fn ocr1bh_reg(self) -> AvrReg8 { self.ocr1bh }

    /// Returns the interrupt mask register (`TIMSK1`).
    #[must_use]
    pub const fn timsk1_reg(self) -> AvrReg8 { self.timsk1 }

    /// Returns the interrupt flag register (`TIFR1`).
    #[must_use]
    pub const fn tifr1_reg(self) -> AvrReg8 { self.tifr1 }
}
