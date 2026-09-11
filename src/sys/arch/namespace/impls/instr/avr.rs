// devela/src/sys/arch/namespace/impls/instr/avr.rs
//
//! Implements processor instructions for AVR.
//

use crate::{Arch, asm};

/// # AVR instructions
impl Arch {
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
}
