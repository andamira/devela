// devela/src/sys/hw/mcu/esp32/c3/namespace.rs
//
//! Defines [`McuEsp32C3`].
//

use crate::{EspReg32, EspUsbSerialJtag};

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

    /// USB Serial/JTAG peripheral base address.
    pub const USB_SERIAL_JTAG_BASE: u32 = 0x6004_3000;

    /// Native USB Serial/JTAG controller.
    pub const USB_SERIAL_JTAG: EspUsbSerialJtag = EspUsbSerialJtag::new(Self::USB_SERIAL_JTAG_BASE);
}

impl McuEsp32C3 {
    const WDT_WKEY: u32 = 0x50D8_3AA1;

    const TIMG0_WDT_CONFIG0: EspReg32 = EspReg32::new(0x6001_F048);
    const TIMG0_WDT_WPROTECT: EspReg32 = EspReg32::new(0x6001_F064);

    const RTC_WDT_CONFIG0: EspReg32 = EspReg32::new(0x6000_8090);
    const RTC_WDT_WPROTECT: EspReg32 = EspReg32::new(0x6000_80A8);

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
