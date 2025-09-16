// devela::sys::arch::namespace::instructions
//
//! Architecture specific instructions.
//

mod portable; // portable abstractions on top

#[cfg(any(doc, any(target_arch = "x86", target_arch = "x86_64")))]
#[cfg_attr(nightly_doc, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
mod x86_or_x86_64;
// #[cfg(any(doc, target_arch = "x86_64"))]
// #[cfg_attr(nightly_doc, doc(cfg(target_arch = "x86_64")))]
// mod x86_64;
// #[cfg(any(doc, target_arch = "x86"))]
// #[cfg_attr(nightly_doc, doc(cfg(target_arch = "x86")))]
// mod x86;

#[cfg(any(doc, target_arch = "arm"))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "arm")))]
mod arm;

#[cfg(any(doc, target_arch = "aarch64"))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "aarch64")))]
mod aarch64;

#[cfg(any(doc, any(target_arch = "riscv32", target_arch = "riscv64")))]
#[cfg_attr(nightly_doc, doc(cfg(any(target_arch = "riscv32", target_arch = "riscv64"))))]
mod riscv;
