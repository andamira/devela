//
//! Defines [`BoardSoliniusSparrow`].
//

use crate::McuAtmega1284;

#[doc = crate::_tags!(hw namespace)]
/// Solinius Sparrow board namespace.
#[doc = crate::_doc_meta!{
    location("board/avr", struct BoardSoliniusSparrow),
    test_size_of(BoardSoliniusSparrow = 0),
}]
/// The board is based on [`McuAtmega1284`] and uses a 14.7456 MHz
/// system clock.
///
/// Additional board wiring will be exposed as it is verified.
#[derive(Debug)]
pub struct BoardSoliniusSparrow;

impl BoardSoliniusSparrow {
    /// The associated ATmega1284 microcontroller.
    pub const MCU: McuAtmega1284 = McuAtmega1284;
}

/// # Clock
impl BoardSoliniusSparrow {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 14_745_600;
}
