// devela/sys/hw/mcu/avr/timer/one/compare.rs

/// # Output compare
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl crate::AvrTimer1 {
    /// Returns the output-compare A value.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_a(self) -> u16 {
        unsafe { Self::read_16_unlatched(self.ocr1al_reg(), self.ocr1ah_reg()) }
    }
    /// Sets the output-compare A value.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1
    /// 16-bit access may interfere with its shared temporary register.
    pub unsafe fn set_compare_a(self, value: u16) {
        unsafe { Self::write_16(self.ocr1ah_reg(), self.ocr1al_reg(), value) }
    }
    /// Returns whether an output-compare A match is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_a_match_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::OCF1A != 0 }
    }
    /// Clears the output-compare A match flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_compare_a_match(self) {
        // OCF1A is write-one-to-clear: do not read-modify-write TIFR1.
        unsafe { self.interrupt_flag_reg().write(Self::OCF1A) };
    }

    /// Returns the output-compare B value.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_b(self) -> u16 {
        unsafe { Self::read_16_unlatched(self.ocr1bl_reg(), self.ocr1bh_reg()) }
    }
    /// Sets the output-compare B value.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1
    /// 16-bit access may interfere with its shared temporary register.
    pub unsafe fn set_compare_b(self, value: u16) {
        unsafe { Self::write_16(self.ocr1bh_reg(), self.ocr1bl_reg(), value) }
    }
    /// Returns whether an output-compare B match is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_b_match_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::OCF1B != 0 }
    }
    /// Clears the output-compare B match flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_compare_b_match(self) {
        // OCF1B is write-one-to-clear: do not read-modify-write TIFR1.
        unsafe { self.interrupt_flag_reg().write(Self::OCF1B) };
    }
}
