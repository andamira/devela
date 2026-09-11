// devela/src/sys/arch/namespace/instr/_.rs
//
//! Architecture specific instructions.
//

mod portable; // portable abstractions on top

// NOTE: using x86_64 as the documentation aggregation target

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(nightly_doc, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
mod x86_or_x86_64;
// #[cfg(target_arch = "x86_64")]
// #[cfg_attr(nightly_doc, doc(cfg(target_arch = "x86_64")))]
// mod x86_64;
// #[cfg(target_arch = "x86")]
// #[cfg_attr(nightly_doc, doc(cfg(target_arch = "x86")))]
// mod x86;

// #[crate::macro_apply(crate::_arch_doc(target_arch = "arm"))]
#[cfg(any(target_arch = "arm", all(doc, target_arch = "x86_64")))] // WAIT:1.99:apply
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "arm")))]
mod arm;

// #[crate::macro_apply(crate::_arch_doc(target_arch = "aarch64"))]
#[cfg(any(target_arch = "aarch64", all(doc, target_arch = "x86_64")))] // WAIT:1.99:apply
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "aarch64")))]
mod aarch64;

// #[crate::macro_apply(crate::_arch_doc(any(target_arch = "riscv32", target_arch = "riscv64")))]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", all(doc, target_arch = "x86_64")))]
#[cfg_attr(nightly_doc, doc(cfg(any(target_arch = "riscv32", target_arch = "riscv64"))))]
mod riscv;

#[cfg(any(target_arch = "avr", all(doc, target_arch = "x86_64")))] // WAIT:1.99:apply
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "avr")))]
mod avr; // AVR processor instructions
