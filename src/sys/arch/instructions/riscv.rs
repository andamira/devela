// devela::sys::arch::instructions:riscv
//
//! Implements processor instruction calls for both riscv32 and riscv64.
//

use crate::{Arch, asm};

/// # RISC-V cycle counter access.
#[cfg_attr(nightly_doc, doc(cfg(any_target_arch_riscv)))]
impl Arch {
    /// Reads the cycle counter using the `rdcycle` pseudo-instruction.
    ///
    /// On RV32, this may not be a complete 64-bit read. See RISC-V ISA manual.
    pub fn rdcycle() -> u64 {
        let cycles;
        unsafe {
            asm!("rdcycle {}", out(reg) cycles, options(nomem, nostack));
        }
        cycles
    }
}
