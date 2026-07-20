// devela/src/media/font/_helper.rs
//

pub(crate) struct _Font;

impl _Font {
    pub(crate) const fn valid_scalar(v: u32) -> bool {
        v <= 0x10ffff && !(v >= 0xd800 && v <= 0xdfff)
    }
    pub(crate) const fn read_u16(b: &[u8], o: usize) -> u16 {
        u16::from_le_bytes([b[o], b[o + 1]])
    }
    pub(crate) const fn read_i16(b: &[u8], o: usize) -> i16 {
        i16::from_le_bytes([b[o], b[o + 1]])
    }
    pub(crate) const fn read_u32(b: &[u8], o: usize) -> u32 {
        u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
    }
}
