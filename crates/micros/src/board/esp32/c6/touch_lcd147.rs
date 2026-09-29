//
//! Defines [`BoardWaveshareC6TouchLcd147`].
//

use crate::{Esp32C6Pin, Jd9853, McuEsp32C6};
#[cfg(feature = "unsafe_mmio")]
use crate::{Esp32C6SpiCmdData, Timeout};

#[doc = crate::_tags!(hw namespace)]
/// Waveshare ESP32-C6-Touch-LCD-1.47 board.
#[doc = crate::_doc_meta!{
    location("board/esp32", struct BoardWaveshareC6TouchLcd147),
    test_size_of(BoardWaveshareC6TouchLcd147 = 0),
}]
/// A compact ESP32-C6FH8 board with a 1.47-inch capacitive touch display.
///
/// The board integrates:
///
/// - 8 MiB flash,
/// - a 172 × 320 JD9853 LCD over 4-wire SPI,
/// - an AXS5106L capacitive touch controller over I²C,
/// - a QMI8658A six-axis IMU sharing the I²C bus,
/// - a microSD/TF slot sharing the LCD SPI clock and MOSI lines,
/// - USB, UART0, battery charging, and battery-voltage sensing.
///
/// This type records the board's fixed wiring. Device-specific drivers
/// are separate from the board definition and can be added independently.
///
/// The shared buses require coordination:
///
/// - LCD and TF share GPIO1 SCK and GPIO2 MOSI; their chip-selects are distinct.
/// - Touch and IMU share GPIO18 SDA and GPIO19 SCL.
///
/// GPIO8 and GPIO9 are boot strapping pins. The published schematic connects `IO8`
/// to GPIO8 and the `BOOT` net to GPIO9; Waveshare's current prose pinout instead
/// labels the BOOT button as GPIO8, so either pin should be treated cautiously
/// until the board revision is verified. GPIO12/13 are USB D-/D+ when USB is in use.
///
/// See also:
///
/// - [Waveshare board documentation]
/// - [Waveshare schematic]
/// - [ESP32-C6 hardware reference]
///
/// [Waveshare board documentation]: https://docs.waveshare.com/ESP32-C6-Touch-LCD-1.47
/// [Waveshare schematic]: https://files.waveshare.com/wiki/ESP32-C6-Touch-LCD-1.47/ESP32-C6-Touch-LCD-1.47-Schematic.pdf
/// [ESP32-C6 hardware reference]: https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/hw-reference/index.html
#[derive(Debug)]
pub struct BoardWaveshareC6TouchLcd147;

impl BoardWaveshareC6TouchLcd147 {
    /// The associated ESP32-C6 microcontroller.
    pub const MCU: McuEsp32C6 = McuEsp32C6;

    /// On-package flash capacity of the ESP32-C6FH8, in bytes.
    pub const FLASH_BYTES: usize = 8 * 1024 * 1024;
}

/// # Shared SPI bus
impl BoardWaveshareC6TouchLcd147 {
    /// Shared LCD/TF SPI clock line.
    pub const SPI_SCK: Esp32C6Pin = Esp32C6Pin::new(1);

    /// Shared LCD/TF SPI controller-to-device data line.
    pub const SPI_MOSI: Esp32C6Pin = Esp32C6Pin::new(2);
}

/// # LCD
impl BoardWaveshareC6TouchLcd147 {
    /// JD9853 panel profile.
    pub const LCD: Jd9853 = Jd9853::WAVESHARE_C6_TOUCH_LCD147;

    /// Initial LCD SPI bus frequency in hertz.
    ///
    /// This conservative bring-up frequency divides the known 40 MHz XTAL
    /// directly and does not depend on PLL clock setup.
    pub const LCD_SPI_HZ: u32 = 20_000_000;

    /// LCD width in pixels.
    pub const LCD_WIDTH: u16 = Self::LCD.width();

    /// LCD height in pixels.
    pub const LCD_HEIGHT: u16 = Self::LCD.height();

    /// LCD SPI clock line, shared with TF.
    pub const LCD_SCK: Esp32C6Pin = Self::SPI_SCK;

    /// LCD SPI data line, shared with TF. Waveshare labels this `LCD_SDA`.
    pub const LCD_MOSI: Esp32C6Pin = Self::SPI_MOSI;

    /// LCD chip-select line.
    pub const LCD_CS: Esp32C6Pin = Esp32C6Pin::new(14);

    /// LCD data/command select line.
    pub const LCD_DC: Esp32C6Pin = Esp32C6Pin::new(15);

    /// LCD reset line.
    pub const LCD_RST: Esp32C6Pin = Esp32C6Pin::new(22);

    /// Active-high LCD backlight control line.
    ///
    /// GPIO23 drives the base of the board's NPN low-side switch for `LEDK`.
    pub const LCD_BL: Esp32C6Pin = Esp32C6Pin::new(23);

    /// Prepares SPI2 and the command/data control lines for the onboard LCD.
    ///
    /// The TF card is deselected before the shared SCK/MOSI lines are used.
    /// The LCD reset and backlight pins remain under caller control.
    ///
    /// # Safety
    /// SPI2 and the LCD/TF SPI pins must not be concurrently configured
    /// or accessed. While the returned transport is alive, its SPI2, CS,
    /// and D/C resources must not be accessed through another raw handle.
    #[cfg(feature = "unsafe_mmio")]
    pub unsafe fn prepare_lcd_spi() -> Result<Esp32C6SpiCmdData, Timeout> {
        unsafe {
            Self::TF_CS.set_output_high();
            let (spi, _actual_hz) =
                McuEsp32C6::prepare_spi2(Self::LCD_SCK, Self::LCD_MOSI, Self::LCD_SPI_HZ)?;
            Ok(Esp32C6SpiCmdData::new_unchecked(spi, Self::LCD_CS, Self::LCD_DC))
        }
    }
}

/// # Shared I²C bus
impl BoardWaveshareC6TouchLcd147 {
    /// Shared touch/IMU I²C data line.
    pub const I2C_SDA: Esp32C6Pin = Esp32C6Pin::new(18);

    /// Shared touch/IMU I²C clock line.
    pub const I2C_SCL: Esp32C6Pin = Esp32C6Pin::new(19);
}

/// # Touch
impl BoardWaveshareC6TouchLcd147 {
    /// Touch-controller I²C data line.
    pub const TOUCH_SDA: Esp32C6Pin = Self::I2C_SDA;

    /// Touch-controller I²C clock line.
    pub const TOUCH_SCL: Esp32C6Pin = Self::I2C_SCL;

    /// AXS5106L touch-controller reset line.
    pub const TOUCH_RST: Esp32C6Pin = Esp32C6Pin::new(20);

    /// AXS5106L touch-controller interrupt line.
    pub const TOUCH_INT: Esp32C6Pin = Esp32C6Pin::new(21);
}

/// # IMU
impl BoardWaveshareC6TouchLcd147 {
    /// QMI8658A I²C data line.
    pub const IMU_SDA: Esp32C6Pin = Self::I2C_SDA;

    /// QMI8658A I²C clock line.
    pub const IMU_SCL: Esp32C6Pin = Self::I2C_SCL;

    /// QMI8658A interrupt 1 line.
    pub const IMU_INT1: Esp32C6Pin = Esp32C6Pin::new(5);

    /// QMI8658A interrupt 2 line.
    pub const IMU_INT2: Esp32C6Pin = Esp32C6Pin::new(6);
}

/// # TF / microSD
impl BoardWaveshareC6TouchLcd147 {
    /// TF SPI clock line, shared with the LCD.
    pub const TF_SCK: Esp32C6Pin = Self::SPI_SCK;

    /// TF SPI controller-to-card data line, shared with the LCD.
    pub const TF_MOSI: Esp32C6Pin = Self::SPI_MOSI;

    /// TF SPI card-to-controller data line.
    pub const TF_MISO: Esp32C6Pin = Esp32C6Pin::new(3);

    /// TF SPI chip-select line.
    pub const TF_CS: Esp32C6Pin = Esp32C6Pin::new(4);
}

/// # Host, serial, and board I/O
impl BoardWaveshareC6TouchLcd147 {
    /// USB D- line.
    pub const USB_D_MINUS: Esp32C6Pin = Esp32C6Pin::new(12);

    /// USB D+ line.
    pub const USB_D_PLUS: Esp32C6Pin = Esp32C6Pin::new(13);

    /// UART0 transmit line.
    pub const UART0_TX: Esp32C6Pin = Esp32C6Pin::new(16);

    /// UART0 receive line.
    pub const UART0_RX: Esp32C6Pin = Esp32C6Pin::new(17);

    /// Battery-voltage ADC input through the board's 200K/100K divider.
    ///
    /// The board-level relation is `VBAT = VADC × 3`.
    pub const BAT_ADC: Esp32C6Pin = Esp32C6Pin::new(0);

    /// BOOT net on GPIO9 according to the published board schematic.
    ///
    /// Waveshare's current prose pinout instead labels the BOOT button as GPIO8,
    /// so this mapping should be verified on the physical board before
    /// using the pin for anything beyond the board's boot function.
    pub const BOOT: Esp32C6Pin = Esp32C6Pin::new(9);

    /// Uncommitted GPIO7 broken out on the board header.
    pub const IO7: Esp32C6Pin = Esp32C6Pin::new(7);

    /// GPIO8 broken out on the board header.
    ///
    /// GPIO8 is also a boot strapping pin on the ESP32-C6.
    pub const IO8: Esp32C6Pin = Esp32C6Pin::new(8);
}
