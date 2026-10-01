//
//! Defines [`BoardLilygoTWatchS3`].
//

use crate::{Esp32S3Pin, EspUsbSerialJtag, McuEsp32S3};

#[doc = crate::_tags!(hw namespace)]
/// LILYGO T-Watch-S3 board.
#[doc = crate::_doc_meta!{
    location("board/esp32", struct BoardLilygoTWatchS3),
    test_size_of(BoardLilygoTWatchS3 = 0),
}]
/// An ESP32-S3 smartwatch board with touch LCD, motion sensing,
/// audio, haptics, RTC, power management, and radio.
///
/// The board integrates:
///
/// - 16 MiB QSPI flash,
/// - 8 MiB OPI PSRAM,
/// - a 240 × 240 ST7789V3 LCD over SPI,
/// - an FT6336U capacitive touch controller,
/// - a BMA423/BMA456 three-axis accelerometer, depending on production batch,
/// - a PCF8563 real-time clock,
/// - an AXP2101 power-management IC,
/// - a DRV2605 haptic driver,
/// - a PDM microphone,
/// - a MAX98357A audio amplifier,
/// - SX1262/SX1280 radio hardware, depending on variant,
/// - an infrared transmitter,
/// - native USB Serial/JTAG.
///
/// The LCD backlight, LCD/touch, radio, and haptic circuitry are powered through
/// AXP2101 rails. In particular, driving [`Self::LCD_BL`] alone does not ensure
/// that the display backlight is powered.
///
/// This type records fixed board wiring. Device-controller implementations
/// remain separate from the board definition.
///
/// See also:
///
/// - [LILYGO T-Watch-S3 product](https://lilygo.cc/products/t-watch-s3)
/// - [LILYGO T-Watch-S3 documenation](https://wiki.lilygo.cc/products/t-watch-series/t-watch-s3/)
/// - [LILYGO T-Watch-S3 hardware reference]
/// - [LilyGoLib]
///
/// [LILYGO T-Watch-S3 hardware reference]:
///     https://github.com/Xinyuan-LilyGO/LilyGoLib/blob/master/docs/hardware/lilygo-t-watch-s3.md
/// [LilyGoLib]:
///     https://github.com/Xinyuan-LilyGO/LilyGoLib
#[derive(Debug)]
pub struct BoardLilygoTWatchS3;

impl BoardLilygoTWatchS3 {
    /// The associated ESP32-S3 microcontroller.
    pub const MCU: McuEsp32S3 = McuEsp32S3;

    /// Onboard flash capacity in bytes.
    pub const FLASH_BYTES: usize = 16 * 1024 * 1024;

    /// Onboard OPI PSRAM capacity in bytes.
    pub const PSRAM_BYTES: usize = 8 * 1024 * 1024;
}

/// # Shared I²C bus
impl BoardLilygoTWatchS3 {
    /// Shared I²C data line.
    pub const I2C_SDA: Esp32S3Pin = Esp32S3Pin::new(10);

    /// Shared I²C clock line.
    pub const I2C_SCL: Esp32S3Pin = Esp32S3Pin::new(11);

    /// AXP2101 power-management interrupt.
    pub const PMIC_INT: Esp32S3Pin = Esp32S3Pin::new(21);

    /// AXP2101 I²C address.
    pub const PMIC_I2C_ADDR: u8 = 0x34;

    /// PCF8563 RTC interrupt.
    pub const RTC_INT: Esp32S3Pin = Esp32S3Pin::new(17);

    /// PCF8563 RTC I²C address.
    pub const RTC_I2C_ADDR: u8 = 0x51;

    /// BMA4xx accelerometer interrupt.
    pub const ACCEL_INT: Esp32S3Pin = Esp32S3Pin::new(14);

    /// BMA423/BMA456 accelerometer I²C address.
    pub const ACCEL_I2C_ADDR: u8 = 0x19;

    /// DRV2605 haptic-controller I²C address.
    pub const HAPTIC_I2C_ADDR: u8 = 0x5A;
}

/// # Touch
impl BoardLilygoTWatchS3 {
    /// FT6336U touch-controller I²C data line.
    pub const TOUCH_SDA: Esp32S3Pin = Esp32S3Pin::new(39);

    /// FT6336U touch-controller I²C clock line.
    pub const TOUCH_SCL: Esp32S3Pin = Esp32S3Pin::new(40);

    /// FT6336U touch-controller interrupt.
    pub const TOUCH_INT: Esp32S3Pin = Esp32S3Pin::new(16);

    /// FT6336U touch-controller I²C address.
    pub const TOUCH_I2C_ADDR: u8 = 0x38;
}

/// # LCD
impl BoardLilygoTWatchS3 {
    /// LCD chip select.
    pub const LCD_CS: Esp32S3Pin = Esp32S3Pin::new(12);

    /// LCD controller-output / peripheral-input line.
    pub const LCD_MOSI: Esp32S3Pin = Esp32S3Pin::new(13);

    /// LCD SPI clock.
    pub const LCD_SCK: Esp32S3Pin = Esp32S3Pin::new(18);

    /// LCD data/command select.
    pub const LCD_DC: Esp32S3Pin = Esp32S3Pin::new(38);

    /// LCD backlight-control signal.
    ///
    /// Backlight power itself is supplied by the AXP2101 ALDO2 rail.
    pub const LCD_BL: Esp32S3Pin = Esp32S3Pin::new(45);
}

/// # Radio
impl BoardLilygoTWatchS3 {
    /// Radio SPI clock.
    pub const RADIO_SCK: Esp32S3Pin = Esp32S3Pin::new(3);

    /// Radio controller-input / peripheral-output line.
    pub const RADIO_MISO: Esp32S3Pin = Esp32S3Pin::new(4);

    /// Radio controller-output / peripheral-input line.
    pub const RADIO_MOSI: Esp32S3Pin = Esp32S3Pin::new(1);

    /// Radio reset.
    pub const RADIO_RST: Esp32S3Pin = Esp32S3Pin::new(8);

    /// Radio busy signal.
    pub const RADIO_BUSY: Esp32S3Pin = Esp32S3Pin::new(7);

    /// Radio chip select.
    pub const RADIO_CS: Esp32S3Pin = Esp32S3Pin::new(5);

    /// Radio interrupt.
    pub const RADIO_IRQ: Esp32S3Pin = Esp32S3Pin::new(9);
}

/// # Audio
impl BoardLilygoTWatchS3 {
    /// MAX98357A I²S bit clock.
    pub const I2S_BCK: Esp32S3Pin = Esp32S3Pin::new(48);

    /// MAX98357A I²S word-select.
    pub const I2S_WS: Esp32S3Pin = Esp32S3Pin::new(15);

    /// MAX98357A I²S data output.
    pub const I2S_DOUT: Esp32S3Pin = Esp32S3Pin::new(46);

    /// PDM microphone clock.
    pub const PDM_SCK: Esp32S3Pin = Esp32S3Pin::new(44);

    /// PDM microphone data.
    pub const PDM_DATA: Esp32S3Pin = Esp32S3Pin::new(47);
}

/// # Infrared
impl BoardLilygoTWatchS3 {
    /// Infrared-transmitter output.
    pub const IR_TX: Esp32S3Pin = Esp32S3Pin::new(2);
}

/// # Serial
impl BoardLilygoTWatchS3 {
    /// Native USB Serial/JTAG.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32S3::USB_SERIAL_JTAG;
}
