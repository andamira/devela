// devela/sys/hw/mcu/board/generic/supermini_oled042.rs
//
//! Defines [`BoardSuperMiniOled042`].
//

use crate::{Esp32C3Pin, EspUsbSerialJtag, McuEsp32C3};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 SuperMini board with a 0.42-inch OLED.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/board", struct BoardSuperMiniOled042),
    test_size_of(BoardSuperMiniOled042 = 0),
}]
/// See also [`McuEsp32C3`][crate::McuEsp32C3].
#[derive(Debug)]
pub struct BoardSuperMiniOled042;

impl BoardSuperMiniOled042 {
    /// Built-in active-low blue LED on GPIO8.
    pub const LED: Esp32C3Pin = Esp32C3Pin::new(8);

    /// Native USB serial interface exposed through the board's USB connector.
    pub const USB_SERIAL: EspUsbSerialJtag = McuEsp32C3::USB_SERIAL_JTAG;
}
