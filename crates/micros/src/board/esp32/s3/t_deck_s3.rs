//
//! Defines [`BoardLilygoTDeckS3`].
//

use crate::{Esp32S3Pin, EspUsbSerialJtag, McuEsp32S3};

#[doc = crate::_tags!(hw namespace)]
/// LILYGO T-Deck board.
#[doc = crate::_doc_meta!{
    location("board/esp32", struct BoardLilygoTDeckS3),
    test_size_of(BoardLilygoTDeckS3 = 0),
}]
/// An ESP32-S3 handheld board with keyboard, trackball,
/// touch LCD, audio, storage, and optional LoRa radio.
///
/// The board integrates:
///
/// - 16 MiB flash,
/// - 8 MiB OPI PSRAM,
/// - a 320 × 240 ST7789 LCD over SPI,
/// - a GT911 capacitive touch controller,
/// - an I²C keyboard controller,
/// - a four-direction trackball with center button,
/// - a microSD slot,
/// - audio input/output hardware,
/// - optional SX1262 LoRa,
/// - native USB Serial/JTAG.
///
/// GPIO10 controls board peripheral power. It must be driven high when using
/// the peripherals while battery powered.
///
/// This type records fixed board wiring. Peripheral-controller
/// implementations remain separate from the board definition.
///
/// See also:
///
/// - [LILYGO T-Deck product](https://lilygo.cc/products/t-deck)
/// - [LILYGO T-Deck documentation](https://wiki.lilygo.cc/products/t-deck-series/t-deck/)
/// - [LILYGO T-Deck repository](https://github.com/Xinyuan-LilyGO/T-Deck)
#[derive(Debug)]
pub struct BoardLilygoTDeckS3;

impl BoardLilygoTDeckS3 {
    /// The associated ESP32-S3 microcontroller.
    pub const MCU: McuEsp32S3 = McuEsp32S3;

    /// Onboard flash capacity in bytes.
    pub const FLASH_BYTES: usize = 16 * 1024 * 1024;

    /// Onboard OPI PSRAM capacity in bytes.
    pub const PSRAM_BYTES: usize = 8 * 1024 * 1024;
}

/// # Board power
impl BoardLilygoTDeckS3 {
    /// Peripheral power-enable line.
    pub const POWER_EN: Esp32S3Pin = Esp32S3Pin::new(10);

    /// Enables power to the board peripherals.
    ///
    /// # Safety
    /// The board GPIO configuration must not be concurrently modified.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn enable_peripherals() {
        unsafe { Self::POWER_EN.set_output_high() };
    }
}

/// # Buttons and trackball
impl BoardLilygoTDeckS3 {
    /// BOOT button / trackball center button on GPIO0.
    pub const BUTTON_BOOT: Esp32S3Pin = Esp32S3Pin::new(0);

    /// Trackball signal G01.
    pub const TRACKBALL_G01: Esp32S3Pin = Esp32S3Pin::new(3);
    /// Trackball signal G02.
    pub const TRACKBALL_G02: Esp32S3Pin = Esp32S3Pin::new(2);
    /// Trackball signal G03.
    pub const TRACKBALL_G03: Esp32S3Pin = Esp32S3Pin::new(15);
    /// Trackball signal G04.
    pub const TRACKBALL_G04: Esp32S3Pin = Esp32S3Pin::new(1);
}

/// # I²C
impl BoardLilygoTDeckS3 {
    /// Shared I²C data line.
    pub const I2C_SDA: Esp32S3Pin = Esp32S3Pin::new(18);

    /// Shared I²C clock line.
    pub const I2C_SCL: Esp32S3Pin = Esp32S3Pin::new(8);

    /// Keyboard interrupt line.
    pub const KEYBOARD_INT: Esp32S3Pin = Esp32S3Pin::new(46);

    /// Keyboard I²C address.
    pub const KEYBOARD_I2C_ADDR: u8 = 0x55;

    /// GT911 touch-controller interrupt.
    pub const TOUCH_INT: Esp32S3Pin = Esp32S3Pin::new(16);
}

/// # Shared SPI bus
impl BoardLilygoTDeckS3 {
    /// Shared SPI clock.
    pub const SPI_SCK: Esp32S3Pin = Esp32S3Pin::new(40);

    /// Shared SPI controller-output / peripheral-input line.
    pub const SPI_MOSI: Esp32S3Pin = Esp32S3Pin::new(41);

    /// Shared SPI controller-input / peripheral-output line.
    pub const SPI_MISO: Esp32S3Pin = Esp32S3Pin::new(38);
}

/// # LCD
impl BoardLilygoTDeckS3 {
    /// LCD chip select.
    pub const LCD_CS: Esp32S3Pin = Esp32S3Pin::new(12);

    /// LCD data/command select.
    pub const LCD_DC: Esp32S3Pin = Esp32S3Pin::new(11);

    /// LCD backlight-controller input.
    ///
    /// The board uses a pulse-controlled 16-level backlight circuit rather
    /// than a simple linear PWM input.
    pub const LCD_BL: Esp32S3Pin = Esp32S3Pin::new(42);

    /// Number of backlight levels exposed by the board controller.
    pub const LCD_BACKLIGHT_LEVELS: u8 = 16;
}

/// # Storage
impl BoardLilygoTDeckS3 {
    /// microSD card chip select on the shared SPI bus.
    pub const SD_CS: Esp32S3Pin = Esp32S3Pin::new(39);
}

/// # LoRa
impl BoardLilygoTDeckS3 {
    /// Optional SX1262 chip select.
    pub const LORA_CS: Esp32S3Pin = Esp32S3Pin::new(9);

    /// Optional SX1262 busy signal.
    pub const LORA_BUSY: Esp32S3Pin = Esp32S3Pin::new(13);

    /// Optional SX1262 reset.
    pub const LORA_RST: Esp32S3Pin = Esp32S3Pin::new(17);

    /// Optional SX1262 interrupt signal.
    pub const LORA_DIO1: Esp32S3Pin = Esp32S3Pin::new(45);
}

/// # Audio
impl BoardLilygoTDeckS3 {
    /// Audio-output I²S word-select.
    pub const I2S_WS: Esp32S3Pin = Esp32S3Pin::new(5);

    /// Audio-output I²S bit clock.
    pub const I2S_BCK: Esp32S3Pin = Esp32S3Pin::new(7);

    /// Audio-output I²S data.
    pub const I2S_DOUT: Esp32S3Pin = Esp32S3Pin::new(6);

    /// ES7210 microphone master clock.
    pub const MIC_MCLK: Esp32S3Pin = Esp32S3Pin::new(48);

    /// ES7210 microphone LR clock.
    pub const MIC_LRCK: Esp32S3Pin = Esp32S3Pin::new(21);

    /// ES7210 microphone serial clock.
    pub const MIC_SCK: Esp32S3Pin = Esp32S3Pin::new(47);

    /// ES7210 microphone data input.
    pub const MIC_DIN: Esp32S3Pin = Esp32S3Pin::new(14);
}

/// # Battery
impl BoardLilygoTDeckS3 {
    /// Battery-voltage ADC input.
    pub const BATTERY_ADC: Esp32S3Pin = Esp32S3Pin::new(4);
}

/// # Serial
impl BoardLilygoTDeckS3 {
    /// Serial/Grove transmit line.
    pub const UART_TX: Esp32S3Pin = Esp32S3Pin::new(43);

    /// Serial/Grove receive line.
    pub const UART_RX: Esp32S3Pin = Esp32S3Pin::new(44);

    /// Native USB Serial/JTAG exposed through USB-C.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32S3::USB_SERIAL_JTAG;
}
