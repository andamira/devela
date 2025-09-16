// devela::sys::arch::namespace::instructions::portable
//
//!
//

use crate::{Arch, cfg_if};

/// # Portable abstractions over architecture-dependent instructions.
#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_hint"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_hint")))]
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
    /// A portable, best-effort cycle counter for performance measurement.
    ///
    /// # Notes and Warnings
    /// - The behavior and availability is entirely architecture-dependent.
    /// - On x86, this uses the TSC. Ensure an 'invariant TSC' exists.
    /// - On ARM (32-bit and 64-bit), this uses the Virtual Count Register (CNTVCT).
    /// - On RISC-V, this uses the `rdcycle` instruction.
    /// - The value is only meaningful for measuring relative durations on the same core.
    #[inline(always)]
    pub fn cycles() -> u64 {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                Arch::rdtsc()
            } else if #[cfg(target_arch = "arm")] {
                Arch::cntvct()
            } else if #[cfg(target_arch = "aarch64")] {
                Arch::cntvct()
            } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
                Arch::rdcycle()
            } else {
                compile_error!("Cycle counter not implemented for this architecture");
            }
        }
    }
}
