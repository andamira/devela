// devela/sys/hw/mcu/avr/timer/one/interrupt.rs

/// # Interrupt
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl crate::AvrTimer1 {
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
        unsafe { reg.write(reg.read() | Self::OCIE1A) };
    }
    /// Disables the output-compare A interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_compare_a_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::OCIE1A) };
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
        unsafe { reg.write(reg.read() | Self::OCIE1B) };
    }
    /// Disables the output-compare B interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_compare_b_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::OCIE1B) };
    }

    /// Enables the input-capture interrupt.
    ///
    /// An input-capture event can request an interrupt when global interrupts
    /// are also enabled.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn enable_input_capture_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() | Self::ICIE1) };
    }
    /// Disables the input-capture interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_input_capture_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::ICIE1) };
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
        unsafe { reg.write(reg.read() | Self::TOIE1) };
    }
    /// Disables the overflow interrupt.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its interrupt mask must not be concurrently modified.
    pub unsafe fn disable_overflow_interrupt(self) {
        let reg = self.interrupt_mask_reg();
        unsafe { reg.write(reg.read() & !Self::TOIE1) };
    }
}
