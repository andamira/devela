//
//!
//

#[allow(unused_imports, reason = "varied feature-gates")]
use crate::{Arch, asm, spin_loop};

/// # Portable abstractions over architecture-dependent instructions.
#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "avr",
    target_arch = "msp430",
    target_arch = "xtensa",
    all(target_arch = "wasm32", nightly),
))]
#[cfg_attr(
    nightly_doc,
    doc(cfg(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "avr",
        target_arch = "msp430",
        target_arch = "xtensa",
        all(target_arch = "wasm32", nightly),
    )))
)]
impl Arch {
    /// Executes one processor no-operation instruction.
    ///
    /// Unlike [`spin_loop`], this always emits an actual `nop` instruction.
    ///
    /// This does not provide a timing guarantee.
    #[inline(always)]
    pub fn nop() {
        unsafe {
            asm!("nop", options(nomem, nostack, preserves_flags));
        }
    }

    /// Performs one processor-relaxation step for a busy-wait loop.
    ///
    /// Uses [`spin_loop`] where Rust currently provides an architecture-specific
    /// spin-loop hint, and falls back to [`Arch::nop`] otherwise.
    ///
    /// This does not yield to an operating-system scheduler and does not provide
    /// a timing guarantee.
    #[inline(always)]
    pub fn relax() {
        cfg_select! {
            any(
                target_arch = "x86",
                target_arch = "x86_64",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64",
            ) => spin_loop(),

            all(
                target_arch = "arm",
                any(
                    all(target_feature = "v6k", not(target_feature = "thumb-mode")),
                    target_feature = "v6t2",
                    all(target_feature = "v6", target_feature = "mclass"),
                )
            ) => spin_loop(),

            _ => Arch::nop(),
        }
    }
}

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
