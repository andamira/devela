//
//! Defines [`seq!`].
//

#[doc = crate::_tags!(construction num)]
/// Defines a zero-sized namespace for indexed mathematical sequences.
#[doc = crate::_doc_meta!{
    location("num/fin", macro seq),
}]
/// Selects unsigned primitive carriers and individual operations explicitly.
///
/// - Supported carriers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.
/// - Supported operations: `thue_morse`, `prouhet`, `gray`,
/// `van_der_corput`, `fibonacci`, or `all` for every operation.
///
/// All indices are zero-based.
///
/// # Examples
/// ```
/// use devela::seq;
/// seq! { pub struct S: (u32) impl; ops(thue_morse, gray); }
/// assert_eq!(S::thue_morse(7), 1);
/// assert_eq!(S::gray(7), 4);
/// seq! { pub struct G: <> impl (u8, u64); ops(fibonacci); }
/// assert_eq!(G::<u8>::fibonacci(13), Some(233));
/// assert_eq!(G::<u64>::fibonacci(20), Some(6765));
/// ```
///
/// Unselected methods are not generated:
/// ```compile_fail
/// use devela::seq;
/// seq! { struct S: (u8) impl; ops(gray); }
/// let _ = S::fibonacci(3);
/// ```
/// Signed carriers are not supported:
/// ```compile_fail
/// use devela::seq;
/// seq! { struct S: (i32) impl; ops(gray); }
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! seq· {
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : ($T:ident) impl;
        ops($($op:ident),* $(,)?);) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug)]
        $vis struct $Name;
        $crate::__seq!(%carrier $T);
        impl $Name {
            $crate::__seq!(%ops $T; $($op),*);
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : <> impl ($($T:ident),+ $(,)?);
        ops($($op:ident),* $(,)?);) => {
        $crate::seq! { $(#[$attr])* $vis struct $Name: <>; }
        $crate::seq! { impl $Name: <> ($($T),+); ops($($op),*); }
    };
    ($(#[$attr:meta])* $vis:vis struct $Name:ident : <>;) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug)]
        $vis struct $Name<T>(::core::marker::PhantomData<T>);
    };
    (impl $Name:ident : <> ($($T:ident),+ $(,)?); ops($($op:ident),* $(,)?);) => {
        $crate::__seq!(%generic $Name; [$($op),*]; $($T),+);
    };
}
#[doc(inline)]
pub use seq· as seq;
