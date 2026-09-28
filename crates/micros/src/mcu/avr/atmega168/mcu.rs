//
//! Defines [`McuAtmega168`].
//

use crate::{AvrAdc, AvrAdcInput, AvrPort, AvrTimer0, AvrTimer1, AvrTimer2, AvrUsart};

#[doc = crate::_tags!(hw namespace)]
/// ATmega168 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct McuAtmega168),
    test_size_of(McuAtmega168 = 0),
}]
/// The device provides 32 × 8-bit general-purpose working registers,
/// 16 KiB of Flash program memory, 1 KiB of SRAM, and 512 B of EEPROM.
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
/// - [ATmega168 product page]
/// - [ATmega48A/PA/88A/PA/168A/PA/328/P datasheet]
///
/// [ATmega168 product page]: https://www.microchip.com/en-us/product/atmega168
/// [ATmega48A/PA/88A/PA/168A/PA/328/P datasheet]: https://www.microchip.com/DS40002061
#[derive(Debug)]
#[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("avr")))))]
pub struct McuAtmega168;

/// # GPIO
impl McuAtmega168 {
    /// GPIO port B.
    pub const PORT_B: AvrPort = AvrPort::new(0x23, 0x24, 0x25);

    /// GPIO port C.
    pub const PORT_C: AvrPort = AvrPort::new(0x26, 0x27, 0x28);

    /// GPIO port D.
    pub const PORT_D: AvrPort = AvrPort::new(0x29, 0x2A, 0x2B);
}

/// # Timers
impl McuAtmega168 {
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

/// # Analog
impl McuAtmega168 {
    /// ADC input 0.
    pub const ADC0: AvrAdcInput = AvrAdcInput::_new(0);
    /// ADC input 1.
    pub const ADC1: AvrAdcInput = AvrAdcInput::_new(1);
    /// ADC input 2.
    pub const ADC2: AvrAdcInput = AvrAdcInput::_new(2);
    /// ADC input 3.
    pub const ADC3: AvrAdcInput = AvrAdcInput::_new(3);
    /// ADC input 4.
    pub const ADC4: AvrAdcInput = AvrAdcInput::_new(4);
    /// ADC input 5.
    pub const ADC5: AvrAdcInput = AvrAdcInput::_new(5);
    /// ADC input 6.
    pub const ADC6: AvrAdcInput = AvrAdcInput::_new(6);
    /// ADC input 7.
    pub const ADC7: AvrAdcInput = AvrAdcInput::_new(7);

    /// 10-bit successive-approximation ADC.
    pub const ADC: AvrAdc = AvrAdc::new(0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7E);

    #[must_use]
    /// Returns the 10-bit successive-approximation ADC.
    pub const fn adc(self) -> AvrAdc {
        Self::ADC
    }
}

/// # Serial
impl McuAtmega168 {
    /// USART 0 peripheral.
    pub const USART_0: AvrUsart = AvrUsart::new(0xC0, 0xC1, 0xC2, 0xC4, 0xC5, 0xC6);
}
