// devela::sys::arch::instructions:riscv
//
//! Implements processor instruction calls for both riscv32 and riscv64.
//

use crate::{Arch, asm};

/// # RISC-V instructions
impl Arch {
    /// Reads the cycle counter using the `rdcycle` pseudo-instruction.
    #[cfg(target_arch = "riscv32")]
    pub fn rdcycle() -> u32 {
        let cycles;
        unsafe {
            asm!("rdcycle {}", out(reg) cycles, options(nomem, nostack));
        }
        cycles
    }
    /// Reads the cycle counter using the `rdcycle` pseudo-instruction.
    #[cfg(target_arch = "riscv64")]
    pub fn rdcycle() -> u64 {
        let cycles;
        unsafe {
            asm!("rdcycle {}", out(reg) cycles, options(nomem, nostack));
        }
        cycles
    }
}
