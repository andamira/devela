//
#![doc = crate::_DOC_PROCESSOR_Z80!()]
#![doc = crate::_doc!(modules: crate::processor; z80)]
#![doc = crate::_doc!(flat:"processor")]
#![doc = crate::_doc!(hr)]
//!
//! This module describes the processor independently of the machine around it.
//! Memory maps, display hardware, keyboard scanning, contention, and other
//! system effects belong to the corresponding computer.
//!
//! # Architecture
//!
//! The Z80 is an 8-bit processor with a 16-bit address space. Its main
//! register set consists of `AF`, `BC`, `DE`, and `HL`, together with the
//! alternate `AF'`, `BC'`, `DE'`, and `HL'` set. It also provides `IX`, `IY`,
//! `SP`, `PC`, the interrupt-vector register `I`, and refresh register `R`.
//!
//! The Z80 also has a distinct I/O address space. devela's port operations
//! use complete `u16` port addresses; the meaning and decoding of those
//! addresses is determined by the surrounding machine.
//!
//! # Timing
//!
//! Z80 instruction timing is expressed in **T-states**, individual processor
//! clock periods. An instruction consists of one or more **machine cycles**
//! (M-cycles), such as opcode fetches, memory accesses, I/O transfers, and
//! interrupt acknowledgement.
//!
//! Instruction tables give nominal T-state counts. A fixed instruction path
//! is deterministic at the processor boundary only while interrupts and
//! externally inserted `WAIT` states are controlled. A complete machine can
//! impose further timing constraints through its memory and I/O hardware.
//!
//! ## Quick reference
//!
//! | instruction | nominal T-states | useful property |
//! |---|---:|---|
//! | `NOP`        | 4      | simplest instruction timing quantum |
//! | `LD r,r`     | 4      | register move |
//! | `LD r,n`     | 7      | immediate byte |
//! | `LD rr,nn`   | 10     | immediate pair |
//! | `INC/DEC r`  | 4      | |
//! | `INC/DEC rr` | 6      | |
//! | `JR e`       | 12     | relative unconditional |
//! | `JR cc,e`    | 12 / 7 | taken / not taken |
//! | `DJNZ e`     | 13 / 8 | taken / finished |
//! | `JP cc,nn`   | 10     | same cost either path |
//! | `CALL nn`    | 17     | |
//! | `RET`        | 10     | |
//! | `OUT (n),A`  | 11     | immediate-port output |
//! | `OUT (C),r`  | 12     | register-port output |
//!
//! # Computers
//!
//! Current Z80 computer support includes
//! `computer::zx::spectrum::ComputerSpectrum48`, the Sinclair ZX Spectrum 48K.
//!
//! # Rust target and code generation
//!
//! Instruction-emitting operations are available when targeting
//! `target_arch = "z80"` with `unsafe_hint`. The current examples use the
//! experimental `z80-unknown-none-elf` target provided by Rust-Z80/LLVM-Z80.
//!
//! On size-sensitive code, prefer `u8` for naturally 8-bit values and `u16`
//! for addresses. Passing small aggregates by reference can be cheaper than
//! passing them by value, and extracting a function can either improve or
//! worsen size depending on register pressure and argument traffic.
//! Compare `opt-level = "s"` and `"z"` when size matters, and inspect the
//! generated Z80 after surprising changes.
//!
//! These are observations about the current experimental toolchain, not
//! requirements of the Z80 architecture.
//!
//! # References
//!
//! See the [Zilog Z80 CPU User Manual pdf],
//! [Z80 Sinclair Wiki], [LLVM-Z80 backend], and [Rust-Z80 fork].
//!
//! [Zilog Z80 CPU User Manual pdf]: https://www.zilog.com/docs/z80/z80cpu_um.pdf
//! [Z80 Sinclair Wiki]: https://sinclair.wiki.zxnet.co.uk/wiki/Z80
//! [LLVM-Z80 backend]: https://github.com/llvm-z80/llvm-z80
//! [Rust-Z80 fork]: https://github.com/llvm-z80/rust-z80
//

crate::mods_in! {
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::ProcessorZ80;
    }
}
