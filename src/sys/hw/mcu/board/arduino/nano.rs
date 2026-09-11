// devela/sys/hw/mcu/board/arduino/nano.rs
//
//! Defines [`ArduinoNano`].
//

use crate::{Atmega328p, AvrPin};

#[doc = crate::_tags!(hw namespace)]
/// Classic Arduino Nano board namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/board/arduino", struct ArduinoNano),
    test_size_of(ArduinoNano = 0),
}]
/// See also the [datasheet pdf].
///
/// [datasheet pdf]: https://docs.arduino.cc/resources/datasheets/A000005-datasheet.pdf
#[derive(Debug)]
pub struct ArduinoNano;

impl ArduinoNano {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;

    /// Built-in LED on digital pin D13 (`PB5` on the ATmega328P).
    pub const LED: AvrPin = AvrPin::new(Atmega328p::PORT_B, 5);
}
