// devela/sys/hw/mcu/board/arduino/nano.rs
//
//! Defines [`BoardArduinoNano`].
//

use crate::{AvrPin, AvrUsart, McuAtmega328p};

#[doc = crate::_tags!(hw namespace)]
/// Classic Arduino Nano board namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/board", struct BoardArduinoNano),
    test_size_of(BoardArduinoNano = 0),
}]
/// See also the [datasheet pdf].
///
/// [datasheet pdf]: https://docs.arduino.cc/resources/datasheets/A000005-datasheet.pdf
#[derive(Debug)]
pub struct BoardArduinoNano;

impl BoardArduinoNano {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;

    /// Built-in LED on digital pin D13 (`PB5` on the ATmega328P).
    pub const LED: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 5);

    /// USART connected to the board's serial RX/TX interface.
    pub const USART: AvrUsart = McuAtmega328p::USART_0;
}
