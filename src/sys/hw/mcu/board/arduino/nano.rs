// devela/sys/hw/mcu/board/arduino/nano.rs
//
//! Defines [`ArduinoNano`].
//

#[doc = crate::_tags!(hw namespace)]
/// Classic Arduino Nano board namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/board/arduino", struct ArduinoNano),
    test_size_of(ArduinoNano = 0),
}]
#[derive(Debug)]
pub struct ArduinoNano;

impl ArduinoNano {
    /// Nominal CPU clock frequency in hertz.
    pub const CPU_HZ: u32 = 16_000_000;
}
