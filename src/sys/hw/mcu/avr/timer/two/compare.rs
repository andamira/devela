// devela/sys/hw/mcu/avr/timer/two/compare.rs

use crate::AvrTimer2;

/// # Output compare
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer2 {
    /// Returns the output-compare A value.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_a(self) -> u8 {
        unsafe { self.compare_a_reg().read() }
    }
    /// Sets the output-compare A value.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its compare-A register must not be concurrently modified.
    pub unsafe fn set_compare_a(self, value: u8) {
        unsafe { self.compare_a_reg().write(value) }
    }

    /// Returns whether an output-compare A match is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_a_match_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::OCF2A != 0 }
    }
    /// Clears the output-compare A match flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_compare_a_match(self) {
        // OCF2A is write-one-to-clear: do not read-modify-write TIFR2.
        unsafe { self.interrupt_flag_reg().write(Self::OCF2A) };
    }

    /// Returns the output-compare B value.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_b(self) -> u8 {
        unsafe { self.compare_b_reg().read() }
    }
    /// Sets the output-compare B value.
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its compare-B register must not be concurrently modified.
    pub unsafe fn set_compare_b(self, value: u8) {
        unsafe { self.compare_b_reg().write(value) }
    }

    /// Returns whether an output-compare B match is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn compare_b_match_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::OCF2B != 0 }
    }
    /// Clears the output-compare B match flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_compare_b_match(self) {
        // OCF2B is write-one-to-clear: do not read-modify-write TIFR2.
        unsafe { self.interrupt_flag_reg().write(Self::OCF2B) };
    }
}
