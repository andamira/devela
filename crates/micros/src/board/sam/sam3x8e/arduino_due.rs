//
//! Defines [`BoardArduinoDue`].
//

use crate::{McuSam3x8e, SamPin};

#[doc = crate::_tags!(hw namespace)]
/// Arduino Due board namespace.
#[doc = crate::_doc_meta!{
    location("board/sam", struct BoardArduinoDue),
    test_size_of(BoardArduinoDue = 0),
}]
/// The board is based on the [`McuSam3x8e`] Arm Cortex-M3 microcontroller.
///
/// Arduino digital pin 13 and the built-in amber `L` LED are connected
/// to SAM3X8E pin `PB27`.
///
/// See also:
/// - [Arduino Due documentation]
/// - [SAM3X8E datasheet]
///
/// [Arduino Due documentation]: https://docs.arduino.cc/hardware/due/
/// [SAM3X8E datasheet]: https://docs.arduino.cc/resources/datasheets/A000062-datasheet.pdf
#[derive(Debug)]
pub struct BoardArduinoDue;

impl BoardArduinoDue {
    /// The associated SAM3X8E microcontroller.
    pub const MCU: McuSam3x8e = McuSam3x8e;

    /// Built-in `L` LED on Arduino pin D13 / SAM3X8E PB27.
    pub const LED: SamPin = SamPin::new(McuSam3x8e::PIOB, 27);
}
