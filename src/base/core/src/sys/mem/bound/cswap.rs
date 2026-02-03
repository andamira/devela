// devela_base_core::sys::mem::bound::cswap
//
//! Defines the [`cswap!`] macro.
//

#[doc = crate::_tags!(code)]
/// Swaps two mutable variables in a *compile-time* friendly manner.
#[doc = crate::_doc_location!("sys/mem")]
///
/// For that it uses either:
/// 1. a temporary variable.
/// 3. [`Mem::swap`][crate::Mem::swap] over their respective mutable references.
/// 3. the [xor swap method], making sure the values are not the same, first.
/// 4. the xor swap method, unchecked. If both values are the same they will get corrupted.
///
/// [xor swap method]: https://en.wikipedia.org/wiki/XOR_swap_algorithm
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cswap {
    (   // defaults to `tmp`
        $a:expr, $b:expr) => {{ $crate::cswap![tmp: $a, $b]; }};
    (xor // deprecated(since = "0.23.0", note = "Use `xor:`")
        $a:expr, $b:expr) => {{ $crate::cswap![xor: $a, $b]; }};
    (tmp:
        // swaps two values using a temporary variable.
        $a:expr, $b:expr) => {{ let tmp = $a; $a = $b; $b = tmp; }};
    (mut:
        // swaps two values by calling core's `swap` over their mut references.
        // $a:expr, $b:expr) => {{ $crate::Mem::swap(&mut $a, &mut $b); }}; // TODO
        $a:expr, $b:expr) => {{ ::core::mem::swap(&mut $a, &mut $b); }}; // TEMP
    (xor:
        // swaps two `T: PartialEq + BitXorAssign` values without a temporary variable.
        $a:expr, $b:expr) => {{ if $a != $b { $a ^= $b; $b ^= $a; $a ^= $b; } }};
    (xor_unchecked:
        // swaps two `T: PartialEq + BitXorAssign` values without a temporary variable,
        // without making sure they are both equal, in which case they'll get corrupted.
        $a:expr, $b:expr) => {{ $a ^= $b; $b ^= $a; $a ^= $b; }};
}
#[doc(inline)]
pub use cswap;
