// devela_macros::bodies::paste::tests_attr

/// ```
/// use devela_macros::paste;
/// paste! {
///     #[doc = "Hello " "World"]
///     pub struct DocStringPaste;
/// }
/// ```
fn test_doc_string_paste() {}

/// ```
/// use devela_macros::paste;
/// paste! {
///     #[doc = "hello"]
///     pub struct DocSingleToken;
/// }
/// ```
fn test_doc_single_token() {}

/// ```
/// use devela_macros::paste;
/// paste! {
///     #[derive(Clone, Copy)]
///     struct DocDeriveAttr(u8);
/// }
/// ```
fn test_derive_attr_in_paste() {}

/// ```
/// use devela_macros::paste;
/// paste! {
///     #[cfg_attr(not(all()), allow([<foo bar>]))]
///     pub struct DocPasteInAttr;
/// }
/// ```
fn test_paste_in_attr_paren() {}

/// ```
/// use devela_macros::paste;
/// paste! {
///     #[cfg_attr(not(all()), ::foo::bar(baz))]
///     pub struct DocAbsPath;
/// }
/// ```
fn test_absolute_path_attr() {}

/// ```
/// use devela_macros::paste;
/// paste! {
///     #[doc = r"Hello " "World"]
///     pub struct DocRawStr;
/// }
/// ```
fn test_raw_str_doc_attr() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! m {
///     ($val:ident) => {
///         paste! {
///             #[cfg_attr(not(all()), allow(ident = $val "world"))]
///             pub struct DocNoneGroupStringlike;
///         }
///     }
/// }
/// m!(hello);
/// ```
fn test_none_group_stringlike() {}

/// ```
/// use devela_macros::paste;
/// macro_rules! with_doc_path {
///     ($m:ident) => {
///         paste! {
///             #[doc = stringify!($m::Item)]
///             pub fn doc_none_group_before_double_colon() {}
///         }
///     }
/// }
///      ///
/// with_doc_path!(my_mod);
/// doc_none_group_before_double_colon();
/// ```
fn test_none_group_before_double_colon_in_attr_context() {}
