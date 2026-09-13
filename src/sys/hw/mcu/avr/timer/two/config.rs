// devela/sys/hw/mcu/avr/timer/two/config.rs

use crate::AvrTimer2;

/// # Configuration
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer2 {
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

            // Use synchronous clkI/O rather than the asynchronous oscillator.
            self.asynchronous_status_reg().write(0);

            // Stop before taking ownership of the configuration.
            self.tccr2b_reg().write(0);

            // WGM22:0 = 0b000: normal mode; OC2A/B disconnected.
            self.tccr2a_reg().write(0);

            // Start from BOTTOM.
            self.set_counter(0);

            // Start without pending Timer2 events.
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            // WGM22 remains zero; CS22:0 selects the timer clock.
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

            // WGM22:0 = 0b010: CTC with OCR2A as TOP; OC2A/B disconnected.
            self.tccr2a_reg().write(Self::WGM21);

            self.set_counter(0);
            self.set_compare_a(top);

            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);

            self.tccr2b_reg().write(clock);
        }
    }
}
