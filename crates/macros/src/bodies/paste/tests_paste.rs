// devela_macros::bodies::paste::tests_paste

/// ```
/// use devela_macros::paste;
/// let arr: [u8; 3] = paste!([1u8, 2, 3]);
/// ```
fn test_non_paste_bracket_returns_input() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///     ($id:ident) => { paste! { fn $id() {} } }
/// }
/// m!(doc_flatten_fn);
/// doc_flatten_fn();
/// ```
fn test_flatten_single_ident_group() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///     ($life:lifetime) => {
///         paste! { struct DocRef<$life>(pub &$life ()); }
///     }
/// }
/// m!('a);
/// ```
fn test_flatten_lifetime_group() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///     ($t:path) => { paste! { type DocPathAlias = $t; } }
/// }
/// m!(std::string::String);
/// let _: DocPathAlias = String::new();
/// ```
fn test_flatten_path_group() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///     ($lit:literal) => { paste! { const DOC_LIT: u8 = $lit; } }
/// }
/// m!(99u8);
/// ```
fn test_flatten_literal_group() {}

/// ```
/// use devela_macros::paste;
/// paste! { let _: std::string::String = String::new(); }
/// ```
fn test_double_colon_none_group() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///    ($t:ty) => {
///       paste! {
///          pub const NONE_GROUP_TY_STR: &str = stringify!($t::method);
///      }
///   }
/// }
/// m!(Vec<u8>);
/// ```
fn test_none_group_complex_type_before_double_colon() {}

/// ```
/// use devela_macros::paste;
/// paste! {
///     mod doc_inner_mod {
///         #![allow(dead_code)]
///         pub struct DocInner;
///     }
/// }
/// ```
fn test_inner_mod() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! allow_lint {
///     ($lint:ident) => {
///         paste! {
///             #[allow(clippy::$lint)]
///             pub struct DocLintStruct;
///         }
///     }
/// }
/// allow_lint!(pedantic);
/// ```
fn test_double_colon_none_group_in_attr() {}

/// ```
/// use devela_macros::paste;
/// paste! { const _: &str = stringify!([<'\u{48}' ello>]); }
/// ```
fn test_char_unicode_escape_in_paste() {}

/// ```
/// use devela_macros::paste;
/// paste! { const _: &str = stringify!([<r"hel" lo>]); }
/// ```
fn test_raw_string_in_paste() {}

/// ```
/// use devela_macros::paste;
/// paste! { struct DocLifeRef<[<'a>]>(pub &[<'a>] ()); }
/// ```
fn test_lifetime_paste_tokens() {}

/// ```
/// use devela_macros::paste;
/// paste! { #[allow(non_camel_case_types)] struct [<# loop>]; }
/// ```
fn test_raw_mode_paste_tokens() {}

/// ```
/// use devela_macros::paste;
/// paste! { const _: &str = stringify!([<'\u{41}' bcde>]); }
/// ```
fn test_char_unicode_paste_tokens() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///     ($t:ty) => {
///         paste! { let _: $t = std::string::String::new(); }
///     }
/// }
/// m!(String);
/// ```
fn test_none_group_not_followed_by_double_colon() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: u32 = [<99 invalid>]; }
/// ```
fn test_error_invalid_literal_with_trailing_tokens() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<@invalid>]; }
/// ```
fn test_error_invalid_identifier_special_char() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [identifier]; }
/// ```
fn test_error_expected_bracket_with_angle() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<identifier]; }
/// ```
fn test_error_expected_closing_angle() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<identifier> extra]; }
/// ```
fn test_error_unexpected_token_after_closing_angle() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<"byte\\string">]; }
/// ```
fn test_error_unsupported_escaped_string() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<b"byte_string">]; }
/// ```
fn test_error_unsupported_byte_string() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn [<x b'y'>]() {} }
/// ```
fn test_error_unsupported_byte_char_literal() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<br"raw_byte">]; }
/// ```
fn test_error_unsupported_raw_byte_string() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<"string.with.dots">]; }
/// ```
fn test_error_unsupported_string_with_dots() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<"string+plus">]; }
/// ```
fn test_error_unsupported_string_with_plus() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<>]; }
/// ```
fn test_error_expected_content_in_brackets() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<a..b>]; }
/// ```
fn test_error_invalid_numeric_literal() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [< >]; }
/// ```
fn test_error_no_tokens_in_paste() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<'static>]; }
/// ```
fn test_error_lifetime_without_identifier() {}

/// ```compile_fail
/// use devela_macros::paste;
/// macro_rules! m {
///     ($x:ident) => {
///         paste! { const _: u32 = [<$x>]; }
///     }
/// }
/// m!(123);
/// ```
fn test_error_invalid_ident_starting_with_number() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<"terminated>]; }
/// ```
fn test_error_terminated_string_literal() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { let x = [<y z>]; }
/// ```
fn test_error_multiple_tokens_non_paste() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { let x: i32 = [<not_a_number>]; }
/// ```
fn test_error_paste_result_type_mismatch() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<'a 'b>]; }
/// ```
fn test_error_multiple_lifetimes() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<42 56>]; }
/// ```
fn test_error_multiple_numeric_tokens() {}

/// ```compile_fail
/// use devela_macros::paste;
/// macro_rules! concat_idents {
///     ($a:ident, $b:ident) => { paste! { const TEST: u32 = [<method_ $a>](); } }
/// }
/// concat_idents!(test, func);
/// ```
fn test_error_method_call_on_paste_result() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _ : &str = [<r# if>]; }
/// ```
fn test_error_raw_keyword_identifier() {}

/// ```compile_fail
/// use devela_macros::paste;
/// macro_rules! m {
///     () => { paste! { const _: u32 = [<>]; } }
/// }
/// m!();
/// ```
fn test_error_empty_macro_result() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<-identifier>]; }
/// ```
fn test_error_hyphen_at_start() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<'123>]; }
/// ```
fn test_error_apostrophe_with_number() {}

/// ```compile_fail
/// use devela_macros::paste;
/// macro_rules! m {
///     ($lit:literal) => {
///         paste! { const _: &str = stringify!([<$lit>]); }
///     }
/// }
/// m!("both" "strings");
/// ```
fn test_error_invalid_literal_argument() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<'\u{D800}>]; }
/// ```
fn test_error_invalid_unicode_escape() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: u32 = [<0x>]; }
/// ```
fn test_error_incomplete_hex_literal() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: u32 = [<0b>]; }
/// ```
fn test_error_incomplete_binary_literal() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: f64 = [<1..5>]; }
/// ```
fn test_error_range_literal_invalid() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<if>]; }
/// ```
fn test_error_keyword_as_identifier() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<fn>]; }
/// ```
fn test_error_fn_keyword() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<match>]; }
/// ```
fn test_error_match_keyword() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<{inner}>]; }
/// ```
fn test_error_brace_group_in_paste() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<(parts)>]; }
/// ```
fn test_error_paren_group_in_paste() {}

/// ```compile_fail
/// use devela_macros::paste;
/// macro_rules! test {
///     () => { paste! { const _: &str = [<&&invalid>]; } }
/// }
/// test!();
/// ```
fn test_error_double_ampersand() {}

/// ```compile_fail
/// use devela_macros::paste;
/// macro_rules! test {
///     () => { paste! { const _: &str = [<||invalid>]; } }
/// }
/// test!();
/// ```
fn test_error_double_pipe() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: &str = [<@#$>]; }
/// ```
fn test_error_random_special_chars() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { let value: i32 = [<test>]; }
/// ```
fn test_error_undefined_identifier_value() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn doc_err_in_brace() { let _ = [<@>]; } }
/// ```
fn test_error_invalid_paste_inside_brace_group() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { const _: u32 = [<0 x>]; }
/// ```
fn test_error_pasted_incomplete_hex_literal() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { #[cfg(test) trailing_token] fn f() {} }
/// ```
fn test_error_attr_trailing_tokens_after_paren_group() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { struct [<Foo::+Bar>]; }
/// ```
fn test_error_invalid_colon_sequence() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn [<Bar:::Test>]() {} }
/// ```
fn test_error_invalid_colon_patterns() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn [*invalid>]() {} }
/// ```
fn test_error_bracket_first_token_not_less_than() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn [<test]() {} }
/// ```
fn test_error_bracket_closing_token_not_greater_than() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn [<test>extra]() {} }
/// ```
fn test_error_extra_tokens_after_bracket() {}

/// ```compile_fail
/// use devela_macros::paste;
/// paste! { fn [<test:replace('\u{XY}', 'A')>]() {} }
/// ```
fn test_error_invalid_hex_unicode() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! test_replace {
///     ($a:expr, $b:expr) => {
///         paste! { concat!(stringify!($a), stringify!($b)) }
///     }
/// }
/// let result = test_replace!(x, "y");
/// ```
fn test_macro_group_flattening() {}

/// ```
/// use devela_macros::paste;
/// paste! { fn [<foo_bar>]() {} }
/// foo_bar();
/// ```
fn test_valid_paste_operation() {}

/// ```
/// use devela_macros::paste;
/// paste! { use std::[<vec>]; }
/// let v = Vec::<i32>::new();
/// assert_eq!(v.len(), 0);
/// ```
fn test_valid_path_with_double_colon() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! concat_name {
///     ($name:ident) => {
///         paste! { const [<$name:lower>]: &str = stringify!($name); }
///     }
/// }
/// concat_name!(TEST);
/// ```
fn test_valid_modifiers() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! test_ident_colon {
///     ($param:ident: $ty:ident) => {
///         paste! { fn [<test_$param>]($param: $ty) {} }
///     }
/// }
/// test_ident_colon!(x: i32);
/// test_x(42i32);
/// ```
fn test_type_annotation() {}
