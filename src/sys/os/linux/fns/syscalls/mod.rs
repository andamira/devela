// devela::sys::os::linux::fns::syscalls
//
//!
//
// - https://doc.rust-lang.org/reference/inline-assembly.html
// - https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html

use crate::items;

mod shared_docs;

#[cfg(target_arch = "x86_64")]
items! { mod x86_64; use x86_64 as sys; }
#[cfg(target_arch = "x86")]
items! { mod x86; use x86 as sys; }
#[cfg(target_arch = "arm")]
items! { mod arm; use arm as sys; }
#[cfg(target_arch = "aarch64")]
items! { mod aarch64; use aarch64 as sys; }
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
items! { mod riscv; use riscv as sys; }

// structural access
items! {
    mod doc_inline {
        pub use super::sys::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] #[allow(unused_imports)]
        pub use super::doc_inline::*;
    }
}
