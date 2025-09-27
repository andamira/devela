// devela_base_core::sys::mem::slice::namespace
//
//! Defines the [`Slice`] namespace.
//
// TOC
// - struct Slice
// - macro slice!
//
// TODO:
// - unchecked subslicing. (split)
// - saturating subslicing.
// - first_chunk_padded

mod core;
mod range;
mod take;
mod split;
mod bytes;

#[cfg(test)]
mod tests;

#[doc = crate::_TAG_NAMESPACE!()]
/// Slice-related operations, all of them *const*.
///
#[doc = crate::_doc!(location: "sys/mem")]
///
/// It is designed as a utility namespace and does not hold or wrap data itself.
/// Instead, it operates on slices provided directly as arguments to its static methods.
///
/// # Methods
/// - [methods namespaced from `core::slice`](#coreslice-namespaced-methods)
///
/// - [`range*` API methods](#range-api-methods-for-subslicing):</br>
///   - [**range_to**](#method.range_to)
///    ([*checked*](#method.range_to_checked),
///     [*unchecked*](#method.range_to_unchecked),
///     [_**mut**_](#method.range_to_mut),
///     [*mut_checked*](#method.range_to_mut_checked),
///     [*mut_unchecked*](#method.range_to_mut_unchecked)),           ≈ `&slice[..end]`
///   - [**range_to_inclusive**](#method.range_to_inclusive)
///    ([*checked*](#method.range_to_inclusive_checked),
///     [*unchecked*](#method.range_to_inclusive_unchecked),
///     [_**mut**_](#method.range_to_inclusive_mut),
///     [*mut_checked*](#method.range_to_inclusive_mut_checked),
///     [*mut_unchecked*](#method.range_to_inclusive_mut_unchecked)), ≈ `&slice[..=end]`
///   - [**range_from**](#method.range_from),
///    ([*checked*](#method.range_from_checked),
///     [*unchecked*](#method.range_from_unchecked),
///     [_**mut**_](#method.range_from_mut),
///     [*mut_checked*](#method.range_from_mut_checked),
///     [*mut_unchecked*](#method.range_from_mut_unchecked)),         ≈ `&slice[start..]`
///   - [**range**](#method.range)
///    ([*checked*](#method.range_checked),
///     [*unchecked*](#method.range_unchecked),
///     [_**mut**_](#method.range_mut),
///     [*mut_checked*](#method.range_mut_checked),
///     [*mut_unchecked*](#method.range_mut_unchecked)),              ≈ `&slice[start..end]`
///   - [**range_inclusive**](#method.range_inclusive)
///    ([*checked*](#method.range_inclusive_checked),
///     [*unchecked*](#method.range_inclusive_unchecked),
///     [_**mut**_](#method.range_inclusive_mut),
///     [*mut_checked*](#method.range_inclusive_mut_checked),
///     [*mut_unchecked*](#method.range_inclusive_mut_unchecked)).    ≈ `&slice[start..=end]`
///
/// - [`take*` API methods](#take-api-methods-for-subslicing):</br>
///   - [**take_first**](#method.take_first)
///    ([*checked*](#method.take_first_checked),
///     [*unchecked*](#method.take_first_unchecked),
///     [_**mut**_](#method.take_first_mut),
///     [*mut_checked*](#method.take_first_mut_checked),
///     [*mut_unchecked*](#method.take_first_mut_unchecked)),         ≈ `&slice[..n]`
///   - [**take_last**](#method.take_last)
///    ([*checked*](#method.take_last_checked),
///     [*unchecked*](#method.take_last_unchecked),
///     [_**mut**_](#method.take_last_mut),
///     [*mut_checked*](#method.take_last_mut_checked),
///     [*mut_unchecked*](#method.take_last_mut_unchecked)),          ≈ `&slice[len - n..]`
///   - [**take_omit_last**](#method.take_omit_last)
///    ([*checked*](#method.take_omit_last_checked),
///     [*unchecked*](#method.take_omit_last_unchecked),
///     [_**mut**_](#method.take_omit_last_mut),
///     [*mut_checked*](#method.take_omit_last_mut_checked),
///     [*mut_unchecked*](#method.take_omit_last_mut_unchecked)).     ≈ `&slice[..len - n]`
///
/// - [`*split*` API methods](#split-api-methods-for-subslicing):</br>
///   - [**lsplit**](#method.lsplit)
///    ([*mut*](#method.lsplit_mut)),
///   - [**rsplit**](#method.rsplit)
///    ([*mut*](#method.rsplit_mut)),
///   - [**msplit_left**](#method.msplit_left)
///    ([*mut*](#method.msplit_left_mut)),
///   - [**msplit_right**](#method.msplit_right)
///    ([*mut*](#method.msplit_right_mut)).
///
/// - [specific methods for byte slices](#methods-for-byte-slices)
/// - [`eq` methods for slices of (slices of) primitives](#method.eq)
///
/// Additionally implements `eq()` methods for comparing primitives and slices of primitives.
///
/// See also: [`slice!`], [`ExtSlice`], [`Mem`][crate::Mem], [`Ptr`][crate::Ptr].
#[doc = crate::doclink!(custom devela "[`ExtSlice`]" "sys/mem/trait.ExtSlice.html")]
#[derive(Debug)]
pub struct Slice<T>(crate::PhantomData<T>);

/// Invokes [`Slice`] [range methods][Slice#range-api-methods-for-subslicing] using short notation.
///
#[doc = crate::_doc!(location: "sys/mem")]
///
/// # Syntax
/// ```text
/// (s = slice, l = lower bound, u = upper bound)
///
/// slice![s, l, ..= u]                  // Slice::range_inclusive(s, l, u)
/// slice![s, l, .. u]                   // Slice::range(s, l, u)
/// slice![s, l, ..]                     // Slice::range_from(s, l, u)
/// slice![s, ..= u]                     // Slice::range_to_inclusive(s, l, u)
/// slice![s, .. u]                      // Slice::range_to(s, l, u)
///
/// slice![checked s, l, ..= u]          // Slice::range_inclusive_checked(s, l, u)
/// slice![checked s, l, .. u]           // Slice::range_checked(s, l, u)
/// slice![checked s, l, ..]             // Slice::range_from_checked(s, l, u)
/// slice![checked s, ..= u]             // Slice::range_to_inclusive_checked(s, l, u)
/// slice![checked s, .. u]              // Slice::range_to_checked(s, l, u)
///
/// slice![unchecked s, l, ..= u]        // Slice::range_inclusive_unchecked(s, l, u)
/// slice![unchecked s, l, .. u]         // Slice::range_unchecked(s, l, u)
/// slice![unchecked s, l, ..]           // Slice::range_from_unchecked(s, l, u)
/// slice![unchecked s, ..= u]           // Slice::range_to_inclusive_unchecked(s, l, u)
/// slice![unchecked s, .. u]            // Slice::range_to_unchecked(s, l, u)
///
/// slice![mut s, l, ..= u]              // Slice::range_inclusive_mut(s, l, u)
/// slice![mut s, l, .. u]               // Slice::range_mut(s, l, u)
/// slice![mut s, l, ..]                 // Slice::range_from_mut(s, l, u)
/// slice![mut s, ..= u]                 // Slice::range_to_inclusive_mut(s, l, u)
/// slice![mut s, .. u]                  // Slice::range_to_mut(s, l, u)
///
/// slice![mut_checked s, l, ..= u]      // Slice::range_inclusive_mut_checked(s, l, u)
/// slice![mut_checked s, l, .. u]       // Slice::range_mut_checked(s, l, u)
/// slice![mut_checked s, l, ..]         // Slice::range_from_mut_checked(s, l, u)
/// slice![mut_checked s, ..= u]         // Slice::range_to_inclusive_mut_checked(s, l, u)
/// slice![mut_checked s, .. u]          // Slice::range_to_mut_checked(s, l, u)
///
/// slice![mut_unchecked s, l, ..= u]    // Slice::range_inclusive_mut_unchecked(s, l, u)
/// slice![mut_unchecked s, l, .. u]     // Slice::range_mut_unchecked(s, l, u)
/// slice![mut_unchecked s, l, ..]       // Slice::range_from_mut_unchecked(s, l, u)
/// slice![mut_unchecked s, ..= u]       // Slice::range_to_inclusive_mut_unchecked(s, l, u)
/// slice![mut_unchecked s, .. u]        // Slice::range_to_mut_unchecked(s, l, u)
/// ```
///
/// # Notes
/// - Commas are always required to separate expressions: `slice![s, a(), ..= b()]`
/// - But they are optional for ranges of literals/blocks: `slice![s, 1..=3]` ≡ `slice![s, 1, ..=3]`
///
/// # Example
/// ```
/// # use devela_base_core::slice;
/*
/// # let (x, y) = (10, 20);
/// # fn calc() -> i32 { 10 }
/// # fn other() -> i32 { 20 }
/// // Literals
/// slice![1 ..=3];    // [1, 3]
/// slice![1<.. 3];    // (1, 3)
///
/// // Expressions
/// slice![(x+1), ..= (y*2)];
/// slicslice![{ calc() }, <.. { other() }];
///
/// // Unbounded
/// interval![.., i32]; // (-∞, ∞) as i32
/// slice![..=10];   // (-∞, 10]
/// slice![1..];     // [1, ∞)
*/
/// ```
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! _slice {
    (

    /* mut_checked expressions */
     mut_checked $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from_mut_checked($s, $l) };
    (mut_checked $s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive_mut_checked($s, $u) };
    (mut_checked $s:expr,           ..  $u:expr) => { $crate::Slice::range_to_mut_checked($s, $u) };
    (
    /* mut_checked blocks */
     mut_checked $s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:block  ..  $u:block) => { $crate::Slice::range_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:block  ..          ) => { $crate::Slice::range_from_mut_checked($s, $l) };
    (mut_checked $s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive_mut_checked($s, $u) };
    (mut_checked $s:expr,           ..  $u:block) => { $crate::Slice::range_to_mut_checked($s, $u) };
    (
    /* mut_checked literals */
     mut_checked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_mut_checked($s, $l) };
    (mut_checked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_mut_checked($s, $u) };
    (mut_checked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_mut_checked($s, $u) };
    (

    /* mut_unchecked expressions */
     mut_unchecked $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from_mut_unchecked($s, $l) };
    (mut_unchecked $s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr,           ..  $u:expr) => { $crate::Slice::range_to_mut_unchecked($s, $u) };
    (
    /* mut_unchecked blocks */
     mut_unchecked $s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:block  ..  $u:block) => { $crate::Slice::range_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:block  ..          ) => { $crate::Slice::range_from_mut_unchecked($s, $l) };
    (mut_unchecked $s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr,           ..  $u:block) => { $crate::Slice::range_to_mut_unchecked($s, $u) };
    (
    /* mut_unchecked literals */
     mut_unchecked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_mut_unchecked($s, $l) };
    (mut_unchecked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_mut_unchecked($s, $u) };
    (

    /* mut expressions */
     mut $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive_mut($s, $l, $u) };
    (mut $s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range_mut($s, $l, $u) };
    (mut $s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from_mut($s, $l) };
    (mut $s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive_mut($s, $u) };
    (mut $s:expr,           ..  $u:expr) => { $crate::Slice::range_to_mut($s, $u) };
    (
    /* mut blocks */
     mut $s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive_mut($s, $l, $u) };
    (mut $s:expr, $l:block  ..  $u:block) => { $crate::Slice::range_mut($s, $l, $u) };
    (mut $s:expr, $l:block  ..          ) => { $crate::Slice::range_from_mut($s, $l) };
    (mut $s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive_mut($s, $u) };
    (mut $s:expr,           ..  $u:block) => { $crate::Slice::range_to_mut($s, $u) };
    (
    /* mut literals */
     mut $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_mut($s, $l, $u) };
    (mut $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_mut($s, $l, $u) };
    (mut $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_mut($s, $l) };
    (mut $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_mut($s, $u) };
    (mut $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_mut($s, $u) };
    (

    /* checked expressions */
     checked $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive_checked($s, $l, $u) };
    (checked $s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range_checked($s, $l, $u) };
    (checked $s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from_checked($s, $l) };
    (checked $s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive_checked($s, $u) };
    (checked $s:expr,           ..  $u:expr) => { $crate::Slice::range_to_checked($s, $u) };
    (
    /* checked blocks */
     checked $s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive_checked($s, $l, $u) };
    (checked $s:expr, $l:block  ..  $u:block) => { $crate::Slice::range_checked($s, $l, $u) };
    (checked $s:expr, $l:block  ..          ) => { $crate::Slice::range_from_checked($s, $l) };
    (checked $s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive_checked($s, $u) };
    (checked $s:expr,           ..  $u:block) => { $crate::Slice::range_to_checked($s, $u) };
    (
    /* checked literals */
     checked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_checked($s, $l, $u) };
    (checked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_checked($s, $l, $u) };
    (checked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_checked($s, $l) };
    (checked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_checked($s, $u) };
    (checked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_checked($s, $u) };
    (

    /* unchecked expressions */
     unchecked $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from_unchecked($s, $l) };
    (unchecked $s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive_unchecked($s, $u) };
    (unchecked $s:expr,           ..  $u:expr) => { $crate::Slice::range_to_unchecked($s, $u) };
    (
    /* unchecked blocks */
     unchecked $s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:block  ..  $u:block) => { $crate::Slice::range_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:block  ..          ) => { $crate::Slice::range_from_unchecked($s, $l) };
    (unchecked $s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive_unchecked($s, $u) };
    (unchecked $s:expr,           ..  $u:block) => { $crate::Slice::range_to_unchecked($s, $u) };
    (
    /* unchecked literals */
     unchecked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_unchecked($s, $l) };
    (unchecked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_unchecked($s, $u) };
    (unchecked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_unchecked($s, $u) };
    // NOTE: this one has to be the last
    (
    /* expressions */
     $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive($s, $l, $u) };
    ($s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range($s, $l, $u) };
    ($s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from($s, $l) };
    ($s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive($s, $u) };
    ($s:expr,           ..  $u:expr) => { $crate::Slice::range_to($s, $u) };
    (
    /* blocks */
     $s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive($s, $l, $u) };
    ($s:expr, $l:block  ..  $u:block) => { $crate::Slice::range($s, $l, $u) };
    ($s:expr, $l:block  ..          ) => { $crate::Slice::range_from($s, $l) };
    ($s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive($s, $u) };
    ($s:expr,           ..  $u:block) => { $crate::Slice::range_to($s, $u) };
    (
    /* literals */
     $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive($s, $l, $u) };
    ($s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range($s, $l, $u) };
    ($s:expr, $l:literal  ..            ) => { $crate::Slice::range_from($s, $l) };
    ($s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive($s, $u) };
    ($s:expr,             ..  $u:literal) => { $crate::Slice::range_to($s, $u) };
    (

    /* syntax error msg */
    $($t:tt)*) => {
        compile_error! { "Invalid interval syntax. Expected forms like: l..u, l..=u, ..x, ..=x" }
    };
}
#[doc(inline)]
pub use _slice as slice;
