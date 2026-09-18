//
//! Defines [`BoardArduinoNano`].
//

use crate::{AvrPin, AvrUsart, McuAtmega328p};

#[doc = crate::_tags!(hw namespace)]
/// Classic Arduino Nano board namespace.
#[doc = crate::_doc_meta!{
    location("board", struct BoardArduinoNano),
    test_size_of(BoardArduinoNano = 0),
}]
/// The board is based on [`McuAtmega328p`].
///
/// See also:
///
/// - [Arduino Nano documentation]
/// - [pinout]
/// - [datasheet]
///
/// [Arduino Nano documentation]: https://docs.arduino.cc/hardware/nano
/// [pinout]: https://docs.arduino.cc/resources/pinouts/A000005-full-pinout.pdf
/// [datasheet]: https://docs.arduino.cc/resources/datasheets/A000005-datasheet.pdf
#[derive(Debug)]
pub struct BoardArduinoNano;

/// # Clock
impl BoardArduinoNano {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;
}

/// # Board I/O
impl BoardArduinoNano {
    /// Built-in LED on digital pin D13 (`PB5` on the ATmega328P).
    pub const LED: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 5);
}

/// # Serial
impl BoardArduinoNano {
    /// USART connected to the board's serial RX/TX interface.
    pub const USART: AvrUsart = McuAtmega328p::USART_0;
}
