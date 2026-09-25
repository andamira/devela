//
//! Defines [`McuSam3x8e`].
//

use crate::SamPort;

#[doc = crate::_tags!(hw namespace)]
/// Microchip SAM3X8E microcontroller.
#[doc = crate::_doc_meta!{
    location("mcu/sam", struct McuSam3x8e),
    test_size_of(McuSam3x8e = 0),
}]
/// The SAM3X8E uses a 32-bit Arm Cortex-M3 core implementing Armv7-M.
#[derive(Debug)]
pub struct McuSam3x8e;

impl McuSam3x8e {
    /// Parallel I/O controller B.
    pub const PIOB: SamPort = SamPort::new(0x400E_1000);
}
