// devela::text::grapheme::family_kind

use super::*;
use crate::Char;

#[test]
fn char_grapheme() {
    let ch = 'A';

    // grapheme_chars
    let mut chars = ch.grapheme_chars();
    assert_eq!(chars.next(), Some('A'));
    assert_eq!(chars.next(), None);

    // grapheme_len_bytes
    assert_eq!(ch.grapheme_len_bytes(), 1); // 'A' is 1 byte as a scalar
    assert_eq!('€'.grapheme_len_bytes(), 2); // '€' is 2 bytes in scalar
    assert_eq!('𐍈'.grapheme_len_bytes(), 3); // '𐍈' is 3 bytes as a scalar

    assert_eq!(ch.grapheme_len_utf8(), 1); // 'A' is 1 byte in UTF-8
    assert_eq!('€'.grapheme_len_utf8(), 3); // '€' is 3 bytes in UTF-8
    assert_eq!('𐍈'.grapheme_len_utf8(), 4); // '𐍈' is 4 bytes in UTF-8

    // grapheme_len_chars
    assert_eq!(ch.grapheme_len_chars(), 1);
}

#[test]
fn char_kind() {
    let ch = 'A';
    assert_eq!(ch.grapheme_kind(), GraphemeKind::char);
    assert!(ch.grapheme_is_kind(GraphemeKind::char));
    assert!(!ch.grapheme_is_kind(GraphemeKind::Nonul));
}

#[test]
fn unicode_chars() {
    let test_cases = [
        ('A', 1, 1, 1), // Basic Latin
        ('ß', 1, 2, 1), // Latin-1 Supplement
        ('€', 2, 3, 1), // BMP
        ('𐍈', 3, 4, 1), // Supplementary Plane
    ];
    for (ch, expected_bytes, expected_utf8, expected_chars) in test_cases {
        assert_eq!(ch.grapheme_len_bytes(), expected_bytes);
        assert_eq!(ch.grapheme_len_utf8(), expected_utf8);
        assert_eq!(ch.grapheme_len_chars(), expected_chars);

        let mut iter = ch.grapheme_chars();
        assert_eq!(iter.next(), Some(ch));
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn char_iterator_properties() {
    let ch = '🦀'; // Crab emoji - 4 bytes

    let mut iter = ch.grapheme_chars();

    // Test Iterator trait
    assert_eq!(iter.next(), Some('🦀'));
    assert_eq!(iter.next(), None);

    // Test size_hint if available
    let iter = ch.grapheme_chars();
    let (min, max) = iter.size_hint();
    assert_eq!(min, 1);
    assert_eq!(max, Some(1));
}

#[test]
fn grapheme_trait_consistency() {
    let test_chars = ['a', 'β', '🎉', '字'];

    for ch in test_chars {
        let iter_count = ch.grapheme_chars().count();
        let method_count = ch.grapheme_len_chars();
        assert_eq!(iter_count, method_count);

        assert_eq!(ch.grapheme_len_bytes(), Char(ch as u32).len_bytes());
        assert_eq!(ch.grapheme_len_utf8(), Char(ch).len_utf8());

        assert_eq!(ch.grapheme_kind(), GraphemeKind::char);
    }
}

#[test]
fn trait_object_compatibility() {
    // let _: &dyn Grapheme<4> = &ch; // NOTE: not object safe (yet?)

    fn process_grapheme(g: impl Grapheme) -> usize {
        g.grapheme_len_utf8()
    }
    assert_eq!(process_grapheme('A'), 1);
    assert_eq!(process_grapheme('€'), 3);
}
