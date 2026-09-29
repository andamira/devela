//
//! Defines [`ProcessorZ80`].
//

#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
use crate::asm;

#[doc = crate::_tags!(hw namespace)]
/// Z80 processor namespace.
#[doc = crate::_doc_meta!{
    location("processor/z80", struct ProcessorZ80),
    test_size_of(ProcessorZ80 = 0),
}]
/// # References
///
/// See the [Zilog Z80 CPU User Manual pdf] and the [Z80 Sinclair Wiki].
///
/// [Zilog Z80 CPU User Manual pdf]: https://www.zilog.com/docs/z80/z80cpu_um.pdf
/// [Z80 Sinclair Wiki]: https://sinclair.wiki.zxnet.co.uk/wiki/Z80
#[derive(Debug)]
pub struct ProcessorZ80;

/// # Execution
#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
impl ProcessorZ80 {
    /// Halts execution until an accepted interrupt or NMI resumes the processor.
    ///
    /// If maskable interrupts are disabled and no NMI occurs,
    /// execution may remain halted indefinitely.
    #[inline(always)]
    pub fn halt() {
        unsafe {
            asm!("halt", options(nostack));
        }
    }
}

/// # Interrupts
///
/// The Z80 has a maskable `INT` input and a higher-priority non-maskable
/// `NMI` input. `DI` and `EI` control acceptance of maskable interrupts.
///
/// The three maskable interrupt modes determine how the handler address is obtained:
///
/// - IM 0: the interrupting device supplies an instruction.
/// - IM 1: execution transfers to the fixed address `0x0038`.
/// - IM 2: an indirect vector is formed using the `I` register
///   and a byte supplied during interrupt acknowledgement.
///
/// See the Zilog *Z80 CPU User Manual*.
#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
impl ProcessorZ80 {
    /// Disables maskable interrupts.
    #[inline(always)]
    pub fn disable_interrupts() {
        unsafe {
            asm!("di", options(nostack));
        }
    }
    /// Enables maskable interrupts.
    ///
    /// The Z80 accepts maskable interrupts only after the instruction
    /// following `EI` has executed.
    ///
    /// # Safety
    /// The active machine must have a valid interrupt configuration and handler.
    #[inline(always)]
    pub unsafe fn enable_interrupts() {
        unsafe {
            asm!("ei", options(nostack));
        }
    }
    /// Selects Z80 interrupt mode 0.
    ///
    /// # Safety
    /// The active machine must provide a valid mode-0 interrupt handler.
    #[inline(always)]
    pub unsafe fn interrupt_mode_0() {
        unsafe {
            asm!("im 0", options(nostack));
        }
    }
    /// Selects Z80 interrupt mode 1.
    ///
    /// # Safety
    /// The active machine must provide a valid mode-1 interrupt handler.
    #[inline(always)]
    pub unsafe fn interrupt_mode_1() {
        unsafe {
            asm!("im 1", options(nostack));
        }
    }
    /// Selects Z80 interrupt mode 1.
    ///
    /// # Safety
    /// The active machine must provide a valid mode-2 interrupt handler.
    #[inline(always)]
    pub unsafe fn interrupt_mode_2() {
        unsafe {
            asm!("im 2", options(nostack));
        }
    }
}

/// # I/O
#[cfg(all(target_arch = "z80", feature = "unsafe_hint"))]
impl ProcessorZ80 {
    /// Writes one byte to a Z80 I/O port.
    ///
    /// # Safety
    /// The caller must ensure that accessing `port` is valid
    /// for the currently executing machine and that
    /// its hardware side effects are appropriate.
    #[inline(always)]
    pub unsafe fn io_write(port: u16, value: u8) {
        unsafe {
            asm!("out (c), a", in("bc") port, in("a") value, options(nostack));
        }
    }
    /// Reads one byte from a Z80 I/O port.
    ///
    /// # Safety
    /// The caller must ensure that accessing `port` is valid
    /// for the currently executing machine.
    #[inline(always)]
    pub unsafe fn io_read(port: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!("in a, (c)", in("bc") port, lateout("a") value, options(nostack));
        }
        value
    }
}
