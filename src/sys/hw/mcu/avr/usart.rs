// devela/sys/hw/mcu/avr/usart.rs
//
//! Defines [`AvrUsart`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw io)]
/// An AVR USART described by its control, baud-rate, and data registers.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr", struct AvrUsart),
    test_size_of(AvrUsart = 12|96; niche !Option),
}]
/// Values can safely be copied and inspected. Operations that access the
/// described registers are unsafe because the addresses must correspond to
/// the active device and access must respect the peripheral's hardware state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrUsart {
    ucsra: AvrReg8,
    ucsrb: AvrReg8,
    ucsrc: AvrReg8,
    ubrrl: AvrReg8,
    ubrrh: AvrReg8,
    udr: AvrReg8,
}

#[rustfmt::skip]
impl AvrUsart {
    /// Creates an AVR USART from its register data-space addresses.
    #[must_use]
    pub const fn new(
        ucsra: u16,
        ucsrb: u16,
        ucsrc: u16,
        ubrrl: u16,
        ubrrh: u16,
        udr: u16,
    ) -> Self {
        Self {
            ucsra: AvrReg8::new(ucsra),
            ucsrb: AvrReg8::new(ucsrb),
            ucsrc: AvrReg8::new(ucsrc),
            ubrrl: AvrReg8::new(ubrrl),
            ubrrh: AvrReg8::new(ubrrh),
            udr: AvrReg8::new(udr),
        }
    }
    /// Returns all six registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 6] {
        [
            self.ucsra,
            self.ucsrb,
            self.ucsrc,
            self.ubrrl,
            self.ubrrh,
            self.udr,
        ]
    }
}

/// # Semantic registers API
#[rustfmt::skip]
impl AvrUsart {
    /// Returns the status and operating-flags register
    /// ([`UCSRnA`](#method.ucsra_reg)).
    #[must_use]
    pub const fn status_reg(self) -> AvrReg8 { self.ucsra_reg() }

    /// Returns the transmitter, receiver, and interrupt control register
    /// ([`UCSRnB`](#method.ucsrb_reg)).
    #[must_use]
    pub const fn control_reg(self) -> AvrReg8 { self.ucsrb_reg() }

    /// Returns the operating-mode and frame-format control register
    /// ([`UCSRnC`](#method.ucsrc_reg)).
    #[must_use]
    pub const fn frame_reg(self) -> AvrReg8 { self.ucsrc_reg() }

    /// Returns the low baud-rate register
    /// ([`UBRRnL`](#method.ubrrl_reg)).
    #[must_use]
    pub const fn baud_low_reg(self) -> AvrReg8 { self.ubrrl_reg() }

    /// Returns the high baud-rate register
    /// ([`UBRRnH`](#method.ubrrh_reg)).
    #[must_use]
    pub const fn baud_high_reg(self) -> AvrReg8 { self.ubrrh_reg() }

    /// Returns the transmit and receive data register
    /// ([`UDRn`](#method.udr_reg)).
    #[must_use]
    pub const fn data_reg(self) -> AvrReg8 { self.udr_reg() }
}

/// # Datasheet registers API
#[rustfmt::skip]
impl AvrUsart {
    /// Returns the USART control and status register A (`UCSRnA`).
    #[must_use]
    pub const fn ucsra_reg(self) -> AvrReg8 { self.ucsra }

    /// Returns the USART control and status register B (`UCSRnB`).
    #[must_use]
    pub const fn ucsrb_reg(self) -> AvrReg8 { self.ucsrb }

    /// Returns the USART control and status register C (`UCSRnC`).
    #[must_use]
    pub const fn ucsrc_reg(self) -> AvrReg8 { self.ucsrc }

    /// Returns the low USART baud-rate register (`UBRRnL`).
    #[must_use]
    pub const fn ubrrl_reg(self) -> AvrReg8 { self.ubrrl }

    /// Returns the high USART baud-rate register (`UBRRnH`).
    #[must_use]
    pub const fn ubrrh_reg(self) -> AvrReg8 { self.ubrrh }

    /// Returns the USART data register (`UDRn`).
    #[must_use]
    pub const fn udr_reg(self) -> AvrReg8 { self.udr }
}

/// # Operational API
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrUsart {
    // UCSRnA
    const U2X: u8 = 1 << 1;
    const UDRE: u8 = 1 << 5;
    // UCSRnB
    const UCSZ2: u8 = 1 << 2;
    const TXEN: u8 = 1 << 3;
    // UCSRnC
    const UCSZ0: u8 = 1 << 1;
    const UCSZ1: u8 = 1 << 2;

    /* configuration */

    /// Returns whether the transmit data register can accept another byte.
    ///
    /// # Safety
    /// This must describe a USART on the active device.
    #[must_use]
    pub unsafe fn tx_ready(self) -> bool {
        unsafe { self.status_reg().read() & Self::UDRE != 0 }
    }
    /// Enables the transmitter, preserving the other control bits.
    ///
    /// # Safety
    /// This must describe a USART on the active device and not be concurrently used or configured.
    pub unsafe fn enable_tx(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() | Self::TXEN) };
    }
    /// Disables the transmitter, preserving the other control bits.
    ///
    /// # Safety
    /// This must describe a USART on the active device and not be concurrently used or configured.
    pub unsafe fn disable_tx(self) {
        let reg = self.control_reg();
        unsafe { reg.write(reg.read() & !Self::TXEN) };
    }

    /// Selects asynchronous 8N1 framing: 8 data bits, no parity, and 1 stop bit.
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
    /// # Panics
    /// Panics if `clock_hz` or `baud` is zero,
    /// or if the requested rate cannot be represented by the AVR baud generator.
    ///
    /// # Safety
    /// This must describe a USART on the active device
    /// and not be concurrently used or configured.
    pub unsafe fn configure_tx_8n1(self, clock_hz: u32, baud: u32) {
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
            self.enable_tx();
        }
    }

    /* transmission */

    /// Attempts to queue one byte for transmission without waiting.
    ///
    /// Returns `true` if the byte was accepted,
    /// or `false` if the transmit data register was still busy.
    ///
    /// # Safety
    /// This must describe a USART on the active device and not be concurrently written.
    #[must_use]
    pub unsafe fn try_write_byte(self, byte: u8) -> bool {
        if unsafe { self.tx_ready() } {
            unsafe { self.data_reg().write(byte) };
            true
        } else {
            false
        }
    }
    /// Queues one byte for transmission, waiting until the USART can accept it.
    ///
    /// Waits for space in the transmit data register;
    /// it does not wait for the byte to finish shifting onto the wire.
    ///
    /// # Safety
    /// This must describe a USART on the active device and not be concurrently written.
    pub unsafe fn write_byte_blocking(self, byte: u8) {
        while !unsafe { self.tx_ready() } {}
        unsafe { self.data_reg().write(byte) };
    }
    /// Queues all bytes for transmission, waiting for space as needed.
    ///
    /// Returns after the final byte has been accepted by the USART,
    /// which may be before that byte has finished transmitting on the wire.
    ///
    /// # Safety
    /// This must describe a USART on the active device and not be concurrently written.
    pub unsafe fn write_bytes_blocking(self, bytes: &[u8]) {
        for &byte in bytes {
            unsafe { self.write_byte_blocking(byte) };
        }
    }
}

/* private helpers */

#[allow(dead_code)]
impl AvrUsart {
    /// Selects the closest representable asynchronous baud setting.
    ///
    /// Returns `(UBRRn, double_speed)`.
    fn baud_setting(clock_hz: u32, baud: u32) -> Option<(u16, bool)> {
        use crate::is;
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
