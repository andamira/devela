use crate::{AvrReg8, AvrTimer2};

/// # Semantic registers API
#[rustfmt::skip]
impl AvrTimer2 {
    /// Returns the counter value register
    /// ([`TCNT2`](#method.tcnt2_reg)).
    #[must_use]
    pub const fn counter_reg(self) -> AvrReg8 { self.tcnt2_reg() }

    /// Returns the output compare register A
    /// ([`OCR2A`](#method.ocr2a_reg)).
    #[must_use]
    pub const fn compare_a_reg(self) -> AvrReg8 { self.ocr2a_reg() }

    /// Returns the output compare register B
    /// ([`OCR2B`](#method.ocr2b_reg)).
    #[must_use]
    pub const fn compare_b_reg(self) -> AvrReg8 { self.ocr2b_reg() }

    /// Returns the asynchronous status register
    /// ([`ASSR`](#method.assr_reg)).
    #[must_use]
    pub const fn asynchronous_status_reg(self) -> AvrReg8 {
        self.assr_reg()
    }

    /// Returns the interrupt mask register
    /// ([`TIMSK2`](#method.timsk2_reg)).
    #[must_use]
    pub const fn interrupt_mask_reg(self) -> AvrReg8 { self.timsk2_reg() }

    /// Returns the interrupt flag register
    /// ([`TIFR2`](#method.tifr2_reg)).
    #[must_use]
    pub const fn interrupt_flag_reg(self) -> AvrReg8 { self.tifr2_reg() }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrTimer2 {
    /// Returns the Timer/Counter control register A (`TCCR2A`).
    #[must_use]
    pub const fn tccr2a_reg(self) -> AvrReg8 { self.tccr2a }

    /// Returns the Timer/Counter control register B (`TCCR2B`).
    #[must_use]
    pub const fn tccr2b_reg(self) -> AvrReg8 { self.tccr2b }

    /// Returns the Timer/Counter register (`TCNT2`).
    #[must_use]
    pub const fn tcnt2_reg(self) -> AvrReg8 { self.tcnt2 }

    /// Returns the output compare register A (`OCR2A`).
    #[must_use]
    pub const fn ocr2a_reg(self) -> AvrReg8 { self.ocr2a }

    /// Returns the output compare register B (`OCR2B`).
    #[must_use]
    pub const fn ocr2b_reg(self) -> AvrReg8 { self.ocr2b }

    /// Returns the asynchronous status register (`ASSR`).
    #[must_use]
    pub const fn assr_reg(self) -> AvrReg8 {
        self.assr
    }

    /// Returns the interrupt mask register (`TIMSK2`).
    #[must_use]
    pub const fn timsk2_reg(self) -> AvrReg8 { self.timsk2 }

    /// Returns the interrupt flag register (`TIFR2`).
    #[must_use]
    pub const fn tifr2_reg(self) -> AvrReg8 { self.tifr2 }
}
