// devela::sys::os::linux::syscalls
//
//! Linux syscalls.
//
// - https://doc.rust-lang.org/reference/inline-assembly.html
// - https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html
use crate::items;

mod consts; // LINUX_SYS
mod shared_docs;

// re-export arch-specific module as sys:
#[cfg(target_arch = "x86_64")]
items! { mod x86_64; use x86_64 as sys; }
#[cfg(target_arch = "x86")]
items! { mod x86; use x86 as sys; }
#[cfg(target_arch = "arm")]
items! { mod arm; use arm as sys; }
#[cfg(target_arch = "aarch64")]
items! { mod aarch64; use aarch64 as sys; }
#[cfg(any_target_riscv)]
items! { mod riscv; use riscv as sys; }

crate::structural_mods! { // _mods
    _mods {
        pub use super::{consts::*, sys::*};
    }
}
