// devela/src/sys/hw/mcu/esp32/c3.rs
//
//! Defines [`McuEsp32C3`].
//

use crate::EspReg32;

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32", struct McuEsp32C3),
    test_size_of(McuEsp32C3 = 0),
}]
#[derive(Debug)]
pub struct McuEsp32C3;

impl McuEsp32C3 {
    /// GPIO peripheral base address.
    pub const GPIO_BASE: u32 = 0x6000_4000;

    /// GPIO output-value register.
    pub const GPIO_OUT: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0004);

    /// GPIO output set register.
    ///
    /// Writing a `1` sets the corresponding bit in [`GPIO_OUT`](Self::GPIO_OUT).
    pub const GPIO_OUT_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0008);

    /// GPIO output clear register.
    ///
    /// Writing a `1` clears the corresponding bit in [`GPIO_OUT`](Self::GPIO_OUT).
    pub const GPIO_OUT_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x000c);

    /// GPIO output-enable register.
    pub const GPIO_ENABLE: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0020);

    /// GPIO output-enable set register.
    pub const GPIO_ENABLE_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0024);

    /// GPIO output-enable clear register.
    pub const GPIO_ENABLE_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0028);

    /// GPIO input-value register.
    pub const GPIO_IN: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x003c);
}
