// devela/src/sys/mem/view/slice/namespace/mod.rs
//
//! Defines [`slice!`].
//

#[cfg(doc)]
use crate::Str;

#[doc = crate::_tags!(namespace)]
/// Invokes [`Slice`] [range methods][Slice#range-api-methods-for-subslicing] using short notation.
#[doc = crate::_doc_meta!{location("sys/mem/view")}]
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
macro_rules! slice· {
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
pub use slice· as slice;
