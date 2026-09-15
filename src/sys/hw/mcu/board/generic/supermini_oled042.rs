// devela/sys/hw/mcu/board/generic/supermini_oled042.rs
//
//! Defines [`BoardSuperMiniOled042`].
//

use crate::{Esp32C3Pin, Esp32C3Uart, EspI2c, EspUsbSerialJtag, I2cAddr7, McuEsp32C3};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 SuperMini board with a 0.42-inch OLED.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/board", struct BoardSuperMiniOled042),
    test_size_of(BoardSuperMiniOled042 = 0),
}]
/// A compact generic development board based on [`McuEsp32C3`],
/// with an onboard 0.42-inch monochrome OLED.
///
/// This definition targets the common OLED variant tested by devela:
///
/// - active-low blue LED on GPIO8,
/// - 72 × 40 OLED over I²C0,
/// - OLED SDA on GPIO5 and SCL on GPIO6,
/// - OLED address `0x3c`,
/// - native USB Serial/JTAG through the USB connector,
/// - direct UART0 routing on GPIO20 RX and GPIO21 TX.
///
/// Boards sold under this name are not controlled by a single vendor
/// and variants exist, so their wiring should be checked before assuming parity.
///
/// See also:
///
/// - [ESP32-C3 hardware reference]
/// - [SuperMini board notes]
/// - [0.42-inch OLED board notes]
///
/// [ESP32-C3 hardware reference]: https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/index.html
/// [SuperMini board notes]: https://sigmdel.ca/michel/ha/esp8266/super_mini_esp32c3_en.html
/// [0.42-inch OLED board notes]: https://github.com/ESP32Home/oled_042

#[derive(Debug)]
pub struct BoardSuperMiniOled042;

/// # Board I/O
impl BoardSuperMiniOled042 {
    /// Built-in active-low blue LED on GPIO8.
    pub const LED: Esp32C3Pin = Esp32C3Pin::new(8);
}

/// # OLED
impl BoardSuperMiniOled042 {
    /// I²C controller connected to the OLED.
    pub const OLED_I2C: EspI2c = McuEsp32C3::I2C0;

    /// OLED I²C bus frequency in hertz.
    pub const OLED_I2C_HZ: u32 = 400_000;

    /// OLED I²C data line.
    pub const OLED_SDA: Esp32C3Pin = Esp32C3Pin::new(5);

    /// OLED I²C clock line.
    pub const OLED_SCL: Esp32C3Pin = Esp32C3Pin::new(6);

    /// OLED 7-bit I²C address.
    pub const OLED_ADDR: I2cAddr7 = I2cAddr7::new(0x3c);

    /// OLED visible width in pixels.
    pub const OLED_WIDTH: usize = 72;

    /// OLED visible height in pixels.
    pub const OLED_HEIGHT: usize = 40;
}

/// # Serial
impl BoardSuperMiniOled042 {
    /// UART0, directly routed to GPIO20 RX and GPIO21 TX.
    pub const UART0: Esp32C3Uart = McuEsp32C3::UART0;

    /// Native USB serial interface exposed through the board's USB connector.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32C3::USB_SERIAL_JTAG;
}
