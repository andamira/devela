/// # PWM
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl crate::AvrTimer1 {
    /// Configures fast PWM with `ICR1` as TOP and starts the timer.
    ///
    /// Timer1 counts upward from BOTTOM through `top` and then begins again
    /// from BOTTOM. The PWM frequency is:
    ///
    /// `frequency = source_clock / (prescaler × (top + 1))`
    ///
    /// Output channels A and B initially remain disconnected. Their compare
    /// values are reset to BOTTOM and can be configured before connecting
    /// either PWM output.
    ///
    /// Using `ICR1` as TOP makes the input-capture function unavailable while
    /// this mode is active.
    ///
    /// # Panics
    /// Panics if `top` is less than 3, or if `prescaler` is not one of
    /// `1`, `8`, `64`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently
    /// configured. No interrupt may access Timer1's 16-bit registers during
    /// the paired writes performed by this operation.
    pub unsafe fn configure_fast_pwm(self, top: u16, prescaler: u16) {
        assert!(top >= 3, "AVR Timer1 fast-PWM TOP must be at least 3");
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer1 prescaler is not supported");
        };
        unsafe {
            // Stop before taking ownership of the configuration.
            self.tccr1b_reg().write(0);

            // Begin from normal mode with OC1A/B disconnected.
            //
            // Initializing OCR1A/B before entering PWM mode avoids their
            // double-buffered PWM update rules during setup.
            self.tccr1a_reg().write(0);
            self.tccr1c_reg().write(0);

            // Start with Timer1 interrupts disabled.
            self.interrupt_mask_reg().write(0);

            // Start from BOTTOM and configure the PWM range.
            self.set_counter(0);
            Self::write_16(self.icr1h_reg(), self.icr1l_reg(), top);
            self.set_compare_a(0);
            self.set_compare_b(0);

            // Start without pending Timer1 events.
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            // WGM13:0 = 0b1110: fast PWM with ICR1 as TOP.
            self.tccr1a_reg().write(Self::WGM11);
            self.tccr1b_reg().write(Self::WGM13 | Self::WGM12 | clock);
        }
    }

    /// Enables non-inverting PWM output on channel A.
    ///
    /// In non-inverting fast PWM mode, `OC1A` is set at BOTTOM
    /// and cleared when the counter matches `OCR1A`.
    ///
    /// The corresponding hardware pin must also be configured as an output.
    ///
    /// # Safety
    /// The timer must belong to the active device and already be configured in a
    /// compatible PWM mode. Its control register must not be concurrently modified.
    pub unsafe fn enable_pwm_a_non_inverting(self) {
        let reg = self.tccr1a_reg();
        let value = unsafe { reg.read() };
        unsafe {
            reg.write((value & !(Self::COM1A1 | Self::COM1A0)) | Self::COM1A1);
        }
    }
    /// Disconnects channel A from its PWM output pin.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn disable_pwm_a_output(self) {
        let reg = self.tccr1a_reg();
        unsafe { reg.write(reg.read() & !(Self::COM1A1 | Self::COM1A0)) };
    }

    /// Enables non-inverting PWM output on channel B.
    ///
    /// In non-inverting fast PWM mode, `OC1B` is set at BOTTOM
    /// and cleared when the counter matches `OCR1B`.
    ///
    /// The corresponding hardware pin must also be configured as an output.
    ///
    /// # Safety
    /// The timer must belong to the active device and already be configured in a
    /// compatible PWM mode. Its control register must not be concurrently modified.
    pub unsafe fn enable_pwm_b_non_inverting(self) {
        let reg = self.tccr1a_reg();
        let value = unsafe { reg.read() };
        unsafe {
            reg.write((value & !(Self::COM1B1 | Self::COM1B0)) | Self::COM1B1);
        }
    }
    /// Disconnects channel B from its PWM output pin.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its control register must not be concurrently modified.
    pub unsafe fn disable_pwm_b_output(self) {
        let reg = self.tccr1a_reg();
        unsafe { reg.write(reg.read() & !(Self::COM1B1 | Self::COM1B0)) };
    }
}
