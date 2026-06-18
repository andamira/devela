// devela/src/text/str/namespace/_tests.rs

// IMPROVE: add tests for missing methods from: range, split, take.

use crate::Str;

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
