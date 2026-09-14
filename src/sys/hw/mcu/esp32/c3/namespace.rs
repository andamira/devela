// devela/src/sys/hw/mcu/esp32/c3/namespace.rs
//
//! Defines [`McuEsp32C3`].
//

use crate::{__cfg_item_unsafe_show, Esp32C3Pin, EspI2c, EspReg32, EspUsbSerialJtag, macro_apply};

#[doc = crate::_tags!(hw namespace)]
/// ESP32-C3 microcontroller namespace.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32", struct McuEsp32C3),
    test_size_of(McuEsp32C3 = 0),
}]
/// ESP32-C3 microcontroller namespace.
///
/// # GPIO model
///
/// GPIO0 through GPIO21 correspond to bits 0 through 21 in the GPIO registers exposed here.
///
/// Simple digital output involves two independent pieces of state:
/// - [`GPIO_OUT`] holds the output levels.
/// - [`GPIO_ENABLE`] selects which pins actively drive those levels.
///
/// The `W1TS` and `W1TC` variants modify selected bits
/// without requiring a read-modify-write:
///
/// - `W1TS`: writing `1` sets the selected bit.
/// - `W1TC`: writing `1` clears the selected bit.
///
/// For example, to drive GPIO8 low:
/// ```ignore
/// let gpio8 = 1 << 8;
///
/// unsafe {
///     McuEsp32C3::GPIO_OUT_W1TC.write(gpio8);
///     McuEsp32C3::GPIO_ENABLE_W1TS.write(gpio8);
/// }
/// ```
///
/// Setting the desired output level before enabling the driver
/// avoids briefly driving an unintended level.
///
/// To drive the same output high afterwards:
/// ```ignore
/// unsafe {
///     McuEsp32C3::GPIO_OUT_W1TS.write(gpio8);
/// }
/// ```
///
/// # GPIO configuration
///
/// The output registers are only part of the ESP32-C3 GPIO system.
/// IO MUX configures the pads and their electrical behavior, while the GPIO matrix
/// can route peripheral signals between internal peripherals and external pins.
///
/// Not every GPIO use therefore consists only of writing the registers above.
///
/// [`GPIO_OUT`]: #associatedconstant.GPIO_OUT
/// [`GPIO_ENABLE`]: #associatedconstant.GPIO_ENABLE
#[derive(Debug)]
pub struct McuEsp32C3;

// # GPIO
impl McuEsp32C3 {
    /// GPIO peripheral base address.
    pub const GPIO_BASE: u32 = 0x6000_4000;

    /// GPIO output-value register.
    pub const GPIO_OUT: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0004);

    /// GPIO output set register.
    ///
    /// Writing a `1` sets the corresponding bit in [`GPIO_OUT`](#associatedconstant.GPIO_OUT).
    pub const GPIO_OUT_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0008);

    /// GPIO output clear register.
    ///
    /// Writing a `1` clears the corresponding bit in [`GPIO_OUT`](#associatedconstant.GPIO_OUT).
    pub const GPIO_OUT_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x000c);

    /// GPIO output-enable register.
    pub const GPIO_ENABLE: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0020);

    /// GPIO output-enable set register.
    pub const GPIO_ENABLE_W1TS: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0024);

    /// GPIO output-enable clear register.
    pub const GPIO_ENABLE_W1TC: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x0028);

    /// GPIO input-value register.
    pub const GPIO_IN: EspReg32 = EspReg32::new(Self::GPIO_BASE + 0x003c);
}

// # I2C, UART and USB serial
impl McuEsp32C3 {
    /// External crystal frequency.
    pub const XTAL_HZ: u32 = 40_000_000;

    /// I²C0 peripheral base address.
    pub const I2C0_BASE: u32 = 0x6001_3000;

    /// I²C0 controller.
    pub const I2C0: EspI2c = EspI2c::new(Self::I2C0_BASE);

    /// USB Serial/JTAG peripheral base address.
    pub const USB_SERIAL_JTAG_BASE: u32 = 0x6004_3000;

    /// Native USB Serial/JTAG controller.
    pub const USB_SERIAL_JTAG: EspUsbSerialJtag = EspUsbSerialJtag::new(Self::USB_SERIAL_JTAG_BASE);
}

/* private helpers */
#[allow(dead_code)]
impl McuEsp32C3 {
    const WDT_WKEY: u32 = 0x50D8_3AA1;

    const TIMG0_WDT_CONFIG0: EspReg32 = EspReg32::new(0x6001_F048);
    const TIMG0_WDT_WPROTECT: EspReg32 = EspReg32::new(0x6001_F064);

    const RTC_WDT_CONFIG0: EspReg32 = EspReg32::new(0x6000_8090);
    const RTC_WDT_WPROTECT: EspReg32 = EspReg32::new(0x6000_80A8);
}

#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl McuEsp32C3 {
    /// Enables I²C0, routes it through `sda` and `scl`,
    /// and configures it as a master at `bus_hz` from the 40 MHz XTAL.
    ///
    /// The routed pins are configured for open-drain operation with input
    /// enabled and their weak internal pull-ups enabled.
    ///
    /// # Safety
    /// I²C0 and both GPIOs must not be concurrently configured or accessed.
    pub unsafe fn prepare_i2c0(sda: Esp32C3Pin, scl: Esp32C3Pin, bus_hz: u32) {
        const SYSTEM_PERIP_CLK_EN0: EspReg32 = EspReg32::new(0x600C_0010);
        const SYSTEM_PERIP_RST_EN0: EspReg32 = EspReg32::new(0x600C_0018);
        const I2C0_CLOCK: u32 = 1 << 7;
        const I2C0_RESET: u32 = 1 << 7;

        const SCL_SIGNAL: u32 = 53;
        const SDA_SIGNAL: u32 = 54;

        unsafe {
            let clock = SYSTEM_PERIP_CLK_EN0;
            clock.write(clock.read() | I2C0_CLOCK);

            let reset = SYSTEM_PERIP_RST_EN0;
            reset.write(reset.read() | I2C0_RESET);
            reset.write(reset.read() & !I2C0_RESET);

            Self::prepare_i2c0_pin(sda, SDA_SIGNAL);
            Self::prepare_i2c0_pin(scl, SCL_SIGNAL);

            Self::I2C0.configure_master_xtal(Self::XTAL_HZ, bus_hz);
        }
    }

    unsafe fn prepare_i2c0_pin(pin: Esp32C3Pin, signal: u32) {
        const IO_MUX_GPIO0: u32 = 0x6000_9004;
        const GPIO_PIN0: u32 = McuEsp32C3::GPIO_BASE + 0x74;
        const GPIO_FUNC0_IN: u32 = McuEsp32C3::GPIO_BASE + 0x154;
        const GPIO_FUNC0_OUT: u32 = McuEsp32C3::GPIO_BASE + 0x554;

        const PAD_DRIVER: u32 = 1 << 2;

        const FUN_PULL_DOWN: u32 = 1 << 7;
        const FUN_PULL_UP: u32 = 1 << 8;
        const FUN_INPUT_ENABLE: u32 = 1 << 9;
        const FUN_SELECT_MASK: u32 = 0b111 << 12;
        const FUN_GPIO: u32 = 1 << 12;

        const INPUT_MATRIX_ENABLE: u32 = 1 << 6;

        let gpio = pin.gpio() as u32;

        let io_mux = EspReg32::new(IO_MUX_GPIO0 + gpio * 4);
        let pin_reg = EspReg32::new(GPIO_PIN0 + gpio * 4);
        let input = EspReg32::new(GPIO_FUNC0_IN + signal * 4);
        let output = EspReg32::new(GPIO_FUNC0_OUT + gpio * 4);

        unsafe {
            // I²C idles high.
            pin.set_high();

            // Open-drain pad.
            pin_reg.write(pin_reg.read() | PAD_DRIVER);

            // GPIO function, input enabled, weak internal pull-up,
            // and no internal pull-down.
            io_mux.write(
                (io_mux.read() & !(FUN_SELECT_MASK | FUN_PULL_DOWN))
                    | FUN_GPIO
                    | FUN_INPUT_ENABLE
                    | FUN_PULL_UP,
            );

            // Peripheral output → GPIO.
            //
            // Clearing bits 8..10 also selects:
            // - non-inverted output,
            // - output-enable controlled by the peripheral,
            // - non-inverted output-enable.
            output.write((output.read() & !0x7ff) | signal);

            // GPIO → peripheral input through the GPIO matrix.
            input.write((input.read() & !0x7f) | INPUT_MATRIX_ENABLE | gpio);
        }
    }
}

//
#[macro_apply(__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl McuEsp32C3 {
    /// Disables the watchdog states left active by ROM flash boot.
    ///
    /// Direct boot bypasses the usual SDK startup that handles these
    /// boot-time watchdogs. Unless they are fed, reconfigured, or disabled,
    /// they can reset a long-running application shortly after startup.
    ///
    /// This disables the Timer Group 0 MWDT and the RTC watchdog,
    /// including their flash-boot modes.
    ///
    /// # Safety
    /// No other code may concurrently configure these watchdogs.
    pub unsafe fn disable_boot_watchdogs() {
        const WDT_WRITE_KEY: u32 = 0x50D8_3AA1;

        const WDT_ENABLE: u32 = 1 << 31;

        const MWDT_FLASHBOOT_ENABLE: u32 = 1 << 14;
        const MWDT_CONFIG_UPDATE: u32 = 1 << 22;

        const RWDT_FLASHBOOT_ENABLE: u32 = 1 << 12;

        unsafe {
            // TG0 MWDT: unlock its protected configuration,
            // disable normal and flash-boot operation, commit, and relock.
            Self::TIMG0_WDT_WPROTECT.write(WDT_WRITE_KEY);

            let config = Self::TIMG0_WDT_CONFIG0.read();
            Self::TIMG0_WDT_CONFIG0
                .write((config & !(WDT_ENABLE | MWDT_FLASHBOOT_ENABLE)) | MWDT_CONFIG_UPDATE);
            Self::TIMG0_WDT_WPROTECT.write(0);

            // RTC WDT: a separate watchdog with its own protected control block.
            Self::RTC_WDT_WPROTECT.write(WDT_WRITE_KEY);

            let config = Self::RTC_WDT_CONFIG0.read();
            Self::RTC_WDT_CONFIG0.write(config & !(WDT_ENABLE | RWDT_FLASHBOOT_ENABLE));
            Self::RTC_WDT_WPROTECT.write(0);
        }
    }
}
