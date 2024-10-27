// devela::code::macros::capture
//
//! capture tokens macro functionality
//

/// Captures the first token from a list of inputs.
///
/// Usage: `first!(<category> <first>, <tail>*, <optional_comma>?);`
///
/// # Examples
/// ```
/// # use devela::capture_first;
/// assert_eq![5, capture_first!(expr 5, 6, 7)];
/// assert_eq![42, capture_first!(literal 42, "hello", true)];
/// ```
#[macro_export]
#[rustfmt::skip]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! capture_first {
    (block $first:block $(, $tail:block)* $(,)?) => { $first };
    (expr $first:expr $(, $tail:expr)* $(,)?) => { $first };
    (ident $first:ident $(, $tail:ident)* $(,)?) => { $first };
    (literal $first:literal $(, $tail:literal)* $(,)?) => { $first };
    (meta $first:meta $(, $tail:meta)* $(,)?) => { $first };
    (pat $first:pat $(, $tail:pat)* $(,)?) => { $first };
    (path $first:path $(, $tail:path)* $(,)?) => { $first };
    (ty $first:ty $(, $tail:ty)* $(,)?) => { $first };
    (tt $first:tt $(, $tail:tt)* $(,)?) => { $first };
}
#[doc(inline)]
pub use capture_first;

/// Captures all the tokens except the first one, as a tuple.
#[macro_export]
#[rustfmt::skip]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! capture_tail_tuple {
    (block $first:block, $($tail:block),* $(,)?) => { ($($tail),*) };
    (expr $first:expr, $($tail:expr),* $(,)?) => { ($($tail),*) };
    (ident $first:ident, $($tail:ident),* $(,)?) => { ($($tail),*) };
    (literal $first:literal, $($tail:literal),* $(,)?) => { ($($tail),*) };
    (meta $first:meta, $($tail:meta),* $(,)?) => { ($($tail),*) };
    (pat $first:pat, $($tail:pat),* $(,)?) => { ($($tail),*) };
    (path $first:path, $($tail:path),* $(,)?) => { ($($tail),*) };
    (tt $first:tt, $($tail:tt),* $(,)?) => { ($($tail),*) };
    (ty $first:ty, $($tail:ty),* $(,)?) => { ($($tail),*) };

    // Handles the case where there is no tail (optional trailing comma)
    ($cat:tt $first:tt) => { () };
}
#[doc(inline)]
pub use capture_tail_tuple;

// /// Captures all the tokens except the first one.
// #[macro_export]
// #[rustfmt::skip]
// macro_rules! capture_tail {
//     (block $first:block, $($tail:block),*) => { $($tail),* };
//     (expr $first:expr, $($tail:expr),*) => { $($tail),* };
//     (ident $first:ident, $($tail:ident),*) => { $($tail),* };
//     (literal $first:literal, $($tail:literal),*) => { $($tail),* };
//     (meta $first:meta, $($tail:meta),*) => { $($tail),* };
//     (pat $first:pat, $($tail:pat),*) => { $($tail),* };
//     (path $first:path, $($tail:path),*) => { $($tail),* };
//     (tt $first:tt, $($tail:tt),*) => { $($tail),* };
//     (ty $first:ty, $($tail:ty),*) => { $($tail),* };
// }
// pub use capture_tail;

/// Captures the last token from a list of inputs.
#[macro_export]
#[rustfmt::skip]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! capture_last {
    // Base case: when there is only one item left, return it
    (block $first:block) => { $first };
    (expr $first:expr) => { $first };
    (ident $first:ident) => { $first };
    (literal $first:literal) => { $first };
    (meta $first:meta) => { $first };
    (pat $first:pat) => { $first };
    (path $first:path) => { $first };
    (tt $first:tt) => { $first };
    (ty $first:ty) => { $first };

    // Recursive case: discard the first item and continue with the rest
    (block $first:block, $($tail:block),* $(,)?) => {
        $crate::code::capture_last!(block $($tail),*) };
    (expr $first:expr, $($tail:expr),* $(,)?) => {
        $crate::code::capture_last!(expr $($tail),*) };
    (ident $first:ident, $($tail:ident),* $(,)?) => {
        $crate::code::capture_last!(ident $($tail),*) };
    (literal $first:literal, $($tail:literal),* $(,)?) => {
        $crate::code::capture_last!(literal $($tail),*) };
    (meta $first:meta, $($tail:meta),* $(,)?) => {
        $crate::code::capture_last!(meta $($tail),*) };
    (pat $first:pat, $($tail:pat),* $(,)?) => {
        $crate::code::capture_last!(pat $($tail),*) };
    (path $first:path, $($tail:path),* $(,)?) => {
        $crate::code::capture_last!(path $($tail),*) };
    (tt $first:tt, $($tail:tt),* $(,)?) => {
        $crate::code::capture_last!(tt $($tail),*) };
    (ty $first:ty, $($tail:ty),* $(,)?) => {
        $crate::code::capture_last!(ty $($tail),*) };
}
#[doc(inline)]
pub use capture_last;

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn capture_first() {
        assert_eq![{ 2 }, capture_first!(block { 1 + 1 }, { loop {} }, { 3 * 2 })];

        assert_eq![5, capture_first!(expr 5, "a", if a == 3 {}, 4 + 3)];

        let my_var = ();
        assert_eq![my_var, capture_first!(ident my_var, another_var)];

        assert_eq!["32", capture_first!(literal "32", 100, '€')];

        assert_eq!['%', capture_first!(tt '%', "$", 3, {}, something)];

        let captured = 1 as capture_first!(ty i32, u64, bool, f32);
        assert_eq![1_i32, captured];

        /* single item */

        assert_eq!(5, capture_first!(expr 5));
        assert_eq!(my_var, capture_first!(ident my_var));
        assert_eq!('€', capture_first!(literal '€'));
    }

    #[test]
    fn capture_tail_tuple() {
        assert_eq![(6, "a"), capture_tail_tuple!(expr if a == 3 {}, 6, "a")];

        let (my_var, another_var) = ((), ());
        assert_eq![another_var, capture_tail_tuple!(ident my_var, another_var)];

        assert_eq![(my_var, another_var), capture_tail_tuple!(tt { token }, my_var, another_var)];

        struct TestStruct<T>(T);
        let _instance: TestStruct<capture_tail_tuple!(ty i32, f32, bool)> = TestStruct((2.5, true));
    }

    #[test]
    fn capture_last() {
        assert_eq![{ 6 }, capture_last!(block { 1 + 1 }, { loop {} }, { 3 * 2 })];

        assert_eq![7, capture_last!(expr 5, "a", if a == 3 {}, 4 + 3)];

        let (my_var, another_var) = ((), ());
        assert_eq![another_var, capture_last!(ident my_var, another_var)];

        assert_eq!['€', capture_last!(literal "32", 100, '€')];

        let something = ();
        assert_eq![something, capture_last!(tt '%', "$", 3, {}, something)];

        let captured = 1.3 as capture_last!(ty i32, u64, bool, f32);
        assert_eq![1.3_f32, captured];

        /* single item */

        assert_eq!(5, capture_last!(expr 5));
        assert_eq!(my_var, capture_last!(ident my_var));
        assert_eq!('€', capture_last!(literal '€'));
    }
}
