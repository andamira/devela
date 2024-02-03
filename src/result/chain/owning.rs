// devela::result::chain::owning
//
//!
//

/// Encapsulates the result of an operation that owns `self` and returns it with another value.
///
/// This struct is designed to be used with methods where ownership is taken by the method,
/// and a new state of the original object is returned alongside some operation-specific result.
/// It's particularly useful for fluent and chainable API designs in Rust, where methods
/// need to return both a transformed version of `self` and an operation result.
#[must_use]
pub struct Owning<SELF, T> {
    /// The new owned instance of the object after the operation.
    ///
    /// This represents the object's state following the transformative operation.
    pub owned: SELF,

    /// The value resulting from the operation.
    ///
    /// This field is intended to hold any operation-specific outcome.
    pub value: T,
}

impl<SELF, T> Owning<SELF, T> {
    ///
    #[inline]
    pub const fn new(owned: SELF, value: T) -> Self {
        Owning { owned, value }
    }

    /// Destructures itself into a tuple.
    #[inline]
    #[must_use]
    pub fn into_tuple(self) -> (SELF, T) {
        (self.owned, self.value)
    }
    /// Destructures itself returning its `owned` field
    #[inline]
    #[must_use]
    pub fn into_owned(self) -> SELF {
        self.owned
    }
    /// Destructures itself returning its `value` field
    #[inline]
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// Returns a reference to the `owned` field without consuming the `Owning` instance.
    #[inline]
    #[must_use]
    pub const fn ref_owned(&self) -> &SELF {
        &self.owned
    }
    /// Returns a reference to the `value` field without consuming the `Owning` instance.
    #[inline]
    #[must_use]
    pub const fn ref_value(&self) -> &T {
        &self.value
    }

    /// Returns `true` if the owned `value` equals the given `expected` value.
    #[inline]
    #[must_use]
    pub fn is_value(&self, expected: &T) -> bool
    where
        T: PartialEq,
    {
        self.value == *expected
    }

    /// Replaces the `owned` self with a `new_self`.
    #[inline]
    pub fn replace_owned(self, new_self: SELF) -> Self {
        Self::new(new_self, self.value)
    }
    /// Replaces the `value` with a `new_value`.
    #[inline]
    pub fn replace_value(self, new_value: T) -> Self {
        Self::new(self.owned, new_value)
    }
    /// Replaces the `owned` self with a `new_self` and the `value` with a `new_value`.
    #[inline]
    pub fn replace_both(self, new_self: SELF, new_value: T) -> Self {
        Self::new(new_self, new_value)
    }

    /// Applies a mapping function `f` to the `owned` field.
    #[inline]
    pub fn map_owned<F, NewSelf>(self, f: F) -> Owning<NewSelf, T>
    where
        F: FnOnce(SELF) -> NewSelf,
    {
        Owning {
            owned: f(self.owned),
            value: self.value,
        }
    }
    /// Applies a mapping function `f` to the `value` field.
    #[inline]
    pub fn map_value<F, NewT>(self, f: F) -> Owning<SELF, NewT>
    where
        F: FnOnce(T) -> NewT,
    {
        Owning {
            owned: self.owned,
            value: f(self.value),
        }
    }
    /// Applies the mapping functions `f_*` to the respective `owned` and `value` fields.
    pub fn map_both<F, G, NewSelf, NewT>(self, f_owned: F, f_value: G) -> Owning<NewSelf, NewT>
    where
        F: FnOnce(SELF) -> NewSelf,
        G: FnOnce(T) -> NewT,
    {
        Owning {
            owned: f_owned(self.owned),
            value: f_value(self.value),
        }
    }
}

/// # Additional *const* methods for when everything is `Copy`.
impl<SELF: Copy, T: Copy> Owning<SELF, T> {
    /// Destructures itself into a tuple.
    #[inline]
    #[must_use]
    pub const fn into_tuple_const(self) -> (SELF, T) {
        (self.owned, self.value)
    }
    /// Destructures itself returning its `owned` field
    #[inline]
    #[must_use]
    pub const fn into_owned_const(self) -> SELF {
        self.owned
    }
    /// Destructures itself returning its `value` field
    #[inline]
    #[must_use]
    pub const fn into_value_const(self) -> T {
        self.value
    }

    /// Replaces the `owned` self with a `new_self`.
    #[inline]
    pub const fn replace_owned_const(self, new_self: SELF) -> Self {
        Self::new(new_self, self.value)
    }
    /// Replaces the `value` with a `new_value`.
    #[inline]
    pub const fn replace_value_const(self, new_value: T) -> Self {
        Self::new(self.owned, new_value)
    }
    /// Replaces the `owned` self with a `new_self` and the `value` with a `new_value`.
    #[inline]
    pub const fn replace_both_const(self, new_self: SELF, new_value: T) -> Self {
        Self::new(new_self, new_value)
    }
}

mod core_impls {
    use super::Owning;
    use core::fmt;

    impl<SELF: Clone, T: Clone> Clone for Owning<SELF, T> {
        fn clone(&self) -> Self {
            Self {
                owned: self.owned.clone(),
                value: self.value.clone(),
            }
        }
    }
    impl<SELF: Copy, T: Copy> Copy for Owning<SELF, T> {}

    impl<SELF: fmt::Debug, T: fmt::Debug> fmt::Debug for Owning<SELF, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut debug = f.debug_struct("Owning");
            debug
                .field("owned", &self.owned)
                .field("value", &self.value)
                .finish()
        }
    }
    impl<SELF: PartialEq, T: PartialEq> PartialEq for Owning<SELF, T> {
        fn eq(&self, other: &Self) -> bool {
            self.owned == other.owned && self.value == other.value
        }
    }
    impl<SELF: Eq, T: Eq> Eq for Owning<SELF, T> {}

    impl<SELF: Default, T: Default> Default for Owning<SELF, T> {
        fn default() -> Self {
            Self {
                owned: SELF::default(),
                value: T::default(),
            }
        }
    }
}
