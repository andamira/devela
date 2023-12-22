// devela::code::ident
//
//! identifier related macro functionality
//

/// Defines a constant for every given identifier with a value of its index in the list.
/// # Examples
/// ```
/// # use devela::code::ident_const_index;
/// ident_const_index![pub(crate), 3; first, second, third];
/// assert_eq![0, first];
/// assert_eq![1, second];
/// assert_eq![2, third];
/// ```
#[macro_export]
macro_rules! ident_const_index {
    (
        // $vis: the visibility of the constants (pub, pub(super), …).
        // $total: the total number of identifiers.
        // $ident, $($rest),*: the list of identifiers.
        $vis:vis, $total:expr; $ident:ident $(, $($rest:ident),* $(,)? )?
    ) => { $crate::code::paste! {
        #[allow(non_upper_case_globals)]
        $vis const $ident: usize = $total - $crate::code::ident_total_count!( $($($rest)*)? ) - 1;
        $( $crate::code::ident_const_index!($vis, $total; $($rest),*); )?
    }};
}
pub use ident_const_index;

/// Counts the total number of given identifiers as a `usize`.
/// # Examples
/// ```
/// # use devela::code::ident_total_count;
/// assert_eq![5, ident_total_count!(one, two, three, four, five,)]; // with commas
/// # assert_eq![4, ident_total_count!(one, two, three, four)]; // no trailing comma
/// assert_eq![4, ident_total_count!(one two three four)]; // without commas
/// # assert_eq![3, ident_total_count!(one, two three)]; // only the first
/// # assert_eq![3, ident_total_count!(one two, three,)]; // all but first
/// ```
#[macro_export]
macro_rules! ident_total_count {
    () => { 0usize };
    ($ident:ident $(,)? $($rest:ident $(,)? )*) => {
        1usize + $crate::code::ident_total_count!($($rest)*)
    };
}
pub use ident_total_count;
