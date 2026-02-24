// devela_base_core::num::fin::ord::cmp::macros
//
//! Defines [`cmp!`].
//

#[rustfmt::skip]
#[doc = crate::_tags!(num)]
/// Provides comparison operations using an operation-first syntax.
#[doc = crate::_doc_location!("num/fin/ord")]
///
/// A thin macro wrapper over [`Cmp`][crate::Cmp].
/// Expands directly to the corresponding `Cmp` method.
///
/// ```rust
/// # use devela_base_core::{Cmp, cmp};
/// let [a, b] = [1u8, 2];
/// assert_eq![
///     cmp!(min a b),
///     Cmp(a).min(b)
/// ];
/// ```
///
/// Notes:
/// - `minmax` / `pminmax` return `(min, max)`.
/// - `total x y` returns an [`Ordering`] using total ordering (floats included).
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _cmp {
    (
    // partial cmp

    pclamp $v:tt $x:tt $y:tt) => { $crate::Cmp($v).pclamp($x, $y) };
    (pclamp $v:expr, $x:expr, $y:expr) => { $crate::Cmp($v).pclamp($x, $y) };
    (pmax $x:tt $y:tt) => { $crate::Cmp($x).pmax($y) };
    (pmax $x:expr, $y:expr) => { $crate::Cmp($x).pmax($y) };
    (pmin $x:tt $y:tt) => { $crate::Cmp($x).pmin($y) };
    (pmin $x:expr, $y:expr) => { $crate::Cmp($x).pmin($y) };
    (pminmax $x:tt $y:tt) => { $crate::Cmp($x).pminmax($y) };
    (pminmax $x:expr, $y:expr) => { $crate::Cmp($x).pminmax($y) };
    (
    // total cmp

    total $x:tt $y:tt) => { $crate::Cmp($x).total_cmp($y) };
    (total $x:expr, $y:expr) => { $crate::Cmp($x).total_cmp($y) };
    (clamp $v:tt $x:tt $y:tt) => { $crate::Cmp($v).clamp($x, $y) };
    (clamp $x:expr, $y:expr) => { $crate::Cmp($x).clamp($x, $y) };
    (max $x:tt $y:tt) => { $crate::Cmp($x).max($y) };
    (max $x:expr, $y:expr) => { $crate::Cmp($x).max($y) };
    (min $x:tt $y:tt) => { $crate::Cmp($x).min($y) };
    (min $x:expr, $y:expr) => { $crate::Cmp($x).min($y) };
    (minmax $x:tt $y:tt) => { $crate::Cmp($x).minmax($y) };
    (minmax $x:expr, $y:expr) => { $crate::Cmp($x).minmax($y) };
    (
    // equality

    eq $x:tt $y:tt) => { $crate::Cmp($x).eq($y) };
    (eq $x:expr, $y:expr) => { $crate::Cmp($x).eq($y) };
    (ne $x:tt $y:tt) => { $crate::Cmp($x).ne($y) };
    (ne $x:expr, $y:expr) => { $crate::Cmp($x).ne($y) };
    (lt $x:tt $y:tt) => { $crate::Cmp($x).lt($y) };
    (lt $x:expr, $y:expr) => { $crate::Cmp($x).lt($y) };
    (le $x:tt $y:tt) => { $crate::Cmp($x).le($y) };
    (le $x:expr, $y:expr) => { $crate::Cmp($x).le($y) };
    (gt $x:tt $y:tt) => { $crate::Cmp($x).gt($y) };
    (gt $x:expr, $y:expr) => { $crate::Cmp($x).gt($y) };
    (ge $x:tt $y:tt) => { $crate::Cmp($x).ge($y) };
    (ge $x:expr, $y:expr) => { $crate::Cmp($x).ge($y) };
}
#[doc(inline)]
pub use _cmp as cmp;

#[cfg(test)]
mod tests {
    use crate::cmp;

    #[test]
    fn test_cmp() {
        let [a, b] = [1_u32, 2];

        assert_eq![cmp!(min a b), a];
        assert_eq![cmp!(min a 4), a];
        assert_eq![cmp!(min a + 1, b * 2), a + 1];
        assert_eq![cmp!(max a b), b];
        assert_eq![cmp!(minmax a b), (a, b)];
        assert_eq![cmp!(pmin 1.0_f32, f32::NAN), None];
    }
}
