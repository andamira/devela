// devela::sys::arch::instructions:arm
//
//! Implements processor instruction calls for arm.
//

use crate::{Arch, asm};

/// # ARM cycle counter access.
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "arm")))]
impl Arch {
    /// Reads the Virtual Count Register (CNTVCT) using the appropriate instruction.
    ///
    /// This provides a 64-bit cycle counter that is virtualized and intended for
    /// timing purposes. It is the ARM equivalent of x86's TSC.
    ///
    /// # Availability
    /// Requires ARM Virtualization Extensions. Common on Cortex-A7, A15, A53, A57, and later.
    ///
    /// On modern Linux systems, this register is typically available from user space.
    pub fn cntvct() -> u64 {
        let [low, high]: [u32; 2];
        unsafe {
            // ARMv7 requires two 32-bit reads to get the full 64-bit value.
            // The register is defined as 64-bit but accessed as two 32-bit registers.
            asm!(
                "mrrc p15, 1, {}, {}, c14",
                out(reg) low,
                out(reg) high,
                options(nomem, nostack, preserves_flags)
            );
        }
        ((high as u64) << 32) | (low as u64)
    }
}
