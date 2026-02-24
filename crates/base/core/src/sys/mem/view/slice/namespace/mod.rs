// devela_base_core::sys::mem::view::slice::namespace
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

#![cfg_attr(all(doc, not(feature = "unsafe_slice")), allow(rustdoc::broken_intra_doc_links))]

#[cfg(doc)]
use crate::Str;

mod core;
mod range;
mod take;
mod split;
mod chunk;
mod bytes;

#[cfg(test)]
mod tests;

#[doc = crate::_tags!(lifetime namespace)]
/// Slice-related operations, most of them *const*.
#[doc = crate::_doc_location!("sys/mem/view")]
///
/// It is designed as a utility namespace and does not hold or wrap data itself.
/// Instead, it operates on slices provided directly as arguments to its static methods.
///
/// # Methods
/// - [methods namespaced from `core::slice`](#coreslice-namespaced-methods)
///
/// - [`range*` API methods](#range-api-methods-for-subslicing):<br/>
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
/// - [`take*` API methods](#take-api-methods-for-subslicing):<br/>
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
/// - [`*split*` API methods](#split-api-methods-for-subslicing):<br/>
///   - [**lsplit**](#method.lsplit)
///    ([*mut*](#method.lsplit_mut)),
///   - [**rsplit**](#method.rsplit)
///    ([*mut*](#method.rsplit_mut)),
///   - [**msplit_left**](#method.msplit_left)
///    ([*mut*](#method.msplit_left_mut)),
///   - [**msplit_right**](#method.msplit_right)
///    ([*mut*](#method.msplit_right_mut)).
///
/// - [`*chunk*` API methods](#chunk-api-methods-for-subslicing):<br/>
///   - [**chunks_exact**](#method.chunks_exact)
///    ([*mut*](#method.chunks_exact_mut)),
///   - [**chunk**](#method.chunk)
///    ([*mut*](#method.chunks_mut)).
///
/// - [specific methods for byte slices](#methods-for-byte-slices)
/// - [`eq` methods for slices of (slices of) primitives](#method.eq)
///
/// Additionally implements `eq()` methods for comparing primitives and slices of primitives.
///
/// See also: [`slice!`], [`SliceExt`], [`Mem`][crate::Mem], [`Ptr`][crate::Ptr].
#[doc = crate::doclink!(custom devela "[`SliceExt`]" "sys/mem/view/trait.SliceExt.html")]
#[derive(Debug)]
pub struct Slice<T>(crate::PhantomData<T>);

#[doc = crate::_tags!(namespace)]
/// Invokes [`Slice`] [range methods][Slice#range-api-methods-for-subslicing] using short notation.
#[doc = crate::_doc_location!("sys/mem/view")]
///
/// # Syntax
/// `(s = slice, l = lower bound, u = upper bound)`
///
/// `/* generic slices */`
///
/// `slice![s, l, ..= u]                 // `[`Slice::range_inclusive`]`(s, l, u)`
/// `slice![s, l, .. u]                  // `[`Slice::range`]`(s, l, u)`
/// `slice![s, l, ..]                    // `[`Slice::range_from`]`(s, l, u)`
/// `slice![s, ..= u]                    // `[`Slice::range_to_inclusive`]`(s, l, u)`
/// `slice![s, .. u]                     // `[`Slice::range_to`]`(s, l, u)`
///
/// `slice![checked s, l, ..= u]         // `[`Slice::range_inclusive_checked`]`(s, l, u)`
/// `slice![checked s, l, .. u]          // `[`Slice::range_checked`]`(s, l, u)`
/// `slice![checked s, l, ..]            // `[`Slice::range_from_checked`]`(s, l, u)`
/// `slice![checked s, ..= u]            // `[`Slice::range_to_inclusive_checked`]`(s, l, u)`
/// `slice![checked s, .. u]             // `[`Slice::range_to_checked`]`(s, l, u)`
///
/// `slice![unchecked s, l, ..= u]       // `[`Slice::range_inclusive_unchecked`]`(s, l, u)`
/// `slice![unchecked s, l, .. u]        // `[`Slice::range_unchecked`]`(s, l, u)`
/// `slice![unchecked s, l, ..]          // `[`Slice::range_from_unchecked`]`(s, l, u)`
/// `slice![unchecked s, ..= u]          // `[`Slice::range_to_inclusive_unchecked`]`(s, l, u)`
/// `slice![unchecked s, .. u]           // `[`Slice::range_to_unchecked`]`(s, l, u)`
///
/// `slice![mut s, l, ..= u]             // `[`Slice::range_inclusive_mut`]`(s, l, u)`
/// `slice![mut s, l, .. u]              // `[`Slice::range_mut`]`(s, l, u)`
/// `slice![mut s, l, ..]                // `[`Slice::range_from_mut`]`(s, l, u)`
/// `slice![mut s, ..= u]                // `[`Slice::range_to_inclusive_mut`]`(s, l, u)`
/// `slice![mut s, .. u]                 // `[`Slice::range_to_mut`]`(s, l, u)`
///
/// `slice![mut_checked s, l, ..= u]     // `[`Slice::range_inclusive_mut_checked`]`(s, l, u)`
/// `slice![mut_checked s, l, .. u]      // `[`Slice::range_mut_checked`]`(s, l, u)`
/// `slice![mut_checked s, l, ..]        // `[`Slice::range_from_mut_checked`]`(s, l, u)`
/// `slice![mut_checked s, ..= u]        // `[`Slice::range_to_inclusive_mut_checked`]`(s, l, u)`
/// `slice![mut_checked s, .. u]         // `[`Slice::range_to_mut_checked`]`(s, l, u)`
///
/// `slice![mut_unchecked s, l, ..= u]   // `[`Slice::range_inclusive_mut_unchecked`]`(s, l, u)`
/// `slice![mut_unchecked s, l, .. u]    // `[`Slice::range_mut_unchecked`]`(s, l, u)`
/// `slice![mut_unchecked s, l, ..]      // `[`Slice::range_from_mut_unchecked`]`(s, l, u)`
/// `slice![mut_unchecked s, ..= u]      // `[`Slice::range_to_inclusive_mut_unchecked`]`(s, l, u)`
/// `slice![mut_unchecked s, .. u]       // `[`Slice::range_to_mut_unchecked`]`(s, l, u)`
///
/// `/* string slices */`
///
/// `slice![str s, l, ..= u]             // `[`Str::range_inclusive`]`(s, l, u)`
/// `slice![str s, l, .. u]              // `[`Str::range`]`(s, l, u)`
/// `slice![str s, l, ..]                // `[`Str::range_from`]`(s, l, u)`
/// `slice![str s, ..= u]                // `[`Str::range_to_inclusive`]`(s, l, u)`
/// `slice![str s, .. u]                 // `[`Str::range_to`]`(s, l, u)`
///
/// `slice![str_checked s, l, ..= u]     // `[`Str::range_inclusive_checked`]`(s, l, u)`
/// `slice![str_checked s, l, .. u]      // `[`Str::range_checked`]`(s, l, u)`
/// `slice![str_checked s, l, ..]        // `[`Str::range_from_checked`]`(s, l, u)`
/// `slice![str_checked s, ..= u]        // `[`Str::range_to_inclusive_checked`]`(s, l, u)`
/// `slice![str_checked s, .. u]         // `[`Str::range_to_checked`]`(s, l, u)`
///
/// `slice![str_mut s, l, ..= u]         // `[`Str::range_inclusive_mut`]`(s, l, u)`
/// `slice![str_mut s, l, .. u]          // `[`Str::range_mut`]`(s, l, u)`
/// `slice![str_mut s, l, ..]            // `[`Str::range_from_mut`]`(s, l, u)`
/// `slice![str_mut s, ..= u]            // `[`Str::range_to_inclusive_mut`]`(s, l, u)`
/// `slice![str_mut s, .. u]             // `[`Str::range_to_mut`]`(s, l, u)`
///
/// `slice![str_mut_checked s, l, ..= u] // `[`Str::range_inclusive_mut_checked`]`(s, l, u)`
/// `slice![str_mut_checked s, l, .. u]  // `[`Str::range_mut_checked`]`(s, l, u)`
/// `slice![str_mut_checked s, l, ..]    // `[`Str::range_from_mut_checked`]`(s, l, u)`
/// `slice![str_mut_checked s, ..= u]    // `[`Str::range_to_inclusive_mut_checked`]`(s, l, u)`
/// `slice![str_mut_checked s, .. u]     // `[`Str::range_to_mut_checked`]`(s, l, u)`
///
/// # Notes
/// - Commas are always required to separate expressions: `slice![s, a(), ..= b()]`
/// - Commas are optional for ranges of literals/blocks: `slice![s, 1..=3]` ≡ `slice![s, 1, ..=3]`
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! _slice {
    /* Str */
    (
     str_mut_checked $s:expr, $l:expr,    ..= $u:expr) => { $crate::Str::range_inclusive_mut_checked($s, $l, $u) };
    (str_mut_checked $s:expr, $l:expr,    ..  $u:expr) => { $crate::Str::range_mut_checked($s, $l, $u) };
    (str_mut_checked $s:expr, $l:expr,    ..         ) => { $crate::Str::range_from_mut_checked($s, $l) };
    (str_mut_checked $s:expr,             ..= $u:expr) => { $crate::Str::range_to_inclusive_mut_checked($s, $u) };
    (str_mut_checked $s:expr,             ..  $u:expr) => { $crate::Str::range_to_mut_checked($s, $u) };
    (str_mut_checked $s:expr, $l:block    ..= $u:block) => { $crate::Str::range_inclusive_mut_checked($s, $l, $u) };
    (str_mut_checked $s:expr, $l:block    ..  $u:block) => { $crate::Str::range_mut_checked($s, $l, $u) };
    (str_mut_checked $s:expr, $l:block    ..          ) => { $crate::Str::range_from_mut_checked($s, $l) };
    (str_mut_checked $s:expr,             ..= $u:block) => { $crate::Str::range_to_inclusive_mut_checked($s, $u) };
    (str_mut_checked $s:expr,             ..  $u:block) => { $crate::Str::range_to_mut_checked($s, $u) };
    (str_mut_checked $s:expr, $l:literal  ..= $u:literal) => { $crate::Str::range_inclusive_mut_checked($s, $l, $u) };
    (str_mut_checked $s:expr, $l:literal  ..  $u:literal) => { $crate::Str::range_mut_checked($s, $l, $u) };
    (str_mut_checked $s:expr, $l:literal  ..            ) => { $crate::Str::range_from_mut_checked($s, $l) };
    (str_mut_checked $s:expr,             ..= $u:literal) => { $crate::Str::range_to_inclusive_mut_checked($s, $u) };
    (str_mut_checked $s:expr,             ..  $u:literal) => { $crate::Str::range_to_mut_checked($s, $u) };
    (
     str_mut $s:expr, $l:expr,    ..= $u:expr) => { $crate::Str::range_inclusive_mut($s, $l, $u) };
    (str_mut $s:expr, $l:expr,    ..  $u:expr) => { $crate::Str::range_mut($s, $l, $u) };
    (str_mut $s:expr, $l:expr,    ..         ) => { $crate::Str::range_from_mut($s, $l) };
    (str_mut $s:expr,             ..= $u:expr) => { $crate::Str::range_to_inclusive_mut($s, $u) };
    (str_mut $s:expr,             ..  $u:expr) => { $crate::Str::range_to_mut($s, $u) };
    (str_mut $s:expr, $l:block    ..= $u:block) => { $crate::Str::range_inclusive_mut($s, $l, $u) };
    (str_mut $s:expr, $l:block    ..  $u:block) => { $crate::Str::range_mut($s, $l, $u) };
    (str_mut $s:expr, $l:block    ..          ) => { $crate::Str::range_from_mut($s, $l) };
    (str_mut $s:expr,             ..= $u:block) => { $crate::Str::range_to_inclusive_mut($s, $u) };
    (str_mut $s:expr,             ..  $u:block) => { $crate::Str::range_to_mut($s, $u) };
    (str_mut $s:expr, $l:literal  ..= $u:literal) => { $crate::Str::range_inclusive_mut($s, $l, $u) };
    (str_mut $s:expr, $l:literal  ..  $u:literal) => { $crate::Str::range_mut($s, $l, $u) };
    (str_mut $s:expr, $l:literal  ..            ) => { $crate::Str::range_from_mut($s, $l) };
    (str_mut $s:expr,             ..= $u:literal) => { $crate::Str::range_to_inclusive_mut($s, $u) };
    (str_mut $s:expr,             ..  $u:literal) => { $crate::Str::range_to_mut($s, $u) };
    (
     str_checked $s:expr, $l:expr,    ..= $u:expr) => { $crate::Str::range_inclusive_checked($s, $l, $u) };
    (str_checked $s:expr, $l:expr,    ..  $u:expr) => { $crate::Str::range_checked($s, $l, $u) };
    (str_checked $s:expr, $l:expr,    ..         ) => { $crate::Str::range_from_checked($s, $l) };
    (str_checked $s:expr,             ..= $u:expr) => { $crate::Str::range_to_inclusive_checked($s, $u) };
    (str_checked $s:expr,             ..  $u:expr) => { $crate::Str::range_to_checked($s, $u) };
    (str_checked $s:expr, $l:block    ..= $u:block) => { $crate::Str::range_inclusive_checked($s, $l, $u) };
    (str_checked $s:expr, $l:block    ..  $u:block) => { $crate::Str::range_checked($s, $l, $u) };
    (str_checked $s:expr, $l:block    ..          ) => { $crate::Str::range_from_checked($s, $l) };
    (str_checked $s:expr,             ..= $u:block) => { $crate::Str::range_to_inclusive_checked($s, $u) };
    (str_checked $s:expr,             ..  $u:block) => { $crate::Str::range_to_checked($s, $u) };
    (str_checked $s:expr, $l:literal  ..= $u:literal) => { $crate::Str::range_inclusive_checked($s, $l, $u) };
    (str_checked $s:expr, $l:literal  ..  $u:literal) => { $crate::Str::range_checked($s, $l, $u) };
    (str_checked $s:expr, $l:literal  ..            ) => { $crate::Str::range_from_checked($s, $l) };
    (str_checked $s:expr,             ..= $u:literal) => { $crate::Str::range_to_inclusive_checked($s, $u) };
    (str_checked $s:expr,             ..  $u:literal) => { $crate::Str::range_to_checked($s, $u) };
    (
     str $s:expr, $l:expr,    ..= $u:expr) => { $crate::Str::range_inclusive($s, $l, $u) };
    (str $s:expr, $l:expr,    ..  $u:expr) => { $crate::Str::range($s, $l, $u) };
    (str $s:expr, $l:expr,    ..         ) => { $crate::Str::range_from($s, $l) };
    (str $s:expr,             ..= $u:expr) => { $crate::Str::range_to_inclusive($s, $u) };
    (str $s:expr,             ..  $u:expr) => { $crate::Str::range_to($s, $u) };
    (str $s:expr, $l:block    ..= $u:block) => { $crate::Str::range_inclusive($s, $l, $u) };
    (str $s:expr, $l:block    ..  $u:block) => { $crate::Str::range($s, $l, $u) };
    (str $s:expr, $l:block    ..          ) => { $crate::Str::range_from($s, $l) };
    (str $s:expr,             ..= $u:block) => { $crate::Str::range_to_inclusive($s, $u) };
    (str $s:expr,             ..  $u:block) => { $crate::Str::range_to($s, $u) };
    (str $s:expr, $l:literal  ..= $u:literal) => { $crate::Str::range_inclusive($s, $l, $u) };
    (str $s:expr, $l:literal  ..  $u:literal) => { $crate::Str::range($s, $l, $u) };
    (str $s:expr, $l:literal  ..            ) => { $crate::Str::range_from($s, $l) };
    (str $s:expr,             ..= $u:literal) => { $crate::Str::range_to_inclusive($s, $u) };
    (str $s:expr,             ..  $u:literal) => { $crate::Str::range_to($s, $u) };

    /* Slice */

    (
     mut_checked $s:expr, $l:expr,    ..= $u:expr) => { $crate::Slice::range_inclusive_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:expr,    ..  $u:expr) => { $crate::Slice::range_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:expr,    ..         ) => { $crate::Slice::range_from_mut_checked($s, $l) };
    (mut_checked $s:expr,             ..= $u:expr) => { $crate::Slice::range_to_inclusive_mut_checked($s, $u) };
    (mut_checked $s:expr,             ..  $u:expr) => { $crate::Slice::range_to_mut_checked($s, $u) };
    (mut_checked $s:expr, $l:block    ..= $u:block) => { $crate::Slice::range_inclusive_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:block    ..  $u:block) => { $crate::Slice::range_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:block    ..          ) => { $crate::Slice::range_from_mut_checked($s, $l) };
    (mut_checked $s:expr,             ..= $u:block) => { $crate::Slice::range_to_inclusive_mut_checked($s, $u) };
    (mut_checked $s:expr,             ..  $u:block) => { $crate::Slice::range_to_mut_checked($s, $u) };
    (mut_checked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_mut_checked($s, $l, $u) };
    (mut_checked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_mut_checked($s, $l) };
    (mut_checked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_mut_checked($s, $u) };
    (mut_checked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_mut_checked($s, $u) };
    (
     mut_unchecked $s:expr, $l:expr,    ..= $u:expr) => { $crate::Slice::range_inclusive_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:expr,    ..  $u:expr) => { $crate::Slice::range_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:expr,    ..         ) => { $crate::Slice::range_from_mut_unchecked($s, $l) };
    (mut_unchecked $s:expr,             ..= $u:expr) => { $crate::Slice::range_to_inclusive_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr,             ..  $u:expr) => { $crate::Slice::range_to_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr, $l:block    ..= $u:block) => { $crate::Slice::range_inclusive_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:block    ..  $u:block) => { $crate::Slice::range_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:block    ..          ) => { $crate::Slice::range_from_mut_unchecked($s, $l) };
    (mut_unchecked $s:expr,             ..= $u:block) => { $crate::Slice::range_to_inclusive_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr,             ..  $u:block) => { $crate::Slice::range_to_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_mut_unchecked($s, $l, $u) };
    (mut_unchecked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_mut_unchecked($s, $l) };
    (mut_unchecked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_mut_unchecked($s, $u) };
    (mut_unchecked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_mut_unchecked($s, $u) };
    (
     mut $s:expr, $l:expr,    ..= $u:expr) => { $crate::Slice::range_inclusive_mut($s, $l, $u) };
    (mut $s:expr, $l:expr,    ..  $u:expr) => { $crate::Slice::range_mut($s, $l, $u) };
    (mut $s:expr, $l:expr,    ..         ) => { $crate::Slice::range_from_mut($s, $l) };
    (mut $s:expr,             ..= $u:expr) => { $crate::Slice::range_to_inclusive_mut($s, $u) };
    (mut $s:expr,             ..  $u:expr) => { $crate::Slice::range_to_mut($s, $u) };
    (mut $s:expr, $l:block    ..= $u:block) => { $crate::Slice::range_inclusive_mut($s, $l, $u) };
    (mut $s:expr, $l:block    ..  $u:block) => { $crate::Slice::range_mut($s, $l, $u) };
    (mut $s:expr, $l:block    ..          ) => { $crate::Slice::range_from_mut($s, $l) };
    (mut $s:expr,             ..= $u:block) => { $crate::Slice::range_to_inclusive_mut($s, $u) };
    (mut $s:expr,             ..  $u:block) => { $crate::Slice::range_to_mut($s, $u) };
    (mut $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_mut($s, $l, $u) };
    (mut $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_mut($s, $l, $u) };
    (mut $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_mut($s, $l) };
    (mut $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_mut($s, $u) };
    (mut $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_mut($s, $u) };
    (
     checked $s:expr, $l:expr,    ..= $u:expr) => { $crate::Slice::range_inclusive_checked($s, $l, $u) };
    (checked $s:expr, $l:expr,    ..  $u:expr) => { $crate::Slice::range_checked($s, $l, $u) };
    (checked $s:expr, $l:expr,    ..         ) => { $crate::Slice::range_from_checked($s, $l) };
    (checked $s:expr,             ..= $u:expr) => { $crate::Slice::range_to_inclusive_checked($s, $u) };
    (checked $s:expr,             ..  $u:expr) => { $crate::Slice::range_to_checked($s, $u) };
    (checked $s:expr, $l:block    ..= $u:block) => { $crate::Slice::range_inclusive_checked($s, $l, $u) };
    (checked $s:expr, $l:block    ..  $u:block) => { $crate::Slice::range_checked($s, $l, $u) };
    (checked $s:expr, $l:block    ..          ) => { $crate::Slice::range_from_checked($s, $l) };
    (checked $s:expr,             ..= $u:block) => { $crate::Slice::range_to_inclusive_checked($s, $u) };
    (checked $s:expr,             ..  $u:block) => { $crate::Slice::range_to_checked($s, $u) };
    (checked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_checked($s, $l, $u) };
    (checked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_checked($s, $l, $u) };
    (checked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_checked($s, $l) };
    (checked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_checked($s, $u) };
    (checked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_checked($s, $u) };
    (
     unchecked $s:expr, $l:expr,    ..= $u:expr) => { $crate::Slice::range_inclusive_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:expr,    ..  $u:expr) => { $crate::Slice::range_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:expr,    ..         ) => { $crate::Slice::range_from_unchecked($s, $l) };
    (unchecked $s:expr,             ..= $u:expr) => { $crate::Slice::range_to_inclusive_unchecked($s, $u) };
    (unchecked $s:expr,             ..  $u:expr) => { $crate::Slice::range_to_unchecked($s, $u) };
    (unchecked $s:expr, $l:block    ..= $u:block) => { $crate::Slice::range_inclusive_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:block    ..  $u:block) => { $crate::Slice::range_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:block    ..          ) => { $crate::Slice::range_from_unchecked($s, $l) };
    (unchecked $s:expr,             ..= $u:block) => { $crate::Slice::range_to_inclusive_unchecked($s, $u) };
    (unchecked $s:expr,             ..  $u:block) => { $crate::Slice::range_to_unchecked($s, $u) };
    (unchecked $s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range_unchecked($s, $l, $u) };
    (unchecked $s:expr, $l:literal  ..            ) => { $crate::Slice::range_from_unchecked($s, $l) };
    (unchecked $s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive_unchecked($s, $u) };
    (unchecked $s:expr,             ..  $u:literal) => { $crate::Slice::range_to_unchecked($s, $u) };
    // needs to be the last matching syntax block:
    (
     $s:expr, $l:expr,  ..= $u:expr) => { $crate::Slice::range_inclusive($s, $l, $u) };
    ($s:expr, $l:expr,  ..  $u:expr) => { $crate::Slice::range($s, $l, $u) };
    ($s:expr, $l:expr,  ..         ) => { $crate::Slice::range_from($s, $l) };
    ($s:expr,           ..= $u:expr) => { $crate::Slice::range_to_inclusive($s, $u) };
    ($s:expr,           ..  $u:expr) => { $crate::Slice::range_to($s, $u) };
    ($s:expr, $l:block  ..= $u:block) => { $crate::Slice::range_inclusive($s, $l, $u) };
    ($s:expr, $l:block  ..  $u:block) => { $crate::Slice::range($s, $l, $u) };
    ($s:expr, $l:block  ..          ) => { $crate::Slice::range_from($s, $l) };
    ($s:expr,           ..= $u:block) => { $crate::Slice::range_to_inclusive($s, $u) };
    ($s:expr,           ..  $u:block) => { $crate::Slice::range_to($s, $u) };
    ($s:expr, $l:literal  ..= $u:literal) => { $crate::Slice::range_inclusive($s, $l, $u) };
    ($s:expr, $l:literal  ..  $u:literal) => { $crate::Slice::range($s, $l, $u) };
    ($s:expr, $l:literal  ..            ) => { $crate::Slice::range_from($s, $l) };
    ($s:expr,             ..= $u:literal) => { $crate::Slice::range_to_inclusive($s, $u) };
    ($s:expr,             ..  $u:literal) => { $crate::Slice::range_to($s, $u) };
    (
    $($t:tt)*) => { compile_error! { "Invalid interval syntax. Expected forms like: l..u, l..=u, ..x, ..=x" } };
}
#[doc(inline)]
pub use _slice as slice;
