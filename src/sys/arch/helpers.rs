// devela::sys::arch::helpers
//
//! Defines the `ARCH` constant.
//!
//! It returns a string literal with the current architecture.
//! It's useful together with `concat!` and the `doc` attribute.
//

#![allow(unused)]

use crate::items;

#[cfg(target_arch = "x86_64")]
items! { macro_rules! ARCH {()=>{"x86_64"}} pub(crate) use ARCH; }
#[cfg(target_arch = "x86")]
items! { macro_rules! ARCH {()=>{"x86"}} pub(crate) use ARCH; }
#[cfg(target_arch = "arm")]
items! { macro_rules! ARCH {()=>{"arm"}} pub(crate) use ARCH; }
#[cfg(target_arch = "aarch64")]
items! { macro_rules! ARCH {()=>{"aarch64"}} pub(crate) use ARCH; }
#[cfg(target_arch = "riscv32")]
items! { macro_rules! ARCH {()=>{"riscv32"}} pub(crate) use ARCH; }
#[cfg(target_arch = "riscv64")]
items! { macro_rules! ARCH {()=>{"riscv64"}} pub(crate) use ARCH; }
