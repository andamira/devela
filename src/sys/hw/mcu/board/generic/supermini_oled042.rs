// devela/sys/hw/mcu/board/generic/supermini_oled042.rs
//
//! Defines [`BoardSuperMiniOled042`].
//

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
    /// GPIO connected to the active-low blue LED.
    pub const LED_GPIO: u8 = 8;

    /// Bit mask for the active-low blue LED.
    pub const LED_MASK: u32 = 1 << Self::LED_GPIO;
}
