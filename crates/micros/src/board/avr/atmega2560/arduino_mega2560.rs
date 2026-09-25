//
//! Defines [`BoardArduinoMega2560`].
//

use crate::{AvrPin, AvrUsart, McuAtmega2560};

#[doc = crate::_tags!(hw namespace)]
/// Classic Arduino Mega 2560 board namespace.
#[doc = crate::_doc_meta!{
    location("board/avr", struct BoardArduinoMega2560),
    test_size_of(BoardArduinoMega2560 = 0),
}]
/// The board is based on [`McuAtmega2560`].
///
///
/// See also:
///
/// - [Arduino Mega 2560 documentation]
/// - [pinout]
/// - [datasheet]
///
/// [Arduino Mega 2560 documentation]: https://docs.arduino.cc/hardware/mega-2560/
/// [pinout]: https://docs.arduino.cc/resources/pinouts/A000067-full-pinout.pdf
/// [datasheet]: https://docs.arduino.cc/resources/datasheets/A000067-datasheet.pdf
#[derive(Debug)]
pub struct BoardArduinoMega2560;

impl BoardArduinoMega2560 {
    /// The associated Atmega2560 microcontroller.
    pub const MCU: McuAtmega2560 = McuAtmega2560;
}

/// # Clock
impl BoardArduinoMega2560 {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;
}

/// # Board I/O
impl BoardArduinoMega2560 {
    /// Built-in LED on digital pin D13 (`PB7` on the ATmega2560).
    pub const LED: AvrPin = AvrPin::new(McuAtmega2560::PORT_B, 7);
}

/// # Serial
impl BoardArduinoMega2560 {
    /// Primary USB-connected USART.
    pub const USART: AvrUsart = Self::USART_0;

    /// Primary USART connected to D0/RX0 and D1/TX0.
    pub const USART_0: AvrUsart = McuAtmega2560::USART_0;
    /// USART connected to D19/RX1 and D18/TX1.
    pub const USART_1: AvrUsart = McuAtmega2560::USART_1;
    /// USART connected to D17/RX2 and D16/TX2.
    pub const USART_2: AvrUsart = McuAtmega2560::USART_2;
    /// USART connected to D15/RX3 and D14/TX3.
    pub const USART_3: AvrUsart = McuAtmega2560::USART_3;
}
