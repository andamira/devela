// devela/sys/hw/mcu/board/generic/supermini_oled042.rs
//
//! Defines [`BoardSuperMiniOled042`].
//

use crate::{Esp32C3Pin, EspI2c, EspUsbSerialJtag, I2cAddr7, McuEsp32C3};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 SuperMini board with a 0.42-inch OLED.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/board", struct BoardSuperMiniOled042),
    test_size_of(BoardSuperMiniOled042 = 0),
}]
/// ESP32-C3 SuperMini board with an onboard 0.42-inch OLED.
///
/// This namespace describes the tested 72×40 I²C OLED variant
/// and its fixed board wiring.
///
/// See also [`McuEsp32C3`][crate::McuEsp32C3].
#[derive(Debug)]
pub struct BoardSuperMiniOled042;

impl BoardSuperMiniOled042 {
    /// Built-in active-low blue LED on GPIO8.
    pub const LED: Esp32C3Pin = Esp32C3Pin::new(8);

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

    /// Native USB serial interface exposed through the board's USB connector.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32C3::USB_SERIAL_JTAG;
}
