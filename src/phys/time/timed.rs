// devela::phys::time::timed
//
//! Defines [`Timed`] and [`MaybeTimed`].
//

use crate::ConstInit;

#[doc = crate::_tags!(time)]
/// A value paired with a time-like companion.
#[doc = crate::_doc_location!("phys/time")]
///
/// `Timed<V, T>` is a minimal carrier for attaching timing information to a value,
/// without assuming a specific source, origin, or policy.
///
/// The `T` parameter is usually a concrete time point, timestamp, or elapsed-like
/// quantity, depending on the domain.
///
/// # Notes
/// - `Timed` itself does not require `T` to implement [`TimePoint`].
/// - It is intentionally generic so it can carry backend timestamps, sampled values,
///   profiling records, or intermediate normalized event data.
///
/// See also [`MaybeTimed`].
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Timed<V, T> {
    /// The carried value.
    pub value: V,
    /// The associated time-like companion.
    pub time: T,
}

#[doc = crate::_tags!(time maybe)]
/// A value paired with an optional time-like companion.
#[doc = crate::_doc_location!("phys/time")]
pub type MaybeTimed<V, T> = Timed<V, Option<T>>;

#[rustfmt::skip]
impl<V, T> Timed<V, T> {
    /// Creates a new timed value.
    #[inline(always)]
    pub const fn new(value: V, time: T) -> Self { Self { value, time } }

    /// Splits this timed value into `(value, time)` copies.
    #[inline(always)]
    pub const fn copy_parts(self) -> (V, T) where V: Copy, T: Copy { (self.value, self.time) }

    /// Splits this timed value into `(value, time)`.
    #[inline(always)]
    pub fn into_parts(self) -> (V, T) { (self.value, self.time) }

    /// Borrows both fields.
    #[inline(always)]
    pub const fn as_ref(&self) -> Timed<&V, &T> {
        Timed { value: &self.value, time: &self.time }
    }
    /// Mutably borrows both fields.
    #[inline(always)]
    pub const fn as_mut(&mut self) -> Timed<&mut V, &mut T> {
        Timed { value: &mut self.value, time: &mut self.time }
    }

    /// Maps the carried value, preserving the time companion.
    #[inline(always)]
    pub fn map_value<V2, F>(self, f: F) -> Timed<V2, T>
    where
        F: FnOnce(V) -> V2,
    {
        Timed { value: f(self.value), time: self.time }
    }

    /// Maps the time companion, preserving the carried value.
    #[inline(always)]
    pub fn map_time<T2, F>(self, f: F) -> Timed<V, T2>
    where
        F: FnOnce(T) -> T2,
    {
        Timed { value: self.value, time: f(self.time) }
    }

    /// Maps both fields at once.
    #[inline(always)]
    pub fn map<V2, T2, FV, FT>(self, fv: FV, ft: FT) -> Timed<V2, T2>
    where
        FV: FnOnce(V) -> V2,
        FT: FnOnce(T) -> T2,
    {
        Timed { value: fv(self.value), time: ft(self.time) }
    }
}

impl<V: ConstInit, T: ConstInit> ConstInit for Timed<V, T> {
    const INIT: Self = Self { value: V::INIT, time: T::INIT };
}
