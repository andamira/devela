// devela_base_alloc::num::int::wrapper::namespace
//
//! Defines the [`IntAlloc`] namespace wrapper.
//

pub use crate::Int;

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// This is a wrapper over [`Int`] with additional allocating methods.
#[must_use]
#[repr(transparent)]
pub struct IntAlloc<T>(pub Int<T>);

impl<T> IntAlloc<T> {
    /// Creates a new `IntAlloc` instance wrapping the given value.
    ///
    /// The returned instance provides access to both allocating integer methods
    /// and the underlying non-allocating [`Int`] methods through `Deref`.
    pub fn new(inner: T) -> Self {
        Self(Int(inner))
    }
}

crate::impl_ops![IntAlloc: i8, i16, i32, i64, i128, isize];
crate::impl_ops![IntAlloc: (no_neg) u8, u16, u32, u64, u128, usize];

#[rustfmt::skip]
mod core_impls {
    use crate::{impl_trait, Int, IntAlloc, Ordering, Deref, DerefMut};

    impl<T> Deref for IntAlloc<T> {
        type Target = Int<T>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for IntAlloc<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    /* following the same traits implemented for Int */

    impl<T: Clone> Clone for IntAlloc<T> {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for IntAlloc<T> {}

    impl_trait![fmt::Debug for IntAlloc[T][T] where T |self, f|
        f.debug_tuple("IntAlloc").field(&self.0).finish()
    ];
    impl_trait![fmt::Display for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Binary for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Octal for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerHex for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperHex for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerExp for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperExp for IntAlloc[T][T] where T |self, f| self.0.fmt(f)];

    /* eq */

    impl_trait![PartialEq for IntAlloc[T][T] where T |self, o| self.0.eq(&o.0)];
    impl<T: Eq> Eq for IntAlloc<T> {}
    // with the inner value:
    impl<T: PartialEq> PartialEq<T> for IntAlloc<T> {
        fn eq(&self, other: &T) -> bool { self.0.eq(other) }
    }

    /* ord*/

    impl<T: PartialOrd> PartialOrd for IntAlloc<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for IntAlloc<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }
    // with the inner value:
    impl<T: PartialOrd> PartialOrd<T> for IntAlloc<T> {
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.0.partial_cmp(other)
        }
    }

    impl_trait![Hash for IntAlloc[T][T] where T |self, s| self.0.hash(s)];
}
