// devela/sys/hw/mcu/avr/timer/two/clock.rs

use crate::AvrTimer2;

/// # Clock
#[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
impl AvrTimer2 {
    /// Returns whether Timer2 is using asynchronous clocking.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn is_asynchronous(self) -> bool {
        unsafe { self.asynchronous_status_reg().read() & Self::AS2 != 0 }
    }

    /// Returns whether any asynchronous Timer2 register update is pending.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    #[must_use]
    pub unsafe fn asynchronous_update_pending(self) -> bool {
        unsafe { self.asynchronous_status_reg().read() & Self::UPDATE_BUSY != 0 }
    }

    /// Waits until all pending asynchronous Timer2 register updates complete.
    ///
    /// This waits for synchronization of `TCNT2`, `OCR2A`, `OCR2B`, `TCCR2A`, and `TCCR2B`.
    /// It can wait indefinitely if the asynchronous Timer2 clock is not running.
    ///
    /// # Safety
    /// The timer must belong to the active device.
    pub unsafe fn wait_for_asynchronous_update(self) {
        while unsafe { self.asynchronous_update_pending() } {}
    }
}
