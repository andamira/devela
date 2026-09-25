//
//! Defines [`BoardArduinoNano`].
//

use crate::{AvrPin, AvrUsart, McuAtmega328p};

#[doc = crate::_tags!(hw namespace)]
/// Classic Arduino Nano board namespace.
#[doc = crate::_doc_meta!{
    location("board/avr", struct BoardArduinoNano),
    test_size_of(BoardArduinoNano = 0),
}]
/// The board is based on [`McuAtmega328p`].
///
/// The analog header labels `A0..=A7` correspond to ADC channels `0..=7`.
/// ADC access itself is provided by the associated [`McuAtmega328p`].
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

impl BoardArduinoNano {
    /// The associated Atmega328p microcontroller.
    pub const MCU: McuAtmega328p = McuAtmega328p;
}

/// # Clock
impl BoardArduinoNano {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;
}

/// # Serial
#[allow(missing_docs)]
impl BoardArduinoNano {
    pub const RX: AvrPin = Self::D0;
    pub const TX: AvrPin = Self::D1;

    /// USART connected to the board's serial RX/TX interface.
    pub const USART: AvrUsart = Self::USART_0;

    pub const USART_0: AvrUsart = McuAtmega328p::USART_0;
}

/// # SPI
#[allow(missing_docs)]
impl BoardArduinoNano {
    pub const SS: AvrPin = Self::D10;
    pub const MOSI: AvrPin = Self::D11;
    pub const MISO: AvrPin = Self::D12;
    pub const SCK: AvrPin = Self::D13;
}

/// # I²C
#[allow(missing_docs)]
impl BoardArduinoNano {
    pub const SDA: AvrPin = Self::A4;
    pub const SCL: AvrPin = Self::A5;
}

/// # Digital I/O
#[allow(missing_docs)]
impl BoardArduinoNano {
    pub const D0: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 0);
    pub const D1: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 1);
    pub const D2: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 2);
    pub const D3: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 3);
    pub const D4: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 4);
    pub const D5: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 5);
    pub const D6: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 6);
    pub const D7: AvrPin = AvrPin::new(McuAtmega328p::PORT_D, 7);

    pub const D8: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 0);
    pub const D9: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 1);
    pub const D10: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 2);
    pub const D11: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 3);
    pub const D12: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 4);
    pub const D13: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 5);

    /// Built-in LED on digital pin D13 (`PB5` on the ATmega328P).
    pub const LED: AvrPin = AvrPin::new(McuAtmega328p::PORT_B, 5);
}

/// # Analog-header GPIO
#[allow(missing_docs)]
impl BoardArduinoNano {
    pub const A0: AvrPin = AvrPin::new(McuAtmega328p::PORT_C, 0);
    pub const A1: AvrPin = AvrPin::new(McuAtmega328p::PORT_C, 1);
    pub const A2: AvrPin = AvrPin::new(McuAtmega328p::PORT_C, 2);
    pub const A3: AvrPin = AvrPin::new(McuAtmega328p::PORT_C, 3);
    pub const A4: AvrPin = AvrPin::new(McuAtmega328p::PORT_C, 4);
    pub const A5: AvrPin = AvrPin::new(McuAtmega328p::PORT_C, 5);
}
