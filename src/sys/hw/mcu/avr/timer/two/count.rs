/// # Counter
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl crate::AvrTimer2 {
    /// Returns the current 8-bit counter value.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn counter(self) -> u8 {
        unsafe { self.counter_reg().read() }
    }

    /// Sets the 8-bit counter value.
    ///
    /// In asynchronous operation, the write is synchronized into the Timer2
    /// clock domain. Before writing `TCNT2` again, wait for the previous update
    /// to complete with [`wait_for_asynchronous_update`](#method.wait_for_asynchronous_update).
    ///
    /// # Safety
    /// The timer must belong to the active device
    /// and its counter register must not be concurrently modified.
    /// In asynchronous operation, no previous `TCNT2` update may still be pending.
    pub unsafe fn set_counter(self, value: u8) {
        unsafe { self.counter_reg().write(value) }
    }
}

/// # Overflow
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl crate::AvrTimer2 {
    /// Returns whether a Timer2 overflow is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn overflow_pending(self) -> bool {
        unsafe { self.interrupt_flag_reg().read() & Self::TOV2 != 0 }
    }
    /// Clears the Timer2 overflow flag.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn clear_overflow(self) {
        // TOV2 is write-one-to-clear: do not read-modify-write TIFR2.
        unsafe { self.interrupt_flag_reg().write(Self::TOV2) };
    }
}
