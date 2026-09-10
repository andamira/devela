// devela/src/sys/hw/mcu/avr/atmega328p/namespace.rs
//
//! Defines [`Atmega328p`].
//

use crate::AvrPort;

#[doc = crate::_tags!(hw namespace)]
/// ATmega328P microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("hw/mcu/avr/atmega328p", struct Atmega328p),
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
}
