// devela::num::geom::metric::helpers
//
//! Defines helpers for implementing common methods on metric types.
//

#[cfg(doc)]
use crate::{Distance, Extent, Position};
#[cfg(all(doc, feature = "metric"))]
use crate::{Orientation, Stride};

/// Helps implementing common methods on metric types of the shape of `Name<T, D> { dim: [T; D] }`.
///
/// It is used for [`Distance`], [`Extent`], [`Orientation`], [`Position`] and [`Stride`].
macro_rules! _impl_metric {
    ( // implement common utility traits:
      // - conversion From arrays and tuples
      // - Default, ConstDefault
      // - Clone, Copy, Hash
      // - Debug, Display
      // - PartialEq, Eq
      // - PartialOrd, Ord
    common_traits: $Name:ident) => {
        /* conversion From arrays and tuples */

        impl<T, const D: usize> From<[T; D]> for $Name<T, D> {
            fn from(dim: [T; D]) -> Self { Self { dim } }
        }
        impl<T> From<(T, T)> for $Name<T, 2> {
            fn from(dim: (T, T)) -> Self { Self { dim: [dim.0, dim.1] } }
        }
        impl<T> From<(T, T, T)> for $Name<T, 3> {
            fn from(dim: (T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2] } }
        }
        impl<T> From<(T, T, T, T)> for $Name<T, 4> {
            fn from(dim: (T, T, T, T)) -> Self { Self { dim: [dim.0, dim.1, dim.2, dim.3] } }
        }

        /* Default, ConstDefault */

        impl<T: Default, const D: usize> Default for $Name<T, D> {
            fn default() -> Self {
                Self::new($crate::array_init![default [T; D], "safe_num", "unsafe_array"])
            }
        }
        impl<T: $crate::ConstDefault, const D: usize> $crate::ConstDefault for $Name<T, D> {
            const DEFAULT: Self = {
                use crate::ConstDefault;
                Self::new($crate::array_init![const_default [T; D]])
            };
        }

        /* Clone, Copy, Hash */

        impl<T: Clone, const D: usize> Clone for $Name<T, D> {
            fn clone(&self) -> Self { Self::new(self.dim.clone()) }
        }
        impl<T: Copy, const D: usize> Copy for $Name<T, D> {}
        impl<T: $crate::Hash, const D: usize> $crate::Hash for $Name<T, D> {
            fn hash<HR: $crate::Hasher>(&self, state: &mut HR) { self.dim.hash(state); }
        }

        /* Debug, Display */

        impl<T: $crate::Debug, const D: usize> $crate::Debug for $Name<T, D> {
            fn fmt(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                f.debug_struct(stringify!($Name)).field("dim", &self.dim).finish()
            }
        }
        impl<T: $crate::Display, const D: usize> $crate::Display for $Name<T, D> {
            fn fmt(&self, f: &mut $crate::Formatter<'_>) -> $crate::FmtResult<()> {
                use $crate::ExtArray;
                write!(f, "{}", self.dim.fmt())
            }
        }

        /* PartialEq, Eq, PartialOrd, Ord */

        impl<T: PartialEq, const D: usize> PartialEq for $Name<T, D> {
            fn eq(&self, other: &Self) -> bool { self.dim == other.dim }
        }
        impl<T: Eq, const D: usize> Eq for $Name<T, D> {}

        impl<T: PartialOrd, const D: usize> PartialOrd for $Name<T, D> {
            fn partial_cmp(&self, other: &Self) -> Option<$crate::Ordering> {
                self.dim.partial_cmp(&other.dim)
            }
        }
        impl<T: Ord, const D: usize> Ord for $Name<T, D> {
            fn cmp(&self, other: &Self) -> $crate::Ordering { self.dim.cmp(&other.dim) }
        }
    };
    ( // implement common methods
      // NOTE: also calls common_methods_[2d|3d]
    common_methods: $Name:ident) => { $crate::paste! {
        impl<T, const D: usize> $Name<T, D> {
            #[doc = "Constructs a new " $Name " from the given dimensions."]
            pub const fn new(dimensions: [T; D]) -> Self {
                Self { dim: dimensions }
            }

            #[doc = "Returns a shared reference to the " $Name:lower " as a slice."]
            #[must_use]
            pub const fn as_slice(&self) -> &[T] {
                &self.dim
            }
            #[doc = "Returns an exclusive reference to the " $Name:lower " as a slice."]
            #[must_use]
            pub fn as_slice_mut(&mut self) -> &mut [T] {
                &mut self.dim
            }

            /// Returns `true` if all dimensions of the extent are equal.
            #[doc = "Returns `true` if all dimensions of the " $Name:lower " are equal."]
            #[must_use]
            pub fn is_uniform_nd(&self) -> bool where T: PartialEq {
                if D == 0 { return true }
                let mut i = 1;
                while i < D {
                    if self.dim[i] != self.dim[0] { return false }
                    i += 1;
                }
                true
            }
        }

        $crate::_impl_metric![common_methods_2d: $Name];
        $crate::_impl_metric![common_methods_3d: $Name];
    }};

    /* manual impls for specific dimensionalities */

    ( // implement common methods for 2 dimensions
    common_methods_2d: $Name:ident) => {
        impl<T> $Name<T, 2> {
            /// Returns a copy of the first dimension `x`.
            #[must_use]
            pub const fn x(self) -> T where T: Copy { self.dim[0] }
            /// Returns a copy of the second dimension `y`.
            #[must_use]
            pub const fn y(self) -> T where T: Copy { self.dim[1] }

            /// Returns a shared reference to the first dimension `x`.
            #[must_use]
            pub const fn x_ref(&self) -> &T { &self.dim[0] }
            /// Returns a shared reference to the second dimension `y`.
            #[must_use]
            pub const fn y_ref(&self) -> &T { &self.dim[1] }

            /// Returns an exclusive reference to the first dimension `x`.
            #[must_use]
            pub fn x_mut(&mut self) -> &mut T { &mut self.dim[0] }
            /// Returns an exclusive reference to the second dimension `y`.
            #[must_use]
            pub fn y_mut(&mut self) -> &mut T { &mut self.dim[1] }

            /// Returns `true` if the 2 dimensions are equal.
            #[must_use]
            pub fn is_uniform(&self) -> bool where T: PartialEq {
                self.dim[0] == self.dim[1]
            }
        }
    };
    ( // implement common methods for 2 dimensions
    common_methods_3d: $Name:ident) => {
        impl<T> $Name<T, 3> {
            /// Returns a copy of the first dimension `x`.
            #[must_use]
            pub const fn x(self) -> T where T: Copy { self.dim[0] }
            /// Returns a copy of the second dimension `y`.
            #[must_use]
            pub const fn y(self) -> T where T: Copy { self.dim[1] }
            /// Returns a copy of the third dimension `z`.
            #[must_use]
            pub const fn z(self) -> T where T: Copy { self.dim[2] }

            /// Returns a shared reference to the first dimension `x`.
            #[must_use]
            pub const fn x_ref(&self) -> &T { &self.dim[0] }
            /// Returns a shared reference to the second dimension `y`.
            #[must_use]
            pub const fn y_ref(&self) -> &T { &self.dim[1] }
            /// Returns a shared reference to the third dimension `z`.
            #[must_use]
            pub const fn z_ref(&self) -> &T { &self.dim[2] }

            /// Returns an exclusive reference to the first dimension `x`.
            #[must_use]
            pub fn x_mut(&mut self) -> &mut T { &mut self.dim[0] }
            /// Returns an exclusive reference to the second dimension `y`.
            #[must_use]
            pub fn y_mut(&mut self) -> &mut T { &mut self.dim[1] }
            /// Returns an exclusive reference to the third dimension `z`.
            #[must_use]
            pub fn z_mut(&mut self) -> &mut T { &mut self.dim[2] }

            /// Returns `true` if the 3 dimensions are equal.
            #[must_use]
            pub fn is_uniform_3d(&self) -> bool where T: PartialEq {
                self.dim[0] == self.dim[1] && self.dim[0] == self.dim[2]
            }
        }
    };
}
pub(crate) use _impl_metric;
