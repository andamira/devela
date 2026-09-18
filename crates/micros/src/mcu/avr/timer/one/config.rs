/// # Configuration
#[cfg(feature = "unsafe_mmio")]
impl crate::AvrTimer1 {
    /// Configures normal mode and starts the timer.
    ///
    /// The counter runs from `0x0000` through `0xFFFF` and then wraps to zero.
    /// Timer1 interrupts are initially disabled, output-compare pins remain
    /// disconnected, and both compare values are reset to zero.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of `1`, `8`, `64`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    /// No interrupt may access Timer1's 16-bit registers during the paired counter write.
    pub unsafe fn configure_normal(self, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer1 prescaler is not supported");
        };
        unsafe {
            // Stop before taking ownership of the configuration.
            self.tccr1b_reg().write(0);

            // WGM13:0 = 0b0000: normal mode; OC1A/B disconnected.
            self.tccr1a_reg().write(0);
            self.tccr1c_reg().write(0);

            // Start with Timer1 interrupts disabled.
            self.interrupt_mask_reg().write(0);

            // Start counting from BOTTOM and reset both compare channels.
            self.set_counter(0);
            self.set_compare_a(0);
            self.set_compare_b(0);

            // Start without pending Timer1 events.
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            // WGM13:0 remains 0b0000; CS12:0 selects the timer clock.
            self.tccr1b_reg().write(clock);
        }
    }
    /// Configures CTC mode with `OCR1A` as TOP and starts the timer.
    ///
    /// The counter advances from zero through `top`, inclusive,
    /// so one compare interval contains `top + 1` timer ticks:
    ///
    /// `period = prescaler × (top + 1) / source_clock`
    ///
    /// Timer1 interrupts are initially disabled
    /// and output-compare B is reset to zero.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of `1`, `8`, `64`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently
    /// configured. No interrupt may access Timer1's 16-bit registers
    /// during the paired writes performed by this operation.
    pub unsafe fn configure_ctc(self, top: u16, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer1 prescaler is not supported");
        };
        unsafe {
            // Stop before taking ownership of the configuration.
            self.tccr1b_reg().write(0);

            // WGM13:0 = 0b0100: CTC with OCR1A as TOP; OC1A/B disconnected.
            self.tccr1a_reg().write(0);
            self.tccr1c_reg().write(0);

            // Start with Timer1 interrupts disabled.
            self.interrupt_mask_reg().write(0);

            // Start counting from BOTTOM, configure TOP,
            // and reset the unused compare channel.
            self.set_counter(0);
            self.set_compare_a(top);
            self.set_compare_b(0);

            // Start without pending Timer1 events.
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            // WGM12 selects CTC; CS12:0 selects the timer clock.
            self.tccr1b_reg().write(Self::WGM12 | clock);
        }
    }
}
