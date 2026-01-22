// devela_base_core::text::char::fns

use crate::ASCII_TRANSLIT_BLOCKS;

#[doc = crate::_tags!(text)]
/// Returns the ASCII transliteration of a Unicode scalar code point.
#[doc = crate::_doc_location!("text/char")]
///
/// Returns an empty string if unhandled.
///
/// Converts Unicode to readable ASCII approximations:
/// - `'°'` → `"deg"`
/// - `'α'` → `"a"`
/// - `'©'` → `"(c)"`
/// - `'中'` → `"Zhong "`
/// - `'🚀'` → `""`
/// - ...
#[doc = crate::_doc!(vendor: "transliteration")]
#[must_use]
pub const fn scalar_as_ascii_translit(scalar: u32) -> &'static str {
    let block = (scalar >> 8) as usize;
    let offset = (scalar & 0xFF) as usize;
    if block < ASCII_TRANSLIT_BLOCKS.len() {
        let block_data = ASCII_TRANSLIT_BLOCKS[block];
        if offset < block_data.len() {
            return block_data[offset];
        }
    }
    ""
}

#[cfg(test)]
mod tests {
    use super::scalar_as_ascii_translit as s2a;
    #[test]
    fn translit() {
        assert_eq![s2a('°' as u32), "deg"];
        assert_eq![s2a('α' as u32), "a"];
        assert_eq![s2a('©' as u32), "(c)"];
        assert_eq![s2a('㍱' as u32), "HPA"];
        assert_eq![s2a('㎮' as u32), "rad/s"];
        assert_eq![s2a('中' as u32), "Zhong "];
        assert_eq![s2a('ﬀ' as u32), "ff"];
        assert_eq![s2a('ﬤ' as u32), "k"];
        assert_eq![s2a('�' as u32), ""];
        assert_eq![s2a('🚀' as u32), ""];
    }
}
