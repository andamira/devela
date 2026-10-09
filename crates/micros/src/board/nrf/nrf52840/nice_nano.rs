//
//! Defines [`BoardNiceNano`].
//

use crate::{McuNrf52840, NrfPin};

#[doc = crate::_tags!(hw namespace)]
/// Nice Keyboards nice!nano nRF52840 board namespace.
#[doc = crate::_doc_meta!{
    location("board/nrf", struct BoardNiceNano),
    test_size_of(BoardNiceNano = 0),
}]
/// The programmable blue status LED is P0.15, active-high.
///
/// Board revisions have different power-switch wiring: this namespace
/// deliberately does not expose an unverified power-control pin.
///
/// See [nice!nano documentation](https://nicekeyboards.com/docs/nice-nano/)
/// and the [Adafruit bootloader board definition](https://github.com/adafruit/Adafruit_nRF52_Bootloader/blob/master/src/boards/nice_nano/board.h).
#[derive(Debug)]
pub struct BoardNiceNano;

impl BoardNiceNano {
    /// Associated nRF52840 microcontroller.
    pub const MCU: McuNrf52840 = McuNrf52840;
    /// Programmable blue status LED (active-high).
    pub const LED: NrfPin = NrfPin::new(McuNrf52840::P0, 15);
}
