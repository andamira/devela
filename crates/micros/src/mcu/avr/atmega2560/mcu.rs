//
//! Defines [`McuAtmega2560`].
//

use crate::{AvrPort, AvrUsart};

#[doc = crate::_tags!(hw namespace)]
/// ATmega2560 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct McuAtmega2560),
    test_size_of(McuAtmega2560 = 0),
}]
/// The device provides 32 × 8-bit general-purpose working registers,
/// 256 KiB of Flash program memory, 8 KiB of SRAM, and 4 KiB of EEPROM.
/// The CPU working registers are distinct from memory-mapped peripheral
/// registers such as [`AvrReg8`][crate::AvrReg8]. Runtime data and the stack
/// share SRAM, while Flash and EEPROM are separate storage spaces.
///
/// devela currently provides low-level GPIO access and all four USARTs.
///
/// The device itself provides six timer/counters:
/// - two 8-bit timers (Timer/Counter0 and Timer/Counter2),
/// - four 16-bit timers (Timer/Counter1, 3, 4, and 5).
///
/// It also provides a 16-channel 10-bit ADC.
///
/// GPIO pins use the AVR notation `Pxy`, where `x` identifies the port
/// and `y` the bit within it; for example, `PB7` is port B bit 7.
///
/// Ports H, J, K, and L use registers in the extended I/O region
/// and are addressed through AVR data-space addresses.
///
/// See also:
///
/// - [ATmega2560 product page]
/// - [ATmega640/1280/1281/2560/2561 datasheet]
///
/// [ATmega2560 product page]: https://www.microchip.com/en-us/product/atmega2560
/// [ATmega640/1280/1281/2560/2561 datasheet]:
///     https://ww1.microchip.com/downloads/en/DeviceDoc/ATmega640-1280-1281-2560-2561-Datasheet-DS40002211A.pdf
#[derive(Debug)]
#[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("avr")))))]
pub struct McuAtmega2560;

/// # GPIO
impl McuAtmega2560 {
    /// GPIO port A.
    pub const PORT_A: AvrPort = AvrPort::new(0x20, 0x21, 0x22);
    /// GPIO port B.
    pub const PORT_B: AvrPort = AvrPort::new(0x23, 0x24, 0x25);
    /// GPIO port C.
    pub const PORT_C: AvrPort = AvrPort::new(0x26, 0x27, 0x28);
    /// GPIO port D.
    pub const PORT_D: AvrPort = AvrPort::new(0x29, 0x2A, 0x2B);
    /// GPIO port E.
    pub const PORT_E: AvrPort = AvrPort::new(0x2C, 0x2D, 0x2E);
    /// GPIO port F.
    pub const PORT_F: AvrPort = AvrPort::new(0x2F, 0x30, 0x31);
    /// GPIO port G.
    pub const PORT_G: AvrPort = AvrPort::new(0x32, 0x33, 0x34);

    /// GPIO port H.
    pub const PORT_H: AvrPort = AvrPort::new(0x100, 0x101, 0x102);
    /// GPIO port J.
    pub const PORT_J: AvrPort = AvrPort::new(0x103, 0x104, 0x105);
    /// GPIO port K.
    pub const PORT_K: AvrPort = AvrPort::new(0x106, 0x107, 0x108);
    /// GPIO port L.
    pub const PORT_L: AvrPort = AvrPort::new(0x109, 0x10A, 0x10B);
}

/// # Serial
impl McuAtmega2560 {
    /// USART 0 peripheral.
    pub const USART_0: AvrUsart = AvrUsart::new(0xC0, 0xC1, 0xC2, 0xC4, 0xC5, 0xC6);
    /// USART 1 peripheral.
    pub const USART_1: AvrUsart = AvrUsart::new(0xC8, 0xC9, 0xCA, 0xCC, 0xCD, 0xCE);
    /// USART 2 peripheral.
    pub const USART_2: AvrUsart = AvrUsart::new(0xD0, 0xD1, 0xD2, 0xD4, 0xD5, 0xD6);
    /// USART 3 peripheral.
    pub const USART_3: AvrUsart = AvrUsart::new(0x130, 0x131, 0x132, 0x134, 0x135, 0x136);
}
