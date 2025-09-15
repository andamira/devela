// devela::sys::arch::instructions
//
//! Architecture specific instructions.
//

// WIP
#[cfg(any(doc, any(target_arch = "x86_64", target_arch = "x86_64")))]
mod x86_x86_64;
// #[cfg(any(doc, target_arch = "x86_64"))]
// mod x86_64;
// #[cfg(any(doc, target_arch = "x86"))]
// mod x86;
#[cfg(any(doc, target_arch = "arm"))]
mod arm;
#[cfg(any(doc, target_arch = "aarch64"))]
mod aarch64;
#[cfg(any(doc, any(target_arch = "riscv32", target_arch = "riscv64")))]
mod riscv;

crate::structural_mods! { // _mods
    _mods {
        #[cfg(any(doc, any(target_arch = "x86_64", target_arch = "x86_64")))]
        pub use super::x86_x86_64::*;
        // #[cfg(target_arch = "x86_64")]
        // pub use super::x86_64::*;
        // #[cfg(target_arch = "x86")]
        // pub use super::x86::*;
        #[cfg(target_arch = "arm")]
        pub use super::arm::*;
        #[cfg(target_arch = "aarch64")]
        pub use super::aarch64::*;
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        pub use super::riscv::*;
    }
}
