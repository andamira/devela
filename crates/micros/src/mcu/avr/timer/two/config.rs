/// # Configuration
#[cfg(feature = "unsafe_mmio")]
impl crate::AvrTimer2 {
    /// Configures normal synchronous counting and starts the timer.
    ///
    /// The counter runs from `0x00` through `0xFF` and wraps to zero.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of
    /// `1`, `8`, `32`, `64`, `128`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    pub unsafe fn configure_normal(self, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer2 prescaler is not supported");
        };
        unsafe {
            // Disable Timer2 interrupts before changing its clock source.
            self.interrupt_mask_reg().write(0);
            self.asynchronous_status_reg().write(0); // Use synchronous clkI/O
            // Establish complete Timer2 state after changing AS2.
            self.tccr2b_reg().write(0);
            self.tccr2a_reg().write(0);
            self.set_counter(0);
            self.set_compare_a(0);
            self.set_compare_b(0);
            // Start without pending Timer2 events.
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);
            self.tccr2b_reg().write(clock);
        }
    }
    /// Configures synchronous CTC mode with `OCR2A` as TOP and starts the timer.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of
    /// `1`, `8`, `32`, `64`, `128`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    pub unsafe fn configure_ctc(self, top: u8, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer2 prescaler is not supported");
        };
        unsafe {
            self.interrupt_mask_reg().write(0);
            self.asynchronous_status_reg().write(0);

            self.tccr2b_reg().write(0);
            self.tccr2a_reg().write(Self::WGM21);
            self.set_counter(0);
            self.set_compare_a(top);
            self.set_compare_b(0);

            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);
            self.tccr2b_reg().write(clock);
        }
    }

    /// Configures normal asynchronous counting from the Timer2 crystal oscillator.
    ///
    /// Timer2 uses the clock from `TOSC1/TOSC2`, independently of `clkI/O`.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of
    /// `1`, `8`, `32`, `64`, `128`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    /// A running asynchronous clock must be available on the Timer2 oscillator pins.
    pub unsafe fn configure_async_normal(self, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer2 prescaler is not supported");
        };
        unsafe {
            // Disable Timer2 interrupts before changing its clock source.
            self.interrupt_mask_reg().write(0);
            // Select the TOSC crystal oscillator (EXCLK = 0, AS2 = 1).
            self.asynchronous_status_reg().write(Self::AS2);

            // Establish complete asynchronous state while stopped.
            self.tccr2a_reg().write(0);
            self.tccr2b_reg().write(0);
            self.counter_reg().write(0);
            self.compare_a_reg().write(0);
            self.compare_b_reg().write(0);

            self.wait_for_asynchronous_update();
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            // Start only after the rest of the state is synchronized.
            self.tccr2b_reg().write(clock);
            self.wait_for_asynchronous_update();
        }
    }
    /// Configures asynchronous CTC mode with `OCR2A` as TOP.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of
    /// `1`, `8`, `32`, `64`, `128`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    /// A running asynchronous clock must be available on the Timer2 oscillator pins.
    pub unsafe fn configure_async_ctc(self, top: u8, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer2 prescaler is not supported");
        };
        unsafe {
            self.interrupt_mask_reg().write(0);
            self.asynchronous_status_reg().write(Self::AS2);

            self.tccr2a_reg().write(Self::WGM21);
            self.tccr2b_reg().write(0);
            self.counter_reg().write(0);
            self.compare_a_reg().write(top);
            self.compare_b_reg().write(0);

            self.wait_for_asynchronous_update();
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            self.tccr2b_reg().write(clock);
            self.wait_for_asynchronous_update();
        }
    }
}
