// devela/sys/hw/mcu/avr/timer/two/pwm.rs

use crate::AvrTimer2;

/// # PWM
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer2 {
    /// Configures synchronous fast PWM with fixed `0xFF` TOP and starts the timer.
    ///
    /// The PWM frequency is:
    ///
    /// `frequency = source_clock / (prescaler × 256)`
    ///
    /// Channels A and B initially remain disconnected.
    ///
    /// # Panics
    /// Panics if `prescaler` is not one of
    /// `1`, `8`, `32`, `64`, `128`, `256`, or `1024`.
    ///
    /// # Safety
    /// The timer must belong to the active device and not be concurrently configured.
    pub unsafe fn configure_fast_pwm(self, prescaler: u16) {
        let Some(clock) = Self::prescaler_bits(prescaler) else {
            panic!("AVR Timer2 prescaler is not supported");
        };
        unsafe {
            self.interrupt_mask_reg().write(0);
            self.asynchronous_status_reg().write(0);
            // Establish complete synchronous state while stopped.
            self.tccr2b_reg().write(0);
            self.tccr2a_reg().write(0);
            self.set_counter(0);
            self.set_compare_a(0);
            self.set_compare_b(0);
            // Start without pending Timer2 events.
            self.interrupt_flag_reg().write(Self::EVENT_FLAGS);
            // WGM22:0 = 0b011: fast PWM, TOP = 0xFF.
            self.tccr2a_reg().write(Self::WGM21 | Self::WGM20);
            self.tccr2b_reg().write(clock);
        }
    }

    /// Enables non-inverting PWM output on channel A.
    ///
    /// # Safety
    /// Timer2 must belong to the active device
    /// and already be configured in a compatible PWM mode.
    pub unsafe fn enable_pwm_a_non_inverting(self) {
        let reg = self.tccr2a_reg();
        let value = unsafe { reg.read() };
        unsafe {
            reg.write((value & !(Self::COM2A1 | Self::COM2A0)) | Self::COM2A1);
        }
    }
    /// Disconnects channel A from its PWM output pin.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn disable_pwm_a_output(self) {
        let reg = self.tccr2a_reg();
        unsafe { reg.write(reg.read() & !(Self::COM2A1 | Self::COM2A0)) };
    }

    /// Enables non-inverting PWM output on channel B.
    ///
    /// # Safety
    /// Timer2 must belong to the active device
    /// and already be configured in a compatible PWM mode.
    pub unsafe fn enable_pwm_b_non_inverting(self) {
        let reg = self.tccr2a_reg();
        let value = unsafe { reg.read() };
        unsafe {
            reg.write((value & !(Self::COM2B1 | Self::COM2B0)) | Self::COM2B1);
        }
    }
    /// Disconnects channel B from its PWM output pin.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn disable_pwm_b_output(self) {
        let reg = self.tccr2a_reg();
        unsafe { reg.write(reg.read() & !(Self::COM2B1 | Self::COM2B0)) };
    }
}
