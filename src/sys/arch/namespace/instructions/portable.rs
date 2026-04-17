// devela::sys::arch::namespace::instructions::portable
//
//!
//

use crate::Arch;

/// # Portable abstractions over architecture-dependent instructions.
#[cfg(any_target_arch_linux)]
#[cfg_attr(
    nightly_doc,
    doc(cfg(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64",
    )))
)]
impl Arch {
    /// A portable, best-effort raw cycle counter for performance measurement.
    ///
    /// # Notes and Warnings
    /// - The behavior and availability is entirely architecture-dependent.
    /// - On x86, this uses the TSC. Ensure an 'invariant TSC' exists.
    /// - On ARM (32-bit and 64-bit), this uses the Virtual Count Register (CNTVCT).
    /// - On RISC-V, this uses the `rdcycle` instruction.
    /// - The value is only meaningful for measuring relative durations on the same core.
    #[inline(always)]
    pub fn cycles() -> u64 {
        cfg_select! {
                 any(target_arch = "x86", target_arch = "x86_64") => Arch::rdtsc(),
                                              target_arch = "arm" => Arch::cntvct(),
                                          target_arch = "aarch64" => Arch::cntvct(),
            any(target_arch = "riscv32", target_arch = "riscv64") => Arch::rdcycle().into(),
            _ => compile_error!("Cycle counter not implemented for this architecture"),
        }
    }
}
