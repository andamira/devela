//
//! Defines [`AvrUsart`].
//

use crate::AvrReg8;

#[doc = crate::_tags!(hw io)]
/// An AVR USART described by its control, baud-rate, and data registers.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct AvrUsart),
    test_size_of(AvrUsart = 12|96; niche !Option),
}]
/// The operational API provides asynchronous 8N1 configuration
/// and byte-oriented polling for reception and transmission.
///
/// Values can safely be copied and inspected. Operations that access the
/// described registers are unsafe because the addresses must correspond to
/// the active device and access must respect the peripheral's hardware state.
///
/// # Methods
///
/// The operational API is grouped by USART function:
///
/// - [Configuration](#configuration) — selects framing, baud rate,
///   and which sides of the USART are enabled.
/// - [Receiver](#receiver) — controls reception, reports readiness,
///   and reads bytes using blocking or non-blocking operations.
/// - [Transmitter](#transmitter) — controls transmission, reports readiness,
///   and queues bytes using blocking or non-blocking operations.
///
/// Semantic and datasheet register accessors are also provided for lower-level use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrUsart {
    pub(super) ucsra: AvrReg8,
    pub(super) ucsrb: AvrReg8,
    pub(super) ucsrc: AvrReg8,
    pub(super) ubrrl: AvrReg8,
    pub(super) ubrrh: AvrReg8,
    pub(super) udr: AvrReg8,
}

#[rustfmt::skip]
impl AvrUsart {
    /// Creates an AVR USART from its register data-space addresses.
    #[must_use]
    pub const fn new(
        ucsra: u16,
        ucsrb: u16,
        ucsrc: u16,
        ubrrl: u16,
        ubrrh: u16,
        udr: u16,
    ) -> Self {
        Self {
            ucsra: AvrReg8::new(ucsra),
            ucsrb: AvrReg8::new(ucsrb),
            ucsrc: AvrReg8::new(ucsrc),
            ubrrl: AvrReg8::new(ubrrl),
            ubrrh: AvrReg8::new(ubrrh),
            udr: AvrReg8::new(udr),
        }
    }
    /// Returns all six registers in constructor order.
    #[must_use]
    pub const fn into_parts(self) -> [AvrReg8; 6] {
        [
            self.ucsra,
            self.ucsrb,
            self.ucsrc,
            self.ubrrl,
            self.ubrrh,
            self.udr,
        ]
    }
}
