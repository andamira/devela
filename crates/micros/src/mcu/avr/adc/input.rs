//
//! Defines [`AvrAdcInput`].
//

#[doc = crate::_tags!(hw)]
/// An input selectable by a classic AVR ADC.
#[doc = crate::_doc_meta!{
    location("mcu/avr", struct AvrAdcInput),
    test_size_of(AvrAdcInput = 1|8; niche !Option),
}]
/// This identifies an ADC multiplexer selection, not a GPIO pin.
/// Concrete MCU namespaces define the inputs available on each device.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AvrAdcInput(u8);

impl AvrAdcInput {
    pub(crate) const fn _new(selection: u8) -> Self {
        assert!(selection < 16, "AVR ADC input selection must be in 0..16");
        Self(selection)
    }

    /// Returns the ADC input-multiplexer selection.
    #[must_use]
    pub const fn selection(self) -> u8 {
        self.0
    }
}
