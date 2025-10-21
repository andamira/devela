// devela_base_core::num::niche::absence
//
//! Defines [`NonNiche`]`<T>`.
//
/// A zero-cost wrapper that behaves like `NonZero<T>` or `NonValue<T>` but stores `T` directly.
///
/// This type is useful when you want to offer API consistency between
/// niche-optimized and non-optimized versions of a type, allowing users
/// to choose based on their memory layout needs.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNiche<T>(T);

#[rustfmt::skip]
impl<T> NonNiche<T> {
    /// Creates a new `NonNiche` if the value is provided.
    ///
    /// This always succeeds, unlike `NonZero*` types.
    #[must_use] #[inline(always)]
    pub const fn new(value: T) -> Option<Self> { Some(Self(value)) }

    /// Creates a new `NonNiche` without checking.
    /// # Safety
    /// This is always safe since `NonNiche` doesn't have any validity constraints.
    #[must_use] #[inline(always)]
    pub const unsafe fn new_unchecked(value: T) -> Self { Self(value) }

    /// Extracts the inner value.
    #[must_use] #[inline(always)]
    pub const fn get(&self) -> T where T: Copy { self.0 }
}

impl<T> From<T> for NonNiche<T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Self(value)
    }
}

// helper make implementations over primitives.
macro_rules! impl_non_niche {
    () => { impl_non_niche!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize); };
    ($($prim:ty),+) => {
        $(
            impl NonNiche<$prim> {
                /// The minimum possible value.
                pub const MIN: Self = Self(<$prim>::MIN);
                /// The maximum possible value.
                pub const MAX: Self = Self(<$prim>::MAX);
            }
        )+
    };
}
impl_non_niche![];
