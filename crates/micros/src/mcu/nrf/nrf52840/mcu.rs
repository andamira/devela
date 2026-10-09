//
//! Defines [`McuNrf52840`].
//

use crate::NrfPort;

#[doc = crate::_tags!(hw namespace)]
/// Nordic Semiconductor nRF52840 microcontroller.
#[doc = crate::_doc_meta!{
    location("mcu/nrf", struct McuNrf52840),
    test_size_of(McuNrf52840 = 0),
}]
/// The nRF52840 has a 64 MHz Arm Cortex-M4F core, 1 MiB of Flash,
/// 256 KiB of RAM, and 48 GPIO pins across two ports.
#[derive(Debug)]
pub struct McuNrf52840;

impl McuNrf52840 {
    /// GPIO port 0, with pins P0.00–P0.31.
    pub const P0: NrfPort = NrfPort::P0;
    /// GPIO port 1, with pins P1.00–P1.15.
    pub const P1: NrfPort = NrfPort::P1;
    /// CPU clock nominal frequency in hertz.
    pub const CPU_HZ: u32 = 64_000_000;
    /// On-chip Flash capacity in bytes.
    pub const FLASH_BYTES: u32 = 1024 * 1024;
    /// On-chip RAM capacity in bytes.
    pub const RAM_BYTES: u32 = 256 * 1024;
}
