// devela/src/sys/hw/mcu/avr/atmega328p.rs
//
//! Defines [`McuAtmega328p`].
//

use crate::{AvrPort, AvrTimer0, AvrTimer1, AvrTimer2, AvrUsart};

#[doc = crate::_tags!(hw namespace)]
/// ATmega328P microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr", struct McuAtmega328p),
    test_size_of(McuAtmega328p = 0),
}]
/// The device provides 32 × 8-bit general-purpose working registers,
/// 32 KiB of Flash program memory, 2 KiB of SRAM, and 1 KiB of EEPROM.
/// The CPU working registers are distinct from memory-mapped peripheral
/// registers such as [`AvrReg8`][crate::AvrReg8]. Runtime data and the stack
/// share SRAM, while Flash and EEPROM are separate storage spaces.
///
/// devela currently provides low-level GPIO access,
/// all three timer/counters, and USART0.
///
/// Its timer/counter peripherals comprise:
/// - two 8-bit timers (Timer/Counter0 and Timer/Counter2),
/// - one 16-bit timer (Timer/Counter1).
///
/// GPIO pins use the AVR notation `Pxy`, where `x` identifies the port
/// and `y` the bit within it; for example, `PB5` is port B bit 5.
///
/// See also:
///
/// - [ATmega328P product page]
/// - [ATmega48A/PA/88A/PA/168A/PA/328/P datasheet]
///
/// [ATmega328P product page]: https://www.microchip.com/en-us/product/atmega328p
/// [ATmega48A/PA/88A/PA/168A/PA/328/P datasheet]: https://www.microchip.com/DS40002061
#[derive(Debug)]
pub struct McuAtmega328p;

/// # GPIO
impl McuAtmega328p {
    /// GPIO port B.
    pub const PORT_B: AvrPort = AvrPort::new(0x23, 0x24, 0x25);

    /// GPIO port C.
    pub const PORT_C: AvrPort = AvrPort::new(0x26, 0x27, 0x28);

    /// GPIO port D.
    pub const PORT_D: AvrPort = AvrPort::new(0x29, 0x2A, 0x2B);
}

/// # Timers
impl McuAtmega328p {
    /// Timer/Counter0 peripheral.
    pub const TIMER_0: AvrTimer0 = AvrTimer0::new(0x44, 0x45, 0x46, 0x47, 0x48, 0x6E, 0x35);

    #[rustfmt::skip]
    /// Timer/Counter1 peripheral.
    pub const TIMER_1: AvrTimer1 = AvrTimer1::new(
        0x80, 0x81, 0x82,
        0x84, 0x85,
        0x86, 0x87,
        0x88, 0x89,
        0x8A, 0x8B,
        0x6F, 0x36,
    );

    /// Timer/Counter2 peripheral.
    pub const TIMER_2: AvrTimer2 = AvrTimer2::new(0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB6, 0x70, 0x37);
}

/// # Serial
impl McuAtmega328p {
    /// USART 0 peripheral.
    pub const USART_0: AvrUsart = AvrUsart::new(0xC0, 0xC1, 0xC2, 0xC4, 0xC5, 0xC6);
}
