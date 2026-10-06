use crate::{AvrUsart, is};

#[cfg(feature = "unsafe_mmio")]
impl AvrUsart {
    unsafe fn configure_8n1(self, clock_hz: u32, baud: u32, enable: u8) {
        let Some((ubrr, double_speed)) = Self::baud_setting(clock_hz, baud) else {
            panic!("AVR USART baud rate is not representable");
        };
        unsafe {
            // Take ownership of this USART configuration.
            // Disables RX, TX and USART interrupts while it is reconfigured.
            self.control_reg().write(0);

            // U2X selects normal (÷16) or double-speed (÷8) asynchronous mode.
            self.status_reg().write(if double_speed { Self::U2X } else { 0 });

            // Updating UBRRnL activates the new divider, so write high first.
            self.baud_high_reg().write((ubrr >> 8) as u8);
            self.baud_low_reg().write(ubrr as u8);

            self.set_frame_8n1();
            self.control_reg().write(enable);
        }
    }
}

/// # Configuration
#[cfg(feature = "unsafe_mmio")]
impl AvrUsart {
    /// Selects asynchronous 8N1 framing: 8 data bits, no parity, and 1 stop bit.
    ///
    /// This does not change the baud rate or enable or disable the receiver or transmitter.
    ///
    /// # Safety
    /// This must describe a USART on the active device and not be concurrently used or configured.
    pub unsafe fn set_frame_8n1(self) {
        let control = self.control_reg();
        unsafe {
            // UCSZn2 = 0; together with UCSZn1:0 = 0b11 this selects 8-bit data.
            control.write(control.read() & !Self::UCSZ2);
            // Asynchronous, no parity, one stop bit, UCSZn1:0 = 0b11.
            self.frame_reg().write(Self::UCSZ1 | Self::UCSZ0);
        }
    }

    /// Configures a transmit-only asynchronous 8N1 connection.
    ///
    /// Selects the closest representable baud rate for `clock_hz` and `baud`,
    /// configures 8 data bits, no parity and 1 stop bit, and enables the transmitter.
    ///
    /// The receiver and USART interrupts are left disabled.
    ///
    /// # Panics
    /// Panics if `clock_hz` or `baud` is zero,
    /// or if the requested rate cannot be represented by the AVR baud generator.
    ///
    /// # Safety
    /// This must describe a USART on the active device
    /// and not be concurrently used or configured.
    pub unsafe fn configure_tx_8n1(self, clock_hz: u32, baud: u32) {
        unsafe { self.configure_8n1(clock_hz, baud, Self::TXEN) };
    }
    /// Configures a full-duplex asynchronous 8N1 connection.
    ///
    /// Selects the closest representable baud rate for `clock_hz` and `baud`,
    /// configures 8 data bits, no parity and 1 stop bit, and enables both
    /// the receiver and transmitter.
    ///
    /// USART interrupts are left disabled.
    ///
    /// # Panics
    /// Panics if `clock_hz` or `baud` is zero,
    /// or if the requested rate cannot be represented by the AVR baud generator.
    ///
    /// # Safety
    /// This must describe a USART on the active device
    /// and not be concurrently used or configured.
    pub unsafe fn configure_rx_tx_8n1(self, clock_hz: u32, baud: u32) {
        unsafe { self.configure_8n1(clock_hz, baud, Self::RXEN | Self::TXEN) };
    }
}

/* private helpers */

#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl AvrUsart {
    // UCSRnA
    pub(super) const RXC: u8 = 1 << 7; // Receive complete.
    pub(super) const UDRE: u8 = 1 << 5; // Data-register empty.
    pub(super) const U2X: u8 = 1 << 1; // Double asynchronous transmission speed.

    // UCSRnB
    pub(super) const RXEN: u8 = 1 << 4; // Receiver enable.
    pub(super) const TXEN: u8 = 1 << 3; // Transmitter enable.
    pub(super) const UCSZ2: u8 = 1 << 2; // Character-size bit 2.

    // UCSRnC
    pub(super) const UCSZ1: u8 = 1 << 2; // Character-size bit 1.
    pub(super) const UCSZ0: u8 = 1 << 1; // Character-size bit 0.

    /// Selects the closest representable asynchronous baud setting.
    ///
    /// Returns `(UBRRn, double_speed)`.
    fn baud_setting(clock_hz: u32, baud: u32) -> Option<(u16, bool)> {
        // Returns: (UBRRn, absolute-error numerator, effective divisor).
        fn candidate(clock_hz: u32, baud: u32, factor: u32) -> Option<(u16, u64, u64)> {
            is! { clock_hz == 0 || baud == 0, return None }
            let (clock, baud, factor) = (clock_hz as u64, baud as u64, factor as u64);

            // baud = clock / (factor × (UBRR + 1))
            // Round UBRR + 1 to the nearest integer.
            let denominator = factor * baud;
            let divider = (clock + denominator / 2) / denominator;

            // UBRR is 12-bit: 0..=4095, therefore divider is 1..=4096.
            is! { divider == 0 || divider > 4096, return None }
            let effective_divisor = factor * divider;

            // The actual baud error is:
            //
            // |clock / effective_divisor - baud|
            // =
            // |clock - baud × effective_divisor| / effective_divisor
            let error = clock.abs_diff(baud * effective_divisor);
            Some(((divider - 1) as u16, error, effective_divisor))
        }
        let normal = candidate(clock_hz, baud, 16);
        let double = candidate(clock_hz, baud, 8);
        match (normal, double) {
            (None, None) => None,
            (Some((ubrr, _, _)), None) => Some((ubrr, false)),
            (None, Some((ubrr, _, _))) => Some((ubrr, true)),
            (
                Some((normal_ubrr, normal_error, normal_divisor)),
                Some((double_ubrr, double_error, double_divisor)),
            ) => {
                // Compare the two rational errors without floating point.
                // Ties prefer normal-speed mode.
                if normal_error * double_divisor <= double_error * normal_divisor {
                    Some((normal_ubrr, false))
                } else {
                    Some((double_ubrr, true))
                }
            }
        }
    }
}
