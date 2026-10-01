//
//! Defines [`BoardLilygoTDisplayS3`].
//

use crate::{Esp32S3Pin, EspUsbSerialJtag, McuEsp32S3};

#[doc = crate::_tags!(hw namespace)]
/// LILYGO T-Display-S3 board.
#[doc = crate::_doc_meta!{
    location("board/esp32", struct BoardLilygoTDisplayS3),
    test_size_of(BoardLilygoTDisplayS3 = 0),
}]
/// An ESP32-S3R8 development board with a 1.9-inch color LCD.
///
/// The board integrates:
///
/// - 16 MiB flash,
/// - 8 MiB OPI PSRAM,
/// - a 170 × 320 ST7789V LCD over an 8-bit parallel interface,
/// - two buttons,
/// - battery-voltage sensing,
/// - a Qwiic/STEMMA QT I²C connector,
/// - native USB Serial/JTAG.
///
/// GPIO15 controls power to the onboard peripherals and must be driven high
/// before using the display.
///
/// This type records the fixed board wiring. LCD-controller and parallel-bus
/// implementations remain separate from the board definition.
///
/// See also:
///
/// - [LILYGO T-Display-S3 product](https://lilygo.cc/products/t-display-s3)
/// - [LILYGO T-Display-S3 documentation](https://wiki.lilygo.cc/products/t-display-series/t-display-s3/)
/// - [LILYGO T-Display-S3 repository](https://github.com/Xinyuan-LilyGO/T-Display-S3)
#[derive(Debug)]
pub struct BoardLilygoTDisplayS3;

impl BoardLilygoTDisplayS3 {
    /// The associated ESP32-S3 microcontroller.
    pub const MCU: McuEsp32S3 = McuEsp32S3;

    /// Onboard flash capacity in bytes.
    pub const FLASH_BYTES: usize = 16 * 1024 * 1024;

    /// Onboard OPI PSRAM capacity in bytes.
    pub const PSRAM_BYTES: usize = 8 * 1024 * 1024;
}

/// # Board power
impl BoardLilygoTDisplayS3 {
    /// Peripheral power-enable line.
    ///
    /// This must be driven high before using the LCD and other powered
    /// peripherals.
    pub const POWER_EN: Esp32S3Pin = Esp32S3Pin::new(15);

    /// Enables power to the board peripherals.
    ///
    /// # Safety
    /// The board GPIO configuration must not be concurrently modified.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn enable_peripherals() {
        unsafe { Self::POWER_EN.set_output_high() };
    }
}

/// # Buttons
impl BoardLilygoTDisplayS3 {
    /// BOOT button on GPIO0.
    pub const BUTTON_BOOT: Esp32S3Pin = Esp32S3Pin::new(0);

    /// User button on GPIO14.
    pub const BUTTON_USER: Esp32S3Pin = Esp32S3Pin::new(14);
}

/// # LCD
impl BoardLilygoTDisplayS3 {
    /// LCD backlight.
    pub const LCD_BL: Esp32S3Pin = Esp32S3Pin::new(38);

    /// LCD reset.
    pub const LCD_RST: Esp32S3Pin = Esp32S3Pin::new(5);

    /// LCD chip select.
    pub const LCD_CS: Esp32S3Pin = Esp32S3Pin::new(6);

    /// LCD data/command select.
    pub const LCD_DC: Esp32S3Pin = Esp32S3Pin::new(7);

    /// LCD write strobe.
    pub const LCD_WR: Esp32S3Pin = Esp32S3Pin::new(8);

    /// LCD read strobe.
    pub const LCD_RD: Esp32S3Pin = Esp32S3Pin::new(9);

    /// LCD parallel data bit 0.
    pub const LCD_D0: Esp32S3Pin = Esp32S3Pin::new(39);
    /// LCD parallel data bit 1.
    pub const LCD_D1: Esp32S3Pin = Esp32S3Pin::new(40);
    /// LCD parallel data bit 2.
    pub const LCD_D2: Esp32S3Pin = Esp32S3Pin::new(41);
    /// LCD parallel data bit 3.
    pub const LCD_D3: Esp32S3Pin = Esp32S3Pin::new(42);
    /// LCD parallel data bit 4.
    pub const LCD_D4: Esp32S3Pin = Esp32S3Pin::new(45);
    /// LCD parallel data bit 5.
    pub const LCD_D5: Esp32S3Pin = Esp32S3Pin::new(46);
    /// LCD parallel data bit 6.
    pub const LCD_D6: Esp32S3Pin = Esp32S3Pin::new(47);
    /// LCD parallel data bit 7.
    pub const LCD_D7: Esp32S3Pin = Esp32S3Pin::new(48);
}

/// # I²C
impl BoardLilygoTDisplayS3 {
    /// Qwiic/STEMMA QT I²C clock.
    pub const I2C_SCL: Esp32S3Pin = Esp32S3Pin::new(17);

    /// Qwiic/STEMMA QT I²C data.
    pub const I2C_SDA: Esp32S3Pin = Esp32S3Pin::new(18);
}

/// # Battery
impl BoardLilygoTDisplayS3 {
    /// Battery-voltage ADC input.
    pub const BATTERY_ADC: Esp32S3Pin = Esp32S3Pin::new(4);
}

/// # Serial
impl BoardLilygoTDisplayS3 {
    /// UART transmit line.
    pub const UART_TX: Esp32S3Pin = Esp32S3Pin::new(43);

    /// UART receive line.
    pub const UART_RX: Esp32S3Pin = Esp32S3Pin::new(44);

    /// Native USB Serial/JTAG exposed through the USB-C connector.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32S3::USB_SERIAL_JTAG;
}
