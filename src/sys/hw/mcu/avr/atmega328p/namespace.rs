// devela/src/sys/hw/mcu/avr/atmega328p/namespace.rs
//
//! Defines [`Atmega328p`].
//
// 35 Register Summary:
// https://onlinedocs.microchip.com/oxy/GUID-0EC909F9-8FB7-46B2-BF4B-05290662B5C3-en-US-12.1.1/GUID-05F454E7-6DE3-4D77-9C23-FEBE05568812.html

use crate::{AvrPort, AvrUsart};

#[doc = crate::_tags!(hw namespace)]
/// ATmega328P microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/avr/atmega328p", struct Atmega328p),
    test_size_of(Atmega328p = 0),
}]
#[derive(Debug)]
pub struct Atmega328p;

impl Atmega328p {
    /// GPIO port B.
    pub const PORT_B: AvrPort = AvrPort::new(0x23, 0x24, 0x25);

    /// GPIO port C.
    pub const PORT_C: AvrPort = AvrPort::new(0x26, 0x27, 0x28);

    /// GPIO port D.
    pub const PORT_D: AvrPort = AvrPort::new(0x29, 0x2A, 0x2B);

    /// USART 0 peripheral.
    pub const USART_0: AvrUsart = AvrUsart::new(0xC0, 0xC1, 0xC2, 0xC4, 0xC5, 0xC6);
}
