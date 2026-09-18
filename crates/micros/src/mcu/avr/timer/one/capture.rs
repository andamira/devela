/// # Input capture
#[cfg(feature = "unsafe_mmio")]
impl crate::AvrTimer1 {
    /// Selects rising edges for input capture.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn set_capture_rising_edge(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() | Self::ICES1) };
    }
    /// Selects falling edges for input capture.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn set_capture_falling_edge(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() & !Self::ICES1) };
    }

    /// Enables the input-capture noise canceler.
    ///
    /// The input must remain stable for four consecutive system-clock samples,
    /// introducing four system-clock cycles of capture delay.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn enable_capture_noise_cancel(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() | Self::ICNC1) };
    }
    /// Disables the input-capture noise canceler.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn disable_capture_noise_cancel(self) {
        let reg = self.tccr1b_reg();
        unsafe { reg.write(reg.read() & !Self::ICNC1) };
    }

    /// Returns whether an input-capture event is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn input_capture_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::ICF1 != 0 }
    }
    /// Returns the most recently captured 16-bit counter value.
    ///
    /// Reading `ICR1L` snapshots the corresponding high byte
    /// so both bytes represent the same capture event.
    ///
    /// # Safety
    /// The timer must belong to the active device and be in a mode where `ICR1`
    /// functions as the input-capture register. No concurrent Timer1 16-bit
    /// access may interfere with its shared temporary register.
    #[must_use]
    pub unsafe fn capture_value(self) -> u16 {
        unsafe { Self::read_16_latched(self.icr1l_reg(), self.icr1h_reg()) }
    }
    /// Clears the input-capture flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_input_capture(self) {
        // ICF1 is write-one-to-clear: do not read-modify-write TIFR1.
        unsafe { self.interrupt_flag_reg().write(Self::ICF1) };
    }
}
