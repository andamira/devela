//
//! Defines [`BoardArduinoDiecimila`].
//

use crate::{AvrAdcInput, AvrPin, AvrUsart, McuAtmega168};

#[doc = crate::_tags!(hw namespace)]
/// Arduino Diecimila board namespace.
#[doc = crate::_doc_meta!{
    location("board/avr", struct BoardArduinoDiecimila),
    test_size_of(BoardArduinoDiecimila = 0),
}]
/// The board is based on [`McuAtmega168`].
///
/// The analog header labels `A0..=A5` correspond to ADC channels `0..=5`.
/// ADC access itself is provided by the associated [`McuAtmega168`].
///
/// See also:
///
/// - [Arduino Diecimila legacy documentation]
/// - [ATmega48A/PA/88A/PA/168A/PA/328/P datasheet]
///
/// [Arduino Diecimila legacy documentation]: https://docs.arduino.cc/retired
/// [ATmega48A/PA/88A/PA/168A/PA/328/P datasheet]: https://www.microchip.com/DS40002061
#[derive(Debug)]
pub struct BoardArduinoDiecimila;

impl BoardArduinoDiecimila {
    /// The associated Atmega168 microcontroller.
    pub const MCU: McuAtmega168 = McuAtmega168;
}

/// # Clock
impl BoardArduinoDiecimila {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;
}

/// # Serial
#[allow(missing_docs)]
impl BoardArduinoDiecimila {
    pub const RX: AvrPin = Self::D0;
    pub const TX: AvrPin = Self::D1;

    /// USART connected to the board's serial RX/TX interface.
    pub const USART: AvrUsart = Self::USART_0;

    pub const USART_0: AvrUsart = McuAtmega168::USART_0;
}

/// # SPI
#[allow(missing_docs)]
impl BoardArduinoDiecimila {
    pub const SS: AvrPin = Self::D10;
    pub const MOSI: AvrPin = Self::D11;
    pub const MISO: AvrPin = Self::D12;
    pub const SCK: AvrPin = Self::D13;
}

/// # I²C
#[allow(missing_docs)]
impl BoardArduinoDiecimila {
    pub const SDA: AvrPin = Self::D18;
    pub const SCL: AvrPin = Self::D19;
}

/// # Digital I/O
#[allow(missing_docs)]
impl BoardArduinoDiecimila {
    pub const D0: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 0);
    pub const D1: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 1);
    pub const D2: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 2);
    pub const D3: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 3);
    pub const D4: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 4);
    pub const D5: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 5);
    pub const D6: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 6);
    pub const D7: AvrPin = AvrPin::new(McuAtmega168::PORT_D, 7);

    pub const D8: AvrPin = AvrPin::new(McuAtmega168::PORT_B, 0);
    pub const D9: AvrPin = AvrPin::new(McuAtmega168::PORT_B, 1);
    pub const D10: AvrPin = AvrPin::new(McuAtmega168::PORT_B, 2);
    pub const D11: AvrPin = AvrPin::new(McuAtmega168::PORT_B, 3);
    pub const D12: AvrPin = AvrPin::new(McuAtmega168::PORT_B, 4);
    pub const D13: AvrPin = AvrPin::new(McuAtmega168::PORT_B, 5);

    pub const D14: AvrPin = AvrPin::new(McuAtmega168::PORT_C, 0);
    pub const D15: AvrPin = AvrPin::new(McuAtmega168::PORT_C, 1);
    pub const D16: AvrPin = AvrPin::new(McuAtmega168::PORT_C, 2);
    pub const D17: AvrPin = AvrPin::new(McuAtmega168::PORT_C, 3);
    pub const D18: AvrPin = AvrPin::new(McuAtmega168::PORT_C, 4);
    pub const D19: AvrPin = AvrPin::new(McuAtmega168::PORT_C, 5);

    /// Built-in LED on digital pin D13 (`PB5` on the ATmega168).
    pub const LED: AvrPin = Self::D13;
}

/// # Analog inputs
#[allow(missing_docs)]
impl BoardArduinoDiecimila {
    pub const A0: AvrAdcInput = McuAtmega168::ADC0;
    pub const A1: AvrAdcInput = McuAtmega168::ADC1;
    pub const A2: AvrAdcInput = McuAtmega168::ADC2;
    pub const A3: AvrAdcInput = McuAtmega168::ADC3;
    pub const A4: AvrAdcInput = McuAtmega168::ADC4;
    pub const A5: AvrAdcInput = McuAtmega168::ADC5;
}
