//
//! Defines [`Esp32S3Pin`].
//

#[cfg(feature = "unsafe_mmio")]
use crate::is;
use crate::{EspReg32, McuEsp32S3};

#[doc = crate::_tags!(hw io)]
/// An ESP32-S3 GPIO pin identified by its GPIO number.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct Esp32S3Pin),
    test_size_of(Esp32S3Pin = 1|8; niche !Option),
}]
/// This provides low-level access to the GPIO output latch, output-enable
/// state, and physical input level.
///
/// The ESP32-S3 GPIO register file is split into two banks:
/// GPIO0..31 use the primary registers and GPIO32 and above use the
/// corresponding `*1` registers.
///
/// This type does not yet configure the IO MUX, GPIO matrix, pull resistors,
/// drive strength, or peripheral routing. Those are independent parts of
/// ESP32-S3 pin configuration and will be added as their contracts are needed.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Esp32S3Pin(u8);

#[rustfmt::skip]
impl Esp32S3Pin {
    /// Creates a pin from an ESP32-S3 GPIO number.
    ///
    /// # Panics
    /// Panics unless `gpio` is in `0..=21` or `26..=48`.
    #[must_use]
    pub const fn new(gpio: u8) -> Self {
        assert!(
            gpio <= 21 || (gpio >= 26 && gpio <= 48),
            "ESP32-S3 GPIO must be in 0..22 or 26..49",
        );
        Self(gpio)
    }

    /// Returns its GPIO number.
    #[must_use]
    pub const fn gpio(self) -> u8 { self.0 }

    /// Returns its one-bit mask within the corresponding GPIO register bank.
    #[must_use]
    pub const fn mask(self) -> u32 { 1u32 << (self.0 & 31) }
}

/* private pin-routing registers */

#[allow(dead_code, reason = "safe helpers used by unsafe-gated code")]
impl Esp32S3Pin {
    const IO_MUX_GPIO0: u32 = 0x6000_9004;
    const GPIO_PIN0: u32 = McuEsp32S3::GPIO_BASE + 0x74;
    const GPIO_FUNC0_IN: u32 = McuEsp32S3::GPIO_BASE + 0x154;
    const GPIO_FUNC0_OUT: u32 = McuEsp32S3::GPIO_BASE + 0x554;

    #[must_use]
    const fn io_mux_reg(self) -> EspReg32 {
        EspReg32::new(Self::IO_MUX_GPIO0 + self.0 as u32 * 4)
    }
    #[must_use]
    const fn pin_config_reg(self) -> EspReg32 {
        EspReg32::new(Self::GPIO_PIN0 + self.0 as u32 * 4)
    }
    #[must_use]
    const fn matrix_input_reg(signal: u8) -> EspReg32 {
        EspReg32::new(Self::GPIO_FUNC0_IN + signal as u32 * 4)
    }
    #[must_use]
    const fn matrix_output_reg(self) -> EspReg32 {
        EspReg32::new(Self::GPIO_FUNC0_OUT + self.0 as u32 * 4)
    }
}

/* private register selection */

#[cfg(feature = "unsafe_mmio")]
impl Esp32S3Pin {
    #[must_use]
    const fn output_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_OUT, McuEsp32S3::GPIO_OUT1 }
    }
    #[must_use]
    const fn output_set_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_OUT_W1TS, McuEsp32S3::GPIO_OUT1_W1TS }
    }
    #[must_use]
    const fn output_clear_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_OUT_W1TC, McuEsp32S3::GPIO_OUT1_W1TC }
    }
    #[must_use]
    const fn enable_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_ENABLE, McuEsp32S3::GPIO_ENABLE1 }
    }
    #[must_use]
    const fn enable_set_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_ENABLE_W1TS, McuEsp32S3::GPIO_ENABLE1_W1TS }
    }
    #[must_use]
    const fn enable_clear_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_ENABLE_W1TC, McuEsp32S3::GPIO_ENABLE1_W1TC }
    }
    #[must_use]
    const fn input_reg(self) -> EspReg32 {
        is! { self.0 < 32, McuEsp32S3::GPIO_IN, McuEsp32S3::GPIO_IN1 }
    }
}

/* private peripheral-routing helpers */

#[cfg(feature = "unsafe_mmio")]
impl Esp32S3Pin {
    /// Selects the GPIO function and configures this pad for an input-enabled,
    /// open-drain signal with a weak internal pull-up.
    ///
    /// This is intended for peripheral signals such as I²C
    /// that use the GPIO matrix.
    ///
    /// # Safety
    /// This pin must not be concurrently configured or accessed.
    pub(crate) unsafe fn configure_open_drain_pullup(self) {
        const PAD_DRIVER: u32 = 1 << 2;
        const FUN_PULL_DOWN: u32 = 1 << 7;
        const FUN_PULL_UP: u32 = 1 << 8;
        const FUN_INPUT_ENABLE: u32 = 1 << 9;
        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;
        unsafe {
            self.set_high(); // Open-drain buses idle high.
            let pin = self.pin_config_reg();
            pin.write(pin.read() | PAD_DRIVER);
            let mux = self.io_mux_reg();
            mux.write(
                (mux.read() & !(FUN_SELECT_MASK | FUN_PULL_DOWN))
                    | FUN_GPIO
                    | FUN_INPUT_ENABLE
                    | FUN_PULL_UP,
            );
        }
    }

    /// Routes a peripheral output signal through the GPIO matrix to this pin.
    ///
    /// Peripheral control of output-enable is preserved.
    ///
    /// # Safety
    /// This pin and matrix output route must not be concurrently configured.
    pub(crate) unsafe fn matrix_route_output(self, signal: u8) {
        const OUT_SELECT_MASK: u32 = 0x1ff;
        const OUT_INVERT: u32 = 1 << 9;
        const OEN_SELECT: u32 = 1 << 10;
        const OEN_INVERT: u32 = 1 << 11;
        unsafe {
            let output = self.matrix_output_reg();
            output.write(
                (output.read() & !(OUT_SELECT_MASK | OUT_INVERT | OEN_SELECT | OEN_INVERT))
                    | signal as u32,
            );
        }
    }

    /// Routes this pin through the GPIO matrix to a peripheral input signal.
    ///
    /// # Safety
    /// This pin and matrix input route must not be concurrently configured.
    pub(crate) unsafe fn matrix_route_input(self, signal: u8) {
        const INPUT_MATRIX_ENABLE: u32 = 1 << 7;
        const INPUT_INVERT: u32 = 1 << 6;
        const INPUT_SELECT_MASK: u32 = 0x3f;
        unsafe {
            let input = Self::matrix_input_reg(signal);
            input.write(
                (input.read() & !(INPUT_MATRIX_ENABLE | INPUT_INVERT | INPUT_SELECT_MASK))
                    | INPUT_MATRIX_ENABLE
                    | self.gpio() as u32,
            );
        }
    }
}

#[cfg(feature = "unsafe_mmio")]
impl Esp32S3Pin {
    /// Configures this pad as a simple push-pull GPIO output.
    ///
    /// Selects the GPIO IO-MUX function and routes this pin's output latch
    /// through the GPIO matrix, with output enable controlled by the GPIO
    /// output-enable register.
    ///
    /// This does not change the output latch or enable the output driver.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. The pin and its GPIO
    /// matrix output route must not be concurrently configured, and selecting
    /// simple GPIO output must be valid for the connected circuit.
    pub unsafe fn configure_output(self) {
        const PAD_DRIVER: u32 = 1 << 2;
        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;
        const OUT_SELECT_MASK: u32 = 0x1ff;
        const OUT_INVERT: u32 = 1 << 9;
        const OEN_SELECT: u32 = 1 << 10;
        const OEN_INVERT: u32 = 1 << 11;
        const GPIO_OUT_SIGNAL: u32 = 256;

        unsafe {
            // Simple digital output is push-pull rather than open-drain.
            let pin = self.pin_config_reg();
            pin.write(pin.read() & !PAD_DRIVER);

            // Select GPIO_OUT[n] and GPIO_ENABLE[n].
            let output = self.matrix_output_reg();
            output.write(
                (output.read() & !(OUT_SELECT_MASK | OUT_INVERT | OEN_SELECT | OEN_INVERT))
                    | GPIO_OUT_SIGNAL
                    | OEN_SELECT,
            );

            // ESP32-S3 uses IO-MUX function 1 for ordinary GPIO.
            let mux = self.io_mux_reg();
            mux.write((mux.read() & !FUN_SELECT_MASK) | FUN_GPIO);
        }
    }

    /// Returns whether its output driver is enabled.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device.
    pub unsafe fn is_output_enabled(self) -> bool {
        unsafe { self.enable_reg().read() & self.mask() != 0 }
    }

    /// Enables its output driver, preserving the output latch.
    ///
    /// This does not alter IO-MUX or GPIO-matrix routing.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. The pin's output-enable
    /// state must not be concurrently modified, and driving the pin at its
    /// current latch level must be valid for the current routing and circuit.
    pub unsafe fn enable_output(self) {
        unsafe { self.enable_set_reg().write(self.mask()) };
    }
    /// Disables its output driver, leaving the pin undriven.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device and the pin's
    /// output-enable state must not be concurrently modified.
    pub unsafe fn disable_output(self) {
        unsafe { self.enable_clear_reg().write(self.mask()) };
    }

    /// Returns whether its output latch is low.
    ///
    /// This reads the configured output level, not the physical pad input.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device.
    pub unsafe fn is_output_low(self) -> bool {
        unsafe { !self.is_output_high() }
    }
    /// Sets its output latch low and enables its output driver.
    ///
    /// This does not configure IO-MUX or GPIO-matrix routing.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. The pin state must not
    /// be concurrently modified and driving it low must be valid for the
    /// current routing and connected circuit.
    pub unsafe fn set_output_low(self) {
        unsafe {
            self.set_low();
            self.configure_output();
            self.enable_output();
        }
    }

    /// Returns whether its output latch is high.
    ///
    /// This reads the configured output level, not the physical pad input.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device.
    pub unsafe fn is_output_high(self) -> bool {
        unsafe { self.output_reg().read() & self.mask() != 0 }
    }
    /// Sets its output latch high and enables its output driver.
    ///
    /// This does not configure IO-MUX or GPIO-matrix routing.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. The pin state must not
    /// be concurrently modified and driving it high must be valid for the
    /// current routing and connected circuit.
    pub unsafe fn set_output_high(self) {
        unsafe {
            self.set_high();
            self.configure_output();
            self.enable_output();
        }
    }

    /// Sets its output latch high.
    ///
    /// When its output driver and routing are configured, this drives the pin high.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. If its output driver is
    /// enabled, driving the pin high must be valid for the connected circuit.
    pub unsafe fn set_high(self) {
        unsafe { self.output_set_reg().write(self.mask()) };
    }
    /// Sets its output latch low.
    ///
    /// When its output driver and routing are configured, this drives the pin low.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. If its output driver is
    /// enabled, driving the pin low must be valid for the connected circuit.
    pub unsafe fn set_low(self) {
        unsafe { self.output_clear_reg().write(self.mask()) };
    }

    /// Toggles its output latch.
    ///
    /// This is a compound read-then-write operation rather than an atomic
    /// write-one-to-toggle operation.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. The pin's output latch
    /// must not be concurrently modified. If its output driver is enabled,
    /// driving the opposite level must be valid for the connected circuit.
    pub unsafe fn toggle(self) {
        unsafe {
            is! { self.is_output_high(), self.set_low(), self.set_high() }
        }
    }

    /// Reads the physical GPIO input level.
    ///
    /// # Safety
    /// This must execute on the active ESP32-S3 device. The pad input path must
    /// be configured appropriately for meaningful results.
    pub unsafe fn is_high(self) -> bool {
        unsafe { self.input_reg().read() & self.mask() != 0 }
    }
    /// Reads whether the physical GPIO input level is low.
    ///
    /// # Safety
    /// This has the same requirements as [`is_high`](Self::is_high).
    pub unsafe fn is_low(self) -> bool {
        unsafe { !self.is_high() }
    }
}
