// devela/sys/hw/mcu/avr/timer/one/count.rs

use crate::AvrTimer1;

/// # Counter
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer1 {
    /// Returns the current 16-bit counter value.
    ///
    /// The low-byte access snapshots the corresponding high byte
    /// so both bytes represent one logical counter value.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1 16-bit
    /// access may interfere with its shared temporary register.
    #[must_use]
    pub unsafe fn counter(self) -> u16 {
        unsafe { Self::read_16_latched(self.tcnt1l_reg(), self.tcnt1h_reg()) }
    }
    /// Sets the 16-bit counter value.
    ///
    /// # Safety
    /// The timer must belong to the active device. No concurrent Timer1 16-bit
    /// access may interfere with its shared temporary register.
    pub unsafe fn set_counter(self, value: u16) {
        unsafe { Self::write_16(self.tcnt1h_reg(), self.tcnt1l_reg(), value) }
    }
}

/// # Overflow
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer1 {
    /// Returns whether a Timer1 overflow is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn overflow_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::TOV1 != 0 }
    }
    /// Clears the Timer1 overflow flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_overflow(self) {
        // TOV1 is write-one-to-clear: do not read-modify-write TIFR1.
        unsafe { self.interrupt_flag_reg().write(Self::TOV1) };
    }
}
