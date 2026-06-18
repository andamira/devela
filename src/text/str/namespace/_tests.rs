// devela/src/text/str/namespace/_tests.rs

// IMPROVE: add tests for missing methods from: range, split, take.

use crate::{GraphemeBoundary, GraphemeIter, Str, const_assert};

#[test]
fn str_strip_prefix_suffix_circumfix() {
    assert_eq!(Str::strip_prefix("foo:bar", "foo:"), Some("bar"));
    assert_eq!(Str::strip_prefix("foo:bar", "bar"), None);
    assert_eq!(Str::strip_suffix("foo.rs", ".rs"), Some("foo"));
    assert_eq!(Str::strip_suffix("foo.rs", ".toml"), None);
    assert_eq!(Str::strip_circumfix("(x)", "(", ")"), Some("x"));
    assert_eq!(Str::strip_circumfix("a", "a", "a"), None);
    assert_eq!(Str::strip_circumfix("aa", "a", "a"), Some(""));
}
#[test]
fn str_strip_char_prefix_suffix_circumfix() {
    assert_eq!(Str::strip_prefix_char("€uro", '€'), Some("uro"));
    assert_eq!(Str::strip_suffix_char("café", 'é'), Some("caf"));
    assert_eq!(Str::strip_circumfix_chars("«ok»", '«', '»'), Some("ok"));
}
#[test]
fn grapheme_iter_char() {
    let input = "H€🧑‍🌾";
    let mut iter = GraphemeIter::<char>::new(input);
    assert_eq!(iter.next(), Some((GraphemeBoundary::Split, 'H')));
    assert_eq!(iter.next(), Some((GraphemeBoundary::Split, '€')));
    assert_eq!(iter.next(), Some((GraphemeBoundary::Split, '🧑')));
    assert_eq!(iter.next(), Some((GraphemeBoundary::Continue, '\u{200d}')));
    assert_eq!(iter.next(), Some((GraphemeBoundary::Continue, '🌾')));
    assert_eq!(iter.next(), None);
}
#[test]
const fn grapheme_iter_charu_count() {
    const COUNT: usize = Str::grapheme_count("H€🧑‍🌾");
    const_assert!(eq COUNT, 3);
}
