// devela::num::quant::interval
//
//! Defines the [`Interval`] wrapper type.
//
// TOC
// - macro interval!
// - struct Interval
//   - impls
//   - impl traits
// - tests

use crate::{
    Bound, ConstDefault, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive, is,
};

#[doc = crate::TAG_QUANT!()]
/// Creates an [`Interval`] using extended range notation.
///
/// # Syntax
///
/// ## Closed Lower Bound `[l, …`
/// ```text
/// [l, u]    interval![l, ..= u]    // closed-closed
/// [l, u)    interval![l, .. u]     // closed-open
/// [l, ∞)    interval![l, ..]       // closed-unbounded
/// ```
///
/// ## Open Lower Bound `(l, …` *extension syntax*
/// ```text
/// (l, u)    interval![l <.. u]     // open-open
/// (l, u]    interval![l <..= u]    // open-closed
/// (l, ∞)    interval![l <..]       // open-unbounded
/// ```
///
/// ## Unbounded Lower Bound `(-∞, …`
/// ```text
/// (-∞, u]   interval![..= u]       // unbounded-closed
/// (-∞, u)   interval![.. u]        // unbounded-open
/// (-∞, ∞)   interval![..]          // fully unbounded (inferred type)
/// (-∞, ∞)   interval![.., T]       // fully unbounded (explicit type) (e.g., `.., i32`)
/// ```
///
/// # Notes
/// - Commas required for expressions: `interval![a(), ..= b()]`
/// - Optional for literals/blocks: `interval![1..=3]` ≡ `interval![1, ..=3]`
/// - `<..` indicates open-left bounds (non-standard Rust extension)
///
/// # Example
/// ```
/// # use devela::interval;
/// # let (x, y) = (10, 20);
/// # fn calc() -> i32 { 10 }
/// # fn other() -> i32 { 20 }
/// // Literals
/// interval![1 ..=3];    // [1, 3]
/// interval![1<.. 3];    // (1, 3)
///
/// // Expressions
/// interval![(x+1), ..= (y*2)];
/// interval![{ calc() }, <.. { other() }];
///
/// // Unbounded
/// interval![.., i32]; // (-∞, ∞) as i32
/// interval![..=10];   // (-∞, 10]
/// interval![1..];     // [1, ∞)
/// ```
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! _interval {
    (
    /* expressions */

     $l:expr,  ..= $u:expr) => { $crate::Interval::closed($l, $u) }; // lower closed
    ($l:expr,  ..  $u:expr) => { $crate::Interval::closed_open($l, $u) };
    ($l:expr,  ..         ) => { $crate::Interval::closed_unbounded($l) };
    ($l:expr, <..  $u:expr) => { $crate::Interval::open($l, $u) }; // lower open
    ($l:expr, <..= $u:expr) => { $crate::Interval::open_closed($l, $u) };
    ($l:expr, <..         ) => { $crate::Interval::open_unbounded($l) };
    (          ..= $u:expr) => { $crate::Interval::unbounded_closed($u) }; // lower unbounded
    (          ..  $u:expr) => { $crate::Interval::unbounded_open($u) };
    (
    /* blocks */

     $l:block  ..= $u:block) => { $crate::Interval::closed($l, $u) }; // lower closed
    ($l:block  ..  $u:block) => { $crate::Interval::closed_open($l, $u) };
    ($l:block  ..          ) => { $crate::Interval::closed_unbounded($l) };
    ($l:block <..  $u:block) => { $crate::Interval::open($l, $u) }; // lower open
    ($l:block <..= $u:block) => { $crate::Interval::open_closed($l, $u) };
    ($l:block <..          ) => { $crate::Interval::open_unbounded($l) };
    (          ..= $u:block) => { $crate::Interval::unbounded_closed($u) }; // lower unbounded
    (          ..  $u:block) => { $crate::Interval::unbounded_open($u) };
    (
    /* literals */

     $l:literal  ..= $u:literal) => { $crate::Interval::closed($l, $u) }; // lower closed
    ($l:literal  ..  $u:literal) => { $crate::Interval::closed_open($l, $u) };
    ($l:literal  ..            ) => { $crate::Interval::closed_unbounded($l) };
    ($l:literal <..  $u:literal) => { $crate::Interval::open($l, $u) }; // lower open
    ($l:literal <..= $u:literal) => { $crate::Interval::open_closed($l, $u) };
    ($l:literal <..            ) => { $crate::Interval::open_unbounded($l) };
    (            ..= $u:literal) => { $crate::Interval::unbounded_closed($u) }; // lower unbounded
    (            ..  $u:literal) => { $crate::Interval::unbounded_open($u) };
    (
    /* fully unbounded variants (explicit and inferred type) */

     .., $T:ty) => { $crate::Interval::<$T>::unbounded() }; // has to come first
    (..       ) => { $crate::Interval::unbounded() };

    (
    /* syntax error msg */

    $($t:tt)*) => {
        compile_error! { "Invalid interval syntax. Expected forms like: \
                      l..u, l..=u, l<..u, l<..=u, ..x, ..=x, ..,Type" }
    };
}
#[doc(inline)]
pub use _interval as interval;

#[doc = crate::TAG_QUANT!()]
/// A range of values with `lower` and `upper` [`Bound`]s.
///
/// The `Interval` type allows modeling ranges of values with optional inclusion
/// or exclusion at each bound. This is useful for mathematical operations,
/// range checks, and interval arithmetic.
///
/// See also the [`interval!`] macro.
#[doc(alias = "Range")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Interval<T> {
    /// The lower bound (also known as the *start* bound, or the *left* bound).
    pub lower: Bound<T>,
    /// The upper bound (also known as the *end*, bound or the *right* bound).
    pub upper: Bound<T>,
}

/// # Methodical constructors
impl<T> Interval<T> {
    // lower-closed

    /// Creates a closed interval $[l, u]$ `lower..=upper` [`RangeInclusive`].
    #[must_use]
    pub const fn closed(lower: T, upper: T) -> Self {
        Self::new(Bound::Included(lower), Bound::Included(upper))
    }
    /// Creates a half-open interval $[l, u)$ `lower..upper` [`Range`].
    #[must_use]
    pub const fn closed_open(lower: T, upper: T) -> Self {
        Self::new(Bound::Included(lower), Bound::Excluded(upper))
    }
    /// Creates an interval $[l, ∞)$ `lower..` [`RangeFrom`].
    #[must_use]
    pub const fn closed_unbounded(lower: T) -> Self {
        Self::new(Bound::Included(lower), Bound::Unbounded)
    }

    // lower-open

    /// Creates an open interval $(l, u)$ *`lower..<upper`*.
    #[must_use]
    pub const fn open(lower: T, upper: T) -> Self {
        Self::new(Bound::Excluded(lower), Bound::Excluded(upper))
    }

    /// Creates a half-open interval $(l, u]$ *`lower..<=upper`*.
    #[must_use]
    pub const fn open_closed(lower: T, upper: T) -> Self {
        Self::new(Bound::Excluded(lower), Bound::Included(upper))
    }

    /// Creates an interval $(l, ∞)$ *`lower..<`*.
    #[must_use]
    pub const fn open_unbounded(lower: T) -> Self {
        Self::new(Bound::Excluded(lower), Bound::Unbounded)
    }

    // lower-unbounded

    /// Creates an unbounded interval $(-∞, ∞)$ `..` [`RangeFull`].
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::new(Bound::Unbounded, Bound::Unbounded)
    }
    /// Creates an interval $(-∞, u]$ `..upper` [`RangeTo`].
    #[must_use]
    pub const fn unbounded_closed(upper: T) -> Self {
        Self::new(Bound::Unbounded, Bound::Included(upper))
    }
    /// Creates an interval $(-∞, u)$ `..=upper` [`RangeToInclusive`].
    #[must_use]
    pub const fn unbounded_open(upper: T) -> Self {
        Self::new(Bound::Unbounded, Bound::Excluded(upper))
    }
}

/// # Additional constructors
impl<T> Interval<T> {
    /// Creates a new interval with the given `lower` and `upper` bounds.
    #[must_use]
    pub const fn new(lower: Bound<T>, upper: Bound<T>) -> Self {
        Self { lower, upper }
    }

    /// Creates a single-point interval,
    /// equivalent to [`closed`][Interval::closed]`(value, value)`.
    #[must_use] #[rustfmt::skip]
    pub fn point(value: T) -> Self where T: Clone {
        Self::closed(value.clone(), value)
    }

    /// Creates a canonical empty interval,
    /// equivalent to [`open`][Interval::open]`(T::default(), T::default())`.
    #[must_use] #[rustfmt::skip]
    pub fn empty() -> Self where T: Default {
        Self::open(T::default(), T::default())
    }
    /// Creates a canonical empty interval,
    /// equivalent to [`open`][Interval::open]`(T::default(), T::default())`.
    #[must_use] #[rustfmt::skip]
    pub const fn empty_const() -> Self where T: ConstDefault {
        Self::open(T::DEFAULT, T::DEFAULT)
    }

    /// Creates a canonical empty interval,
    /// equivalent to [`open`][Interval::open]`(value, value)`.
    #[must_use] #[rustfmt::skip]
    pub fn empty_with(value: T) -> Self where T: Clone {
        Self::open(value.clone(), value)
    }
}

impl<T: Copy> Interval<T> {
    /// Returns a copy of both bounds as a tuple `(lower, upper)`.
    ///
    /// # Example
    /// ```
    /// # use devela::Interval;
    /// let r = Interval::from(1..3usize);
    /// assert_eq!("bc", &"abcd"[r.to_tuple()]);
    /// ```
    #[must_use]
    pub const fn to_tuple(self) -> (Bound<T>, Bound<T>) {
        (self.lower, self.upper)
    }
}

impl<T> Interval<T> {
    /// Returns both bounds as a tuple `(lower, upper)`.
    #[must_use]
    pub fn into_tuple(self) -> (Bound<T>, Bound<T>) {
        (self.lower, self.upper)
    }

    /// Returns a reference to both bounds as a tuple `(&lower, &upper)`.
    #[must_use]
    pub fn to_tuple_ref(&self) -> (Bound<&T>, Bound<&T>) {
        (self.lower_ref(), self.upper_ref())
    }

    /// Returns a reference to the lower bound.
    #[must_use]
    pub fn lower_ref(&self) -> Bound<&T> {
        self.lower.as_ref()
    }

    /// Returns a reference to the upper bound.
    #[must_use]
    pub fn upper_ref(&self) -> Bound<&T> {
        self.upper.as_ref()
    }

    /// Checks if the interval is both lower and upper bounded.
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        self.is_lower_bounded() && self.is_upper_bounded()
    }
    /// Checks if the lower bound is bounded.
    #[must_use]
    pub const fn is_lower_bounded(&self) -> bool {
        !matches!(self.lower, Bound::Unbounded)
    }
    /// Checks if the upper bound is bounded.
    #[must_use]
    pub const fn is_upper_bounded(&self) -> bool {
        !matches!(self.upper, Bound::Unbounded)
    }
    /// Checks if the lower bound is open (excluded).
    #[must_use]
    pub const fn is_lower_open(&self) -> bool {
        matches!(self.lower, Bound::Excluded(_))
    }
    /// Checks if the lower bound is closed (included).
    #[must_use]
    pub const fn is_lower_closed(&self) -> bool {
        matches!(self.lower, Bound::Included(_))
    }
    /// Checks if the upper bound is open (excluded).
    #[must_use]
    pub const fn is_upper_open(&self) -> bool {
        matches!(self.upper, Bound::Excluded(_))
    }
    /// Checks if the upper bound is closed (included).
    #[must_use]
    pub const fn is_upper_closed(&self) -> bool {
        matches!(self.upper, Bound::Included(_))
    }
}

impl<T: PartialOrd> Interval<T> {
    /// Checks if the interval is empty (contains no values).
    ///
    /// An interval is empty if:
    /// - The bounds exclude each other, such as `(x, x)`, `[x, x)`, or `(x, x]`.
    /// - The `lower` bound is strictly greater than the `upper` bound.
    ///
    /// Unbounded intervals are never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match (&self.lower, &self.upper) {
            (Bound::Unbounded, _) | (_, Bound::Unbounded) => false,
            (Bound::Included(l), Bound::Included(u)) => l > u,
            (Bound::Included(l), Bound::Excluded(u)) => l >= u,
            (Bound::Excluded(l), Bound::Included(u)) => l >= u,
            (Bound::Excluded(l), Bound::Excluded(u)) => l >= u,
        }
    }

    /// Validates that the interval bounds are ordered correctly.
    ///
    /// Returns `true` if the lower bound is less than or equal to the upper bound.
    /// Unbounded intervals are always considered well ordered.
    #[must_use]
    pub fn is_well_ordered(&self) -> bool {
        match (&self.lower, &self.upper) {
            (Bound::Unbounded, _) | (_, Bound::Unbounded) => true,
            (Bound::Included(l), Bound::Included(u)) => l <= u,
            (Bound::Included(l), Bound::Excluded(u)) => l < u,
            (Bound::Excluded(l), Bound::Included(u)) => l < u,
            (Bound::Excluded(l), Bound::Excluded(u)) => l < u,
        }
    }

    /// Checks if the interval contains the given value.
    ///
    /// ```
    /// # use devela::Interval;
    /// let interval = Interval::closed(1, 5);
    /// assert!(interval.contains(&3));
    /// assert!(!interval.contains(&6));
    /// ```
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        let lower_check = match &self.lower {
            Bound::Included(lower) => *lower <= *value,
            Bound::Excluded(lower) => *lower < *value,
            Bound::Unbounded => true,
        };
        let upper_check = match &self.upper {
            Bound::Included(upper) => *value <= *upper,
            Bound::Excluded(upper) => *value < *upper,
            Bound::Unbounded => true,
        };
        lower_check && upper_check
    }

    /// Returns the size of the interval, if finite.
    #[must_use]
    pub fn size(&self) -> Option<T>
    where
        T: Clone + core::ops::Sub<Output = T>,
    {
        match (&self.lower, &self.upper) {
            (Bound::Included(l), Bound::Included(u)) => {
                is![l <= u; Some(u.clone() - l.clone()); None]
            }
            (Bound::Included(l), Bound::Excluded(u)) => {
                is![l < u; Some(u.clone() - l.clone()); None]
            }
            (Bound::Excluded(l), Bound::Included(u)) => {
                is![l < u; Some(u.clone() - l.clone()); None]
            }
            (Bound::Excluded(l), Bound::Excluded(u)) => {
                is![l < u; Some(u.clone() - l.clone()); None]
            }
            _ => None, // Unbounded intervals don't have a finite size
        }
    }
}

#[rustfmt::skip]
mod impl_traits {
    use super::*;
    use crate::{
        ConstDefault, NumError, NumError::IncompatibleBounds, NumResult, Ordering, RangeBounds,
    };

    /// Provides a default value for `Interval`, the unbounded interval $(-\infty, \infty)$.
    ///
    /// This choice emphasizes neutrality and generality,
    /// where the interval encompasses all possible values of `T`. It:
    /// - Represents a neutral and maximal range for generic use cases.
    /// - Avoids reliance on [`Default`] for `T`, making it applicable to all types.
    /// - Aligns with mathematical conventions, where unbounded intervals are a natural default.
    impl<T> Default for Interval<T> {
        fn default() -> Self {
            Self::unbounded()
        }
    }
    /// Provides a *const* default value for `Interval`, the unbounded interval $(-\infty, \infty)$.
    ///
    /// See the [`Default`][Self::default] implementation for more information.
    ///
    /// See [`Default`] for more information.
    impl<T> ConstDefault for Interval<T> {
        const DEFAULT: Self = Self::unbounded();
    }

    /* infallible conversions */

    // lower-closed
    impl<T> From<RangeInclusive<T>> for Interval<T> {
        fn from(r: RangeInclusive<T>) -> Self {
            let (start, end) = r.into_inner();
            Self::closed(start, end)
        }
    }
    impl<T> From<Range<T>> for Interval<T> {
        fn from(r: Range<T>) -> Self { Self::closed_open(r.start, r.end) }
    }
    impl<T> From<RangeFrom<T>> for Interval<T> {
        fn from(r: RangeFrom<T>) -> Self { Self::closed_unbounded(r.start) }
    }
    // lower-unbounded
    impl<T> From<RangeFull> for Interval<T> {
        fn from(_: RangeFull) -> Self { Self::unbounded() }
    }
    impl<T> From<RangeTo<T>> for Interval<T> {
        fn from(r: RangeTo<T>) -> Self { Self::unbounded_closed(r.end) }
    }
    impl<T> From<RangeToInclusive<T>> for Interval<T> {
        fn from(r: RangeToInclusive<T>) -> Self { Self::unbounded_open(r.end) }
    }

    /* fallible conversions */

    // lower-closed
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeInclusive<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Included(start), Bound::Included(end)) => Ok(start..=end),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for Range<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Included(start), Bound::Excluded(end)) => Ok(start..end),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeFrom<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Included(start), Bound::Unbounded) => Ok(start..),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    // lower-unbounded
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeFull {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Unbounded, Bound::Unbounded) => Ok(..),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeTo<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Unbounded, Bound::Excluded(end)) => Ok(..end),
                _ => Err(IncompatibleBounds),
            }
        }
    }
    /// # Errors
    /// Returns [`IncompatibleBounds`] if the bounds are not compatible.
    impl<T> TryFrom<Interval<T>> for RangeToInclusive<T> {
        type Error = NumError;
        fn try_from(interval: Interval<T>) -> NumResult<Self> {
            match (interval.lower, interval.upper) {
                (Bound::Unbounded, Bound::Included(end)) => Ok(..=end),
                _ => Err(IncompatibleBounds),
            }
        }
    }

    /* other traits */

    impl<T> RangeBounds<T> for Interval<T> {
        fn start_bound(&self) -> Bound<&T> {
            self.lower_ref()
        }
        fn end_bound(&self) -> Bound<&T> {
            self.upper_ref()
        }
    }

    #[cfg(feature = "alloc")]
    crate::items! {
        use crate::{format, String};
        use core::fmt;

        impl<T: fmt::Display> fmt::Display for Interval<T> {
            /// Formats the interval as a human-readable string.
            ///
            /// Examples:
            /// - `(-∞, 5]` for an unbounded lower bound and inclusive upper bound.
            /// - `[1, 3)` for a closed lower bound and open upper bound.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let lower = match &self.lower {
                    Bound::Included(value) => format!("[{}", value),
                    Bound::Excluded(value) => format!("({}", value),
                    Bound::Unbounded => String::from("(-∞"),
                };
                let upper = match &self.upper {
                    Bound::Included(value) => format!("{}, {}]", lower, value),
                    Bound::Excluded(value) => format!("{}, {})", lower, value),
                    Bound::Unbounded => format!("{}, ∞)", lower),
                };
                write!(f, "{}", upper)
            }
        }
    }

    /// Comparison Logic:
    /// - We compare the lower bounds first.
    /// - If the lower bounds are equal, we compare the upper bounds.
    /// - We define Unbounded as less than any bounded value.
    /// - We define that Included(l) < Excluded(l) at same point l.
    impl<T: PartialOrd> PartialOrd for Interval<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match compare_bounds(&self.lower, &other.lower) {
                Some(Ordering::Equal) => compare_bounds(&self.upper, &other.upper),
                ord => ord,
            }
        }
    }
    /// Comparison Logic:
    /// - We compare the lower bounds first.
    /// - If the lower bounds are equal, we compare the upper bounds.
    /// - We define Unbounded as less than any bounded value.
    /// - We define that Included(l) < Excluded(l) at same point l.
    impl<T: Ord> Ord for Interval<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            match compare_bounds_ord(&self.lower, &other.lower) {
                Ordering::Equal => compare_bounds_ord(&self.upper, &other.upper),
                ord => ord,
            }
        }
    }

    /* helpers */

    fn compare_bounds<T: PartialOrd>(l: &Bound<T>, u: &Bound<T>) -> Option<Ordering> {
        use Bound::{Excluded, Included, Unbounded};
        match (l, u) {
            (Unbounded, Unbounded) => Some(Ordering::Equal),
            (Unbounded, _) => Some(Ordering::Less),
            (_, Unbounded) => Some(Ordering::Greater),
            (Included(a_val), Included(b_val)) => a_val.partial_cmp(b_val),
            (Excluded(a_val), Excluded(b_val)) => a_val.partial_cmp(b_val),
            (Included(a_val), Excluded(b_val)) => {
                match a_val.partial_cmp(b_val) {
                    Some(Ordering::Equal) => Some(Ordering::Less),
                    ord => ord,
                }
            }
            (Excluded(a_val), Included(b_val)) => {
                match a_val.partial_cmp(b_val) {
                    Some(Ordering::Equal) => Some(Ordering::Greater),
                    ord => ord,
                }
            }
        }
    }
    fn compare_bounds_ord<T: Ord>(l: &Bound<T>, u: &Bound<T>) -> Ordering {
        use Bound::{Excluded, Included, Unbounded};
        match (l, u) {
            (Unbounded, Unbounded) => Ordering::Equal,
            (Unbounded, _) => Ordering::Less,
            (_, Unbounded) => Ordering::Greater,
            (Included(a_val), Included(b_val)) => a_val.cmp(b_val),
            (Excluded(a_val), Excluded(b_val)) => a_val.cmp(b_val),
            (Included(a_val), Excluded(b_val)) => {
                match a_val.cmp(b_val) {
                    Ordering::Equal => Ordering::Less,
                    ord => ord,
                }
            }
            (Excluded(a_val), Included(b_val)) => {
                match a_val.cmp(b_val) {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                }
            }
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::{Interval, interval};

    #[test]
    #[allow(unused_parens, reason = "testing expressions")]
    fn interval_macro_expr() {
        assert_eq![interval![(5+5), ..= (10+10)], Interval::closed(10, 20)];
        assert_eq![interval![(5+5), ..  (10+10)], Interval::closed_open(10, 20)];
        assert_eq![interval![(5+5), ..         ], Interval::closed_unbounded(10)];
        //
        assert_eq![interval![(5+5), <..  (10+10)], Interval::open(10, 20)];
        assert_eq![interval![(5+5), <..= (10+10)], Interval::open_closed(10, 20)];
        assert_eq![interval![(5+5), <..         ], Interval::open_unbounded(10)];
        //
        assert_eq![interval![.., i32    ], Interval::<i32>::unbounded()]; // explicit type
        assert_eq![interval![..= (10+10)], Interval::unbounded_closed(20)];
        assert_eq![interval![..  (10+10)], Interval::unbounded_open(20)];
    }
    #[test]
    #[allow(unused_braces, reason = "testing blocks")]
    fn interval_macro_block() {
        assert_eq![interval![{2*5} ..= {2*10}], Interval::closed(10, 20)];
        assert_eq![interval![{2*5} ..  {2*10}], Interval::closed_open(10, 20)];
        assert_eq![interval![{2*5} ..        ], Interval::closed_unbounded(10)];
        //
        assert_eq![interval![{2*5} <..  {2*10}], Interval::open(10, 20)];
        assert_eq![interval![{2*5} <..= {2*10}], Interval::open_closed(10, 20)];
        assert_eq![interval![{2*5} <..        ], Interval::open_unbounded(10)];
        //
        // assert_eq![interval![..        ], Interval::<i32>::unbounded()]; // =↓ type inference
        assert_eq![interval![..= {2*10}], Interval::unbounded_closed(20)];
        assert_eq![interval![..  {2*10}], Interval::unbounded_open(20)];
    }
    #[test]
    fn interval_macro_literal() {
        assert_eq![interval![10 ..= 20], Interval::closed(10, 20)];
        assert_eq![interval![10 ..  20], Interval::closed_open(10, 20)];
        assert_eq![interval![10 ..    ], Interval::closed_unbounded(10)];
        //
        assert_eq![interval![10 <..  20], Interval::open(10, 20)];
        assert_eq![interval![10 <..= 20], Interval::open_closed(10, 20)];
        assert_eq![interval![10 <..    ], Interval::open_unbounded(10)];
        //
        assert_eq![interval![..    ], Interval::<i32>::unbounded()]; // type inference
        assert_eq![interval![..= 20], Interval::unbounded_closed(20)];
        assert_eq![interval![..  20], Interval::unbounded_open(20)];
    }
    // #[test] #[should_panic]
    // fn interval_macro_error() { let e = interval![10 => 20]; }
}
