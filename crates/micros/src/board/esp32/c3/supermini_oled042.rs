//
//! Defines [`BoardSuperMiniOled042`].
//

use crate::{Esp32C3Pin, Esp32C3Uart, EspUsbSerialJtag, I2cAddr7, McuEsp32C3};
#[cfg(feature = "unsafe_mmio")]
use crate::{EspI2c, I2cController};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 SuperMini board with a 0.42-inch OLED.
#[doc = crate::_doc_meta!{
    location("board/esp32", struct BoardSuperMiniOled042),
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

    /// Number of 8-pixel pages in the OLED framebuffer.
    pub const OLED_PAGE_COUNT: usize = Self::OLED_HEIGHT / 8;

    /// Number of bytes in one native 1-bit OLED frame.
    pub const OLED_FRAME_BYTES: usize = Self::OLED_WIDTH * Self::OLED_PAGE_COUNT;

    /// SSD1306 GDDRAM column corresponding to visible column zero.
    pub const OLED_RAM_COLUMN_OFFSET: u8 = 28;

    /// Last SSD1306 GDDRAM column occupied by the visible OLED area.
    pub const OLED_RAM_COLUMN_LAST: u8 = Self::OLED_RAM_COLUMN_OFFSET + Self::OLED_WIDTH as u8 - 1;

    /// Last SSD1306 GDDRAM page occupied by the visible OLED area.
    pub const OLED_RAM_PAGE_LAST: u8 = Self::OLED_PAGE_COUNT as u8 - 1;

    /// Prepares the I²C bus connected to the onboard OLED.
    ///
    /// # Safety
    /// I²C0 and GPIO5/6 must not be concurrently configured or accessed.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn prepare_oled_i2c() -> I2cController<EspI2c> {
        unsafe { McuEsp32C3::prepare_i2c0(Self::OLED_SDA, Self::OLED_SCL, Self::OLED_I2C_HZ) }
    }
}

/// # Serial
impl BoardSuperMiniOled042 {
    /// UART0, directly routed to GPIO20 RX and GPIO21 TX.
    pub const UART0: Esp32C3Uart = McuEsp32C3::UART0;

    /// Native USB serial interface exposed through the board's USB connector.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32C3::USB_SERIAL_JTAG;
}
