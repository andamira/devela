// devela::sys::arch::instructions:aarch64
//
//! Implements processor instructions for AArch64.
//

use crate::{Arch, asm};

/// # AArch64 instructions
impl Arch {
    /// Reads the Virtual Count Register (CNTVCT_EL0) using `mrs cntvct_el0`.
    ///
    /// This is the standard userspace-accessible cycle counter on AArch64.
    pub fn cntvct() -> u64 {
        let cnt;
        unsafe {
            asm!("mrs {}, cntvct_el0", out(reg) cnt, options(nomem, nostack));
        }
        cnt
    }
}
