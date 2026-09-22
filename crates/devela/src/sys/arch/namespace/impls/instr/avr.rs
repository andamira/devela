//
//! Implements processor instructions for AVR.
//

use crate::{Arch, asm};

/// # AVR instructions
impl Arch {
    /// Returns whether global interrupts are enabled in `SREG`.
    #[inline(always)]
    #[must_use]
    pub fn interrupts_enabled() -> bool {
        let sreg: u8;
        unsafe {
            asm!(
                "in {sreg}, 0x3f",
                sreg = out(reg) sreg,
                options(nomem, nostack, preserves_flags),
            );
        }
        sreg & (1 << 7) != 0
    }

    /// Disables global interrupts by clearing the interrupt-enable bit in `SREG`.
    #[inline(always)]
    pub fn disable_interrupts() {
        unsafe { asm!("cli", options(nostack)) }
    }
    /// Enables global interrupts by setting the interrupt-enable bit in `SREG`.
    ///
    /// # Safety
    /// Every enabled interrupt source must have a valid handler,
    /// and asynchronous interrupt execution must be valid at this point.
    #[inline(always)]
    pub unsafe fn enable_interrupts() {
        unsafe { asm!("sei", options(nostack)) }
    }

    /// Runs `f` with global interrupts disabled, restoring the previous state afterwards.
    ///
    /// If interrupts were already disabled they remain disabled.
    ///
    /// The previous state is sampled and interrupts are disabled in one instruction
    /// sequence so the protected region can also be used for multi-byte snapshots
    /// shared with interrupt handlers.
    pub fn with_interrupts_disabled<R>(f: impl FnOnce() -> R) -> R {
        struct RestoreInterrupts(bool);

        impl Drop for RestoreInterrupts {
            #[inline(always)]
            fn drop(&mut self) {
                if self.0 {
                    // SAFETY: this restores the global state that was active
                    // before entering the critical region.
                    unsafe { Arch::enable_interrupts() };
                }
            }
        }

        let sreg: u8;
        unsafe {
            asm!(
                "in {sreg}, 0x3f",
                "cli",
                sreg = out(reg) sreg,
                options(nostack),
            );
        }
        let restore = RestoreInterrupts(sreg & (1 << 7) != 0);
        let result = f();
        drop(restore);
        result
    }
}
