// devela/sys/hw/mcu/esp32/c3/pin.rs
//
//! Defines [`Esp32C3Pin`].
//

#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
use crate::McuEsp32C3;

#[doc = crate::_tags!(hw io)]
/// An ESP32-C3 GPIO pin identified by its GPIO number.
///
/// This provides low-level access to the simple
/// GPIO output latch and output-enable state.
///
/// It does not configure the IO MUX, GPIO matrix, pull resistors, or input path.
/// Those are independent parts of the ESP32-C3 pin configuration.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32/c3", struct Esp32C3Pin),
    test_size_of(Esp32C3Pin = 1|8; niche !Option),
}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Esp32C3Pin(u8);

#[rustfmt::skip]
impl Esp32C3Pin {
    /// Creates a pin from an ESP32-C3 GPIO number.
    ///
    /// # Panics
    /// Panics if `gpio` is greater than 21.
    #[must_use]
    pub const fn new(gpio: u8) -> Self {
        assert!(gpio < 22, "ESP32-C3 GPIO must be in 0..22");
        Self(gpio)
    }

    /// Returns its GPIO number.
    #[must_use]
    pub const fn gpio(self) -> u8 { self.0 }

    /// Returns its one-bit GPIO register mask.
    #[must_use]
    pub const fn mask(self) -> u32 { 1u32 << self.0 }
}

#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl Esp32C3Pin {
    /// Returns whether its output driver is enabled.
    pub unsafe fn is_output_enabled(self) -> bool {
        unsafe { McuEsp32C3::GPIO_ENABLE.read() & self.mask() != 0 }
    }
    /// Enables its output driver, preserving the output latch.
    pub unsafe fn enable_output(self) {
        unsafe { McuEsp32C3::GPIO_ENABLE_W1TS.write(self.mask()) };
    }
    /// Disables its output driver, leaving the pin undriven.
    pub unsafe fn disable_output(self) {
        unsafe { McuEsp32C3::GPIO_ENABLE_W1TC.write(self.mask()) };
    }

    /// Returns whether its output latch is low.
    ///
    /// This reads the configured output level, not the physical pad input.
    pub unsafe fn is_output_low(self) -> bool {
        unsafe { !self.is_output_high() }
    }
    /// Enables its output driver after setting its latch low.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.enable_output();
        }
    }

    /// Returns whether its output latch is high.
    ///
    /// This reads the configured output level, not the physical pad input.
    pub unsafe fn is_output_high(self) -> bool {
        unsafe { McuEsp32C3::GPIO_OUT.read() & self.mask() != 0 }
    }
    /// Enables its output driver after setting its latch high.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.enable_output();
        }
    }

    /// Sets its output latch high.
    ///
    /// When its output driver is enabled, this drives the pin high.
    pub unsafe fn set_high(self) {
        unsafe { McuEsp32C3::GPIO_OUT_W1TS.write(self.mask()) };
    }
    /// Sets its output latch low.
    ///
    /// When its output driver is enabled, this drives the pin low.
    pub unsafe fn set_low(self) {
        unsafe { McuEsp32C3::GPIO_OUT_W1TC.write(self.mask()) };
    }
}
