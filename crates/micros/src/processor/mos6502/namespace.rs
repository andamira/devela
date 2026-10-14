//
//! Defines [`ProcessorMos6502`].
//

#[doc = crate::_tags!(hw namespace)]
/// Original NMOS MOS Technology 6502 processor namespace.
#[doc = crate::_doc_meta!{
    location("processor/mos6502", struct ProcessorMos6502),
    test_size_of(ProcessorMos6502 = 0),
}]
/// The vectors below are addresses in the CPU's 16-bit address space.
///
/// The machine determines which ROM or RAM is mapped at these addresses.
#[derive(Debug)]
pub struct ProcessorMos6502;

impl ProcessorMos6502 {
    /// The data bus is 8 bits wide.
    pub const DATA_BITS: u8 = 8;
    /// The address bus is 16 bits wide.
    pub const ADDRESS_BITS: u8 = 16;
    /// First address of the processor's zero page.
    pub const ZERO_PAGE: u16 = 0x0000;
    /// First address of the hardware stack page (`0x0100..=0x01FF`).
    pub const STACK_PAGE: u16 = 0x0100;
    /// Address of the NMI vector (two bytes, little-endian).
    pub const NMI_VECTOR: u16 = 0xFFFA;
    /// Address of the reset vector (two bytes, little-endian).
    pub const RESET_VECTOR: u16 = 0xFFFC;
    /// Address of the IRQ/BRK vector (two bytes, little-endian).
    pub const IRQ_VECTOR: u16 = 0xFFFE;
}
