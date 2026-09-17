//
//! Defines [`Esp32C3Uart`].
//

#[cfg(doc)]
use crate::McuEsp32C3;
use crate::{__cfg_item_unsafe_show, EspReg32, is, macro_apply};

#[doc = crate::_tags!(hw io)]
/// An ESP32-C3 UART controller.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32", struct Esp32C3Uart),
    test_size_of(Esp32C3Uart = 4|32; niche !Option),
}]
/// The ESP32-C3 provides two UART controllers with 128-byte transmit
/// and receive FIFOs. This type models their C3-specific register layout
/// and provides asynchronous 8N1 configuration together with byte-oriented
/// polling for reception and transmission.
///
/// devela currently exposes [`McuEsp32C3::UART0`]. UART0 has direct IO-MUX
/// routing through GPIO20 (`U0RXD`) and GPIO21 (`U0TXD`), configured by
/// [`McuEsp32C3::prepare_uart0_default`].
///
/// UART0 is also used by the ESP32-C3 ROM for boot messages and serial
/// download. A terminal attached to these pins may therefore display ROM
/// output and binary flashing traffic before the application takes control.
///
/// Alternative pin routing and UART1 can use the GPIO matrix and are not
/// configured by the current convenience API.
///
/// See the [ESP32-C3 Technical Reference Manual] for the complete UART
/// register and routing model.
///
/// [ESP32-C3 Technical Reference Manual]:
/// https://documentation.espressif.com/esp32-c3_technical_reference_manual_en.pdf
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Esp32C3Uart(u32);

/// # Registers
#[rustfmt::skip]
impl Esp32C3Uart {
    /// Creates a UART controller from its register base address.
    #[must_use]
    pub const fn new(base: u32) -> Self { Self(base) }

    /// Returns its register base address.
    #[must_use]
    pub const fn base_addr(self) -> u32 { self.0 }

    /// Returns the FIFO data register.
    #[must_use]
    pub const fn fifo_reg(self) -> EspReg32 { EspReg32::new(self.0) }

    /// Returns the baud-rate divider register.
    #[must_use]
    pub const fn clock_div_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x14) }

    /// Returns the FIFO and line-status register.
    #[must_use]
    pub const fn status_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x1c) }

    /// Returns the primary configuration register.
    #[must_use]
    pub const fn config_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x20) }

    /// Returns the secondary configuration register.
    #[must_use]
    pub const fn config1_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x24) }

    /// Returns the source-clock configuration register.
    #[must_use]
    pub const fn clock_config_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x78) }
}

/* private helpers */
#[allow(dead_code)]
impl Esp32C3Uart {
    const FIFO_CAPACITY: u32 = 128;

    // STATUS
    const RXFIFO_COUNT_MASK: u32 = 0x3ff;
    const TXFIFO_COUNT_MASK: u32 = 0x3ff << 16;
    const TXFIFO_COUNT_SHIFT: u32 = 16;

    // CONF0
    const PARITY_ENABLE: u32 = 1 << 1;
    const DATA_BITS_MASK: u32 = 0b11 << 2;
    const DATA_BITS_8: u32 = 0b11 << 2;
    const STOP_BITS_MASK: u32 = 0b11 << 4;
    const STOP_BITS_1: u32 = 0b01 << 4;
    const TX_FLOW_ENABLE: u32 = 1 << 15;
    const RXFIFO_RESET: u32 = 1 << 17;
    const TXFIFO_RESET: u32 = 1 << 18;

    // CONF1
    const RX_FLOW_ENABLE: u32 = 1 << 20;

    // CLK_CONF
    const SCLK_DIV_B_MASK: u32 = 0x3f;
    const SCLK_DIV_A_MASK: u32 = 0x3f << 6;
    const SCLK_DIV_NUM_MASK: u32 = 0xff << 12;
    const SCLK_DIV_NUM_SHIFT: u32 = 12;
    const SCLK_SEL_MASK: u32 = 0b11 << 20;
    const SCLK_SEL_XTAL: u32 = 0b11 << 20;
    const SCLK_ENABLE: u32 = 1 << 22;
    const TX_SCLK_ENABLE: u32 = 1 << 24;
    const RX_SCLK_ENABLE: u32 = 1 << 25;

    /// Returns `(integer, fraction, source_divider_minus_one)`.
    fn baud_setting(source_hz: u32, baud: u32) -> Option<(u16, u8, u8)> {
        is! { source_hz == 0 || baud == 0, return None }
        let (source, baud) = (source_hz as u64, baud as u64);

        // Keep the 12-bit UART divider representable.
        let denominator = 4095 * baud;
        let source_div = (source + denominator - 1) / denominator;

        // SCLK_DIV_NUM stores divider - 1 in eight bits.
        is! { source_div == 0 || source_div > 256, return None }

        // UART divider has a 12-bit integer and 4-bit fractional part.
        let divider_16 = (source << 4) / (baud * source_div);
        let integer = divider_16 >> 4;
        let fraction = divider_16 & 0xf;

        is! { integer == 0 || integer > 4095, return None }
        Some((integer as u16, fraction as u8, (source_div - 1) as u8))
    }
}

/// # Configuration
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl Esp32C3Uart {
    /// Configures asynchronous 8N1 using the XTAL clock.
    ///
    /// # Panics
    /// Panics if the requested baud rate cannot be represented.
    ///
    /// # Safety
    /// This must describe an active UART peripheral which is not being
    /// concurrently used or configured.
    pub unsafe fn configure_8n1_xtal(self, xtal_hz: u32, baud: u32) {
        let Some((integer, fraction, source_div)) = Self::baud_setting(xtal_hz, baud) else {
            panic!("ESP UART baud rate is not representable");
        };

        unsafe {
            // Select XTAL and enable the UART core, TX, and RX clocks.
            let clock = self.clock_config_reg();
            let mut value = clock.read();

            value &= !(Self::SCLK_DIV_B_MASK
                | Self::SCLK_DIV_A_MASK
                | Self::SCLK_DIV_NUM_MASK
                | Self::SCLK_SEL_MASK);

            value |= (source_div as u32) << Self::SCLK_DIV_NUM_SHIFT;
            value |= Self::SCLK_SEL_XTAL
                | Self::SCLK_ENABLE
                | Self::TX_SCLK_ENABLE
                | Self::RX_SCLK_ENABLE;

            clock.write(value);

            // The UART baud divider is integer + fraction/16.
            self.clock_div_reg().write(integer as u32 | ((fraction as u32) << 20));

            // 8 data bits, no parity, one stop bit, no TX flow control.
            let config = self.config_reg();
            let mut value = config.read();

            value &= !(Self::PARITY_ENABLE
                | Self::DATA_BITS_MASK
                | Self::STOP_BITS_MASK
                | Self::TX_FLOW_ENABLE);

            value |= Self::DATA_BITS_8 | Self::STOP_BITS_1;
            config.write(value);

            // Disable RX hardware flow control as well.
            let config1 = self.config1_reg();
            config1.write(config1.read() & !Self::RX_FLOW_ENABLE);

            // Start with empty receive and transmit FIFOs.
            config.write(value | Self::RXFIFO_RESET | Self::TXFIFO_RESET);
            config.write(value);
        }
    }
}

/// # Receiver
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl Esp32C3Uart {
    /// Returns the number of bytes currently held in the receive FIFO.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently read.
    #[must_use]
    pub unsafe fn rx_len(self) -> u16 {
        unsafe { (self.status_reg().read() & Self::RXFIFO_COUNT_MASK) as u16 }
    }

    /// Returns whether a received byte is ready.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently read.
    #[must_use]
    pub unsafe fn rx_ready(self) -> bool {
        unsafe { self.rx_len() != 0 }
    }

    /// Attempts to read one received byte without waiting.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently read.
    #[must_use]
    pub unsafe fn try_read_byte(self) -> Option<u8> {
        is! { unsafe { self.rx_ready() }, Some(unsafe { self.fifo_reg().read() as u8 }), None }
    }
    /// Reads one received byte, waiting until one is available.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently read.
    #[must_use]
    pub unsafe fn read_byte_blocking(self) -> u8 {
        while !unsafe { self.rx_ready() } {}
        unsafe { self.fifo_reg().read() as u8 }
    }
}

/// # Transmitter
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl Esp32C3Uart {
    /// Returns the number of bytes currently held in the transmit FIFO.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently written.
    #[must_use]
    pub unsafe fn tx_len(self) -> u16 {
        unsafe {
            ((self.status_reg().read() & Self::TXFIFO_COUNT_MASK) >> Self::TXFIFO_COUNT_SHIFT)
                as u16
        }
    }
    /// Returns whether the transmit FIFO can accept another byte.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently written.
    #[must_use]
    pub unsafe fn tx_ready(self) -> bool {
        unsafe { self.tx_len() < Self::FIFO_CAPACITY as u16 }
    }

    /// Attempts to send one byte without waiting.
    ///
    /// Returns `false` if the transmit FIFO is full.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently written.
    #[must_use]
    pub unsafe fn try_write_byte(self, byte: u8) -> bool {
        if unsafe { self.tx_ready() } {
            unsafe { self.fifo_reg().write(byte as u32) };
            true
        } else {
            false
        }
    }
    /// Sends one byte, waiting until the transmit FIFO has space.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently written.
    pub unsafe fn write_byte_blocking(self, byte: u8) {
        while !unsafe { self.tx_ready() } {}
        unsafe { self.fifo_reg().write(byte as u32) };
    }
    /// Sends all bytes, waiting for transmit FIFO space as needed.
    ///
    /// # Safety
    /// This must describe the active UART peripheral and not be concurrently written.
    pub unsafe fn write_bytes_blocking(self, bytes: &[u8]) {
        for &byte in bytes {
            unsafe { self.write_byte_blocking(byte) };
        }
    }
}
