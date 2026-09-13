// devela/sys/hw/mcu/avr/timer/two/interrupt.rs

/// # Interrupt
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl crate::AvrTimer2 {
    /// Enables the output-compare A interrupt.
    ///
    /// A compare-A match can request an interrupt when global interrupts
    /// are also enabled.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn enable_compare_a_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() | Self::OCIE2A) };
    }
    /// Disables the output-compare A interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_compare_a_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::OCIE2A) };
    }

    /// Enables the output-compare B interrupt.
    ///
    /// A compare-B match can request an interrupt when global interrupts
    /// are also enabled.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn enable_compare_b_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() | Self::OCIE2B) };
    }
    /// Disables the output-compare B interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_compare_b_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::OCIE2B) };
    }

    /// Enables the overflow interrupt.
    ///
    /// A timer overflow can request an interrupt when global interrupts
    /// are also enabled.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn enable_overflow_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() | Self::TOIE2) };
    }
    /// Disables the overflow interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_overflow_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::TOIE2) };
    }
}
