//
//! Defines [`Esp32C3SystemTimer`].
//

use crate::EspReg32;

#[doc = crate::_tags!(hw time)]
/// ESP32-C3 52-bit system timer.
#[doc = crate::_doc_meta!{
    location("mcu/esp32", struct Esp32C3SystemTimer),
    test_size_of(Esp32C3SystemTimer = 4|32; niche !Option),
}]
/// SYSTIMER provides two 52-bit counters (`UNIT0` and `UNIT1`)
/// observed by three comparators.
///
/// The counters use the XTAL-derived `CNT_CLK`. With the ESP32-C3 40 MHz crystal,
/// its alternating fractional division gives an average counter frequency of 16 MHz.
///
/// This initial API exposes coherent observation of `UNIT0`. Reading a live
/// counter value requires an update request which snapshots
/// its high and low parts into readable registers.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Esp32C3SystemTimer(u32);

impl Esp32C3SystemTimer {
    /// Width of each SYSTIMER counter, in bits.
    pub const COUNTER_BITS: u32 = 52;

    /// Average `CNT_CLK` frequency, in hertz.
    pub const COUNTER_HZ: u64 = 16_000_000;

    /// Bit mask covering one complete 52-bit counter value.
    pub const COUNTER_MASK: u64 = (1_u64 << Self::COUNTER_BITS) - 1;
}

/// # Registers
#[rustfmt::skip]
impl Esp32C3SystemTimer {
    /// Creates a system timer from its register base address.
    #[must_use]
    pub const fn new(base: u32) -> Self { Self(base) }

    /// Returns its register base address.
    #[must_use]
    pub const fn base_addr(self) -> u32 { self.0 }

    /// Returns the main configuration register.
    #[must_use]
    pub const fn config_reg(self) -> EspReg32 { EspReg32::new(self.0) }

    /// Returns the UNIT0 update/status register.
    #[must_use]
    pub const fn unit0_op_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x04) }

    /// Returns the high 20 bits of the latched UNIT0 value.
    #[must_use]
    pub const fn unit0_value_hi_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x40) }

    /// Returns the low 32 bits of the latched UNIT0 value.
    #[must_use]
    pub const fn unit0_value_lo_reg(self) -> EspReg32 { EspReg32::new(self.0 + 0x44) }
}

/// # Counter
#[cfg(feature = "unsafe_mmio")]
impl Esp32C3SystemTimer {
    /// Returns the current `UNIT0` counter value.
    ///
    /// Requests an update, waits until the snapshot is valid, then combines
    /// the latched high 20 and low 32 bits.
    ///
    /// # Safety
    /// This must describe the active ESP32-C3 System Timer.
    #[must_use]
    pub unsafe fn unit0_count(self) -> u64 {
        const UPDATE: u32 = 1 << 30;
        const VALUE_VALID: u32 = 1 << 29;

        unsafe {
            let op = self.unit0_op_reg();
            op.write(UPDATE);
            while op.read() & VALUE_VALID == 0 {}
            let lo = self.unit0_value_lo_reg().read() as u64;
            let hi = (self.unit0_value_hi_reg().read() & 0x000f_ffff) as u64;
            hi << 32 | lo
        }
    }
}
