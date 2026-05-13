// devela_macros::bodies::paste::tests_segment

#[test]
fn test_is_quoted_string_or_char() {
    use super::PasteSegment as Ps;
    assert!(Ps::is_quoted_string_or_char("\"hello\"", false));
    assert!(!Ps::is_quoted_string_or_char("abc", false));
    assert!(!Ps::is_quoted_string_or_char("\"", false));
    assert!(!Ps::is_quoted_string_or_char("\"abc", false));
    assert!(Ps::is_quoted_string_or_char("'a'", true));
    assert!(!Ps::is_quoted_string_or_char("'abc", true));
    assert!(!Ps::is_quoted_string_or_char("abc", true));
}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<a + b>]() {} }
/// ```
fn test_unexpected_punct() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<{}>]() {} }
/// ```
fn test_unexpected_group() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env! foo>]() {} }
/// ```
fn test_env_missing_paren() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!(foo)>]() {} }
/// ```
fn test_env_non_literal_arg() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!()>]() {} }
/// ```
fn test_env_empty() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!("CARGO_PKG_NAME" "extra")>]() {} }
/// ```
fn test_env_unexpected_token() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<a:>]() {} }
/// ```
fn test_no_ident_after_colon() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<a:replace x>]() {} }
/// ```
fn test_replace_missing_paren() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<a:pillow>]() {} }
/// ```
fn test_unsupported_modifier() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<a:replace("x","y","z")>]() {} }
/// ```
fn test_replace_extra_token() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { struct [<:replace("H","W")>]; }
/// ```
fn test_replace_no_preceding_value() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<a:replace(x)>]() {} }
/// ```
fn test_replace_wrong_format() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { struct [<foo # bar>]; }
/// ```
fn test_raw_mode_wrong_position() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<'a 'b>]() {} }
/// ```
fn test_unexpected_lifetime() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!("THIS_VAR_DOES_NOT_EXIST_XYZ_999")>]() {} }
/// ```
fn test_no_such_env_var() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<:lower>]() {} }
/// ```
fn test_modifier_no_preceding_value() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!["CARGO_PKG_NAME"]>]() {} }
/// ```
fn test_env_bracket_group() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { struct [<a:replace["x", "y"]>]; }
/// ```
fn test_replace_bracket_group() {}

/// ```compile_fail
/// use pastey::paste;
/// macro_rules! m {
///     ($e:expr) => { paste! { fn [<$e foo>]() {} } }
/// }
/// m!(1 > 0);
/// ```
fn test_none_group_unexpected_token() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { struct [<a:replace("x"; "y")>]; }
/// ```
fn test_replace_wrong_separator() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!(42)>]() {} }
/// ```
fn test_env_numeric_literal() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { fn [<env!('a')>]() {} }
/// ```
fn test_env_char_literal() {}

/// ```compile_fail
/// use pastey::paste;
/// paste! { struct [<Foo:replace(["x"], "y")>]; }
/// ```
fn test_replace_from_bracket_group() {}

/// ```compile_fail
/// use pastey::paste;
/// macro_rules! m {
///     ($from:expr) => { paste! { struct [<Foo:replace($from, "y")>]; } }
/// }
/// m!(x + y);
/// ```
fn test_replace_from_multi_token_none_group() {}

/// ```compile_fail
/// use pastey::paste;
/// macro_rules! m {
///     ($to:expr) => { paste! { struct [<Foo:replace("o", $to)>]; } }
/// }
/// m!(x + y);
/// ```
fn test_replace_to_multi_token_none_group() {}
