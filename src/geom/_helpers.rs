// devela::geom::_helpers
//
//! Defines helpers for implementing common methods on geometric types.
//
// TOC
// - macro _impl_geom_dim!
// - macro _geom_dim_cast_ctor!
// - macro _define_geom_dim_macro!
// - macro _geom_region_cast_ctor!

#[cfg(doc)]
use crate::{Distance, Extent, Orientation, Position, Stride};

/// Helps implementing common methods for geometric types of the form:
/// `Name<T, const D: usize> { dim: [T; D] }`.
///
/// It is used for [`Distance`], [`Extent`], [`Orientation`], [`Position`] and [`Stride`].
macro_rules! _impl_geom_dim {
    ( // implement common utility traits:
      // - conversion From arrays and tuples
      // - ConstInit, Default
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

        /* Default, ConstInit */

        impl<T: Default, const D: usize> Default for $Name<T, D> {
            fn default() -> Self {
                Self::new($crate::init_array![default [T; D], "safe_geom", "unsafe_array"])
            }
        }
        impl<T: $crate::ConstInit, const D: usize> $crate::ConstInit for $Name<T, D> {
            const INIT: Self = Self::new($crate::init_array![INIT in $crate::ConstInit [T; D]]);
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
                use $crate::ArrayExt;
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
            pub const fn as_slice_mut(&mut self) -> &mut [T] {
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

            #[doc = "Returns a new " $Name " by applying `f` to each dimension."]
            ///
            /// This is a runtime, dimension-preserving transformation.
            ///
            /// It is useful for reshaping the inner scalar type without introducing
            /// blanket `From`/`TryFrom` impl conflicts on the wrapper itself.
            #[must_use]
            pub fn map<U>(self, f: impl FnMut(T) -> U) -> $Name<U, D> {
                $Name::new(self.dim.map(f))
            }

            #[doc = "Returns a new " $Name " by fallibly applying `f` to each dimension."]
            ///
            /// Stops at the first conversion error and returns it.
            ///
            /// This is the fallible counterpart to [`map`](Self::map), and is the
            /// recommended runtime path for per-dimension checked conversion.
            pub fn try_map<U, E>(self, mut f: impl FnMut(T) -> Result<U, E>)
                -> Result<$Name<U, D>, E> {
                let mut dim: [Option<U>; D] = core::array::from_fn(|_| None);
                for (i, value) in self.dim.into_iter().enumerate() {
                    dim[i] = Some(f(value)?);
                }
                Ok($Name::new(dim.map(|value| match value {
                    Some(value) => value,
                    None => unreachable!(),
                })))
            }

            #[doc = "Converts this " $Name:lower " to another inner type `U` when `U` implements `From<T>`."]
            ///
            /// This is a convenience wrapper over [`map`](Self::map).
            #[must_use]
            pub fn map_into<U>(self) -> $Name<U, D> where U: From<T> {
                self.map(U::from)
            }

            #[doc = "Tries to convert this " $Name:lower
            " to another inner type `U` when `U` implements `TryFrom<T>`."]
            ///
            /// This is a convenience wrapper over [`try_map`](Self::try_map).
            pub fn try_map_into<E, U>(self) -> Result<$Name<U, D>, E> where U: TryFrom<T, Error = E> {
                self.try_map(U::try_from)
            }
        }

        $crate::_impl_geom_dim![common_methods_2d: $Name];
        $crate::_impl_geom_dim![common_methods_3d: $Name];
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

            /// Returns `true` if the 3 dimensions are equal.
            #[must_use]
            pub fn is_uniform_3d(&self) -> bool where T: PartialEq {
                self.dim[0] == self.dim[1] && self.dim[0] == self.dim[2]
            }
        }
    };
}
#[doc(hidden)]
pub(crate) use _impl_geom_dim;

#[macro_export]
#[doc(hidden)]
macro_rules! _geom_dim_cast_ctor {
    (@scalar checked    $x:expr => $P:ty) => { $crate::cast!(checked    $x => $P) };
    (@scalar saturating $x:expr => $P:ty) => { $crate::cast!(saturating $x => $P) };
    (@scalar wrapping   $x:expr => $P:ty) => { $crate::cast!(wrapping   $x => $P) };
    (@plain $Wrap:ident; $op:ident => $P:ty; $($arg:expr),+ $(,)?) => {
        $crate::$Wrap::new([
            $($crate::_geom_dim_cast_ctor!(@scalar $op $arg => $P)),+
        ])
    };

    ($Wrap:ident; saturating => $P:ty; $($arg:expr),+ $(,)?) => {
        $crate::_geom_dim_cast_ctor!(@plain $Wrap; saturating => $P; $($arg),+)
    };
    ($Wrap:ident; wrapping => $P:ty; $($arg:expr),+ $(,)?) => {
        $crate::_geom_dim_cast_ctor!(@plain $Wrap; wrapping => $P; $($arg),+)
    };

    ($Wrap:ident; saturating $from:expr => $P:ty) => {
        $from.map(|x| $crate::cast!(saturating x => $P))
    };
    ($Wrap:ident; wrapping $from:expr => $P:ty) => {
        $from.map(|x| $crate::cast!(wrapping x => $P))
    };

    // keep checked separate
    ($Wrap:ident; checked => $P:ty; $x:expr) => {
        match $crate::cast!(checked $x => $P) {
            Ok(x) => Ok($crate::$Wrap::new([x])),
            Err(e) => Err(e),
        }
    };
    ($Wrap:ident; checked => $P:ty; $x:expr, $y:expr) => {
        match (
            $crate::cast!(checked $x => $P),
            $crate::cast!(checked $y => $P),
        ) {
            (Ok(x), Ok(y)) => Ok($crate::$Wrap::new([x, y])),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    };
    ($Wrap:ident; checked => $P:ty; $x:expr, $y:expr, $z:expr) => {
        match (
            $crate::cast!(checked $x => $P),
            $crate::cast!(checked $y => $P),
            $crate::cast!(checked $z => $P),
        ) {
            (Ok(x), Ok(y), Ok(z)) => Ok($crate::$Wrap::new([x, y, z])),
            (Err(e), _, _) => Err(e),
            (_, Err(e), _) => Err(e),
            (_, _, Err(e)) => Err(e),
        }
    };
    ($Wrap:ident; checked => $P:ty; $x:expr, $y:expr, $z:expr, $w:expr) => {
        match (
            $crate::cast!(checked $x => $P),
            $crate::cast!(checked $y => $P),
            $crate::cast!(checked $z => $P),
            $crate::cast!(checked $w => $P),
        ) {
            (Ok(x), Ok(y), Ok(z), Ok(w)) => Ok($crate::$Wrap::new([x, y, z, w])),
            (Err(e), _, _, _) => Err(e),
            (_, Err(e), _, _) => Err(e),
            (_, _, Err(e), _) => Err(e),
            (_, _, _, Err(e)) => Err(e),
        }
    };

    ($Wrap:ident; checked? => $P:ty; $($arg:expr),+ $(,)?) => {
        $crate::unwrap![ok? $crate::_geom_dim_cast_ctor!($Wrap; checked => $P; $($arg),+)]
    };
    ($Wrap:ident; checked_unwrap => $P:ty; $($arg:expr),+ $(,)?) => {
        $crate::unwrap![ok $crate::_geom_dim_cast_ctor!($Wrap; checked => $P; $($arg),+)]
    };
    ($Wrap:ident; checked_expect => $P:ty; $($arg:expr),+, $msg:expr) => {
        $crate::unwrap![ok_expect
            $crate::_geom_dim_cast_ctor!($Wrap; checked => $P; $($arg),+),
            $msg
        ]
    };

    ($Wrap:ident; checked $from:expr => $P:ty) => {
        $from.try_map(|x| $crate::cast!(checked x => $P))
    };
    ($Wrap:ident; checked? $from:expr => $P:ty) => {
        $crate::unwrap![ok? $crate::_geom_dim_cast_ctor!($Wrap; checked $from => $P)]
    };
    ($Wrap:ident; checked_unwrap $from:expr => $P:ty) => {
        $crate::unwrap![ok $crate::_geom_dim_cast_ctor!($Wrap; checked $from => $P)]
    };
    ($Wrap:ident; checked_expect $from:expr => $P:ty, $msg:expr) => {
        $crate::unwrap![ok_expect
            $crate::_geom_dim_cast_ctor!($Wrap; checked $from => $P),
            $msg
        ]
    };
}
#[doc(hidden)]
pub use _geom_dim_cast_ctor;

macro_rules! _define_geom_dim_macro {
    // # Args
    // $_d: the dollar sign passed as a token, as a trick to be able to nest repetitions.
    // $name: the name of the macro. E.g. ext.
    // $det: the determinant used to introduce a singular $Wrap. Either "a" or "an".
    // $Wrap: the name of the wrapper type. E.g. Extent.
    // $tag1: the first doc tag for _tag!. E.g. geom.
    // $location: the location for _doc_location!. E.g. "geom/metric"
    (($_d:tt) $name:ident, $det:literal, $Wrap:ident, $tag1:ident, $location:literal
    ) => { $crate::paste! {
        #[doc = crate::_tags!($tag1 construction)]
        #[doc = "Constructs " $det " [`" $Wrap "`] with inferred dimensionality."]
        #[doc = crate::_doc_location!($location)]
        ///
        /// Supports:
        /// - positional construction for 1 to 4 dimensions,
        /// - uniform repeated construction for any const dimension,
        /// - cast-construction for primitive scalars.
        ///
        /// Notes:
        /// - Explicit cast-construction supports 1 to 4 dimensions and is const-friendly.
        /// - Cast forms delegate to [`Cast`][crate::Cast] and [`cast!`][crate::cast].
        /// - Whole-value cast shorthand supports any dimension and is runtime-only.
        ///
        /// # Example
        /// ```
        #[doc = "# use devela::{" $Wrap ", " $Wrap "2, " $Wrap "3, " $name "};"]
        #[doc = "// construct"]
        #[doc = "let a = " $name "!(4, 7);"]
        #[doc = "assert_eq![a.x(), 4_i32];"]
        ///
        #[doc = "const B: " $Wrap "3<i32> = " $name "!(1000, 3, 21);"]
        #[doc = "const C: " $Wrap "<u64, 5> = " $name "!([u32::MAX as u64 + 2; 5]);"]
        ///
        #[doc = "// checked"]
        #[doc = "let a2 = " $name "!(checked => i16; a.x(), a.y());"]
        #[doc = "let a3 = " $name "!(checked a => i16); // runtime shorthand over the whole value"]
        #[doc = "assert_eq![a2, Ok(" $Wrap "2::<i16>::new([4, 7]))];"]
        #[doc = "assert_eq![a3, Ok(" $Wrap "2::<i16>::new([4, 7]))];"]
        ///
        #[doc = "// saturating"]
        #[doc = "const B2: " $Wrap "3<u8> = " $name "!(saturating => u8; B.x(), B.y(), B.z());"]
        #[doc = "assert_eq![B2, " $Wrap "3::<u8>::new([255, 3, 21])];"]
        ///
        #[doc = "// wrapping"]
        #[doc = "const B3: " $Wrap "3<u8> = " $name "!(wrapping => u8; B.x(), B.y(), B.z());"]
        #[doc = "assert_eq![B3, " $Wrap "3::<u8>::new([232, 3, 21])];"]
        ///
        #[doc = "let c2 = " $name "!(wrapping C => u32);"]
        #[doc = "assert_eq![c2, " $Wrap "::<u32, 5>::new([1, 1, 1, 1, 1])];"]
        /// ```
        #[macro_export]
        #[doc(hidden)]
        macro_rules! [<_ $name>] {
            (
            // uniform repeated construction for any const dimension
             [$v:expr; $n:expr]) => { $crate::$Wrap::new([$v; $n]) };

            (
            // positional construction for 1 to 4 dimensions
             $x:expr $_d(,)?) => { $crate::$Wrap::new([$x]) };
            ($x:expr, $y:expr $_d(,)?) => { $crate::$Wrap::new([$x, $y]) };
            ($x:expr, $y:expr, $z:expr $_d(,)?) => { $crate::$Wrap::new([$x, $y, $z]) };
            ($x:expr, $y:expr, $z:expr, $w:expr $_d(,)?) => { $crate::$Wrap::<_, 4>::new([$x, $y, $z, $w]) };

            (
             // explicit component cast-construction; const-friendly
             checked => $P:ty; $_d($arg:expr),+ $_d(,)?) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked => $P; $_d($arg),+)
            };
            (checked? => $P:ty; $_d($arg:expr),+ $_d(,)?) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked? => $P; $_d($arg),+)
            };
            (checked_unwrap => $P:ty; $_d($arg:expr),+ $_d(,)?) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked_unwrap => $P; $_d($arg),+)
            };
            (checked_expect => $P:ty; $_d($arg:expr),+, $msg:expr) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked_expect => $P; $_d($arg),+, $msg)
            };
            (saturating => $P:ty; $_d($arg:expr),+ $_d(,)?) => {
                $crate::_geom_dim_cast_ctor!($Wrap; saturating => $P; $_d($arg),+)
            };
            (wrapping => $P:ty; $_d($arg:expr),+ $_d(,)?) => {
                $crate::_geom_dim_cast_ctor!($Wrap; wrapping => $P; $_d($arg),+)
            };

            (
             // whole-value cast shorthand; runtime-only
             checked $from:expr => $P:ty) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked $from => $P)
            };
            (checked? $from:expr => $P:ty) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked? $from => $P)
            };
            (checked_unwrap $from:expr => $P:ty) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked_unwrap $from => $P)
            };
            (checked_expect $from:expr => $P:ty, $msg:expr) => {
                $crate::_geom_dim_cast_ctor!($Wrap; checked_expect $from => $P, $msg)
            };
            (saturating $from:expr => $P:ty) => {
                $crate::_geom_dim_cast_ctor!($Wrap; saturating $from => $P)
            };
            (wrapping $from:expr => $P:ty) => {
                $crate::_geom_dim_cast_ctor!($Wrap; wrapping $from => $P)
            };
        }
        #[doc(inline)]
        pub use [<_ $name>] as $name;
    }};
}
pub(crate) use _define_geom_dim_macro;

#[macro_export]
#[doc(hidden)]
macro_rules! _geom_region_cast_ctor {
    (
    // explicit component cast-construction; const-friendly
     checked => $P:ty, $E:ty;
     $($pos:expr),+ $(,)?;
     $($ext:expr),+ $(,)?
    ) => {
        match (
            $crate::pos!(checked => $P; $($pos),+),
            $crate::ext!(checked => $E; $($ext),+),
        ) {
            (Ok(pos), Ok(ext)) => Ok($crate::Region::new(pos, ext)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    };
    (checked? => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::unwrap![ok?
            $crate::_geom_region_cast_ctor!(checked => $P, $E; $($pos),+; $($ext),+)
        ]
    };
    (checked_unwrap => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::unwrap![ok
            $crate::_geom_region_cast_ctor!(checked => $P, $E; $($pos),+; $($ext),+)
        ]
    };
    (checked_expect => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+, $msg:expr) => {
        $crate::unwrap![ok_expect
            $crate::_geom_region_cast_ctor!(checked => $P, $E; $($pos),+; $($ext),+),
            $msg
        ]
    };

    (
    // explicit component cast-construction; const-friendly
     saturating => $P:ty, $E:ty;
     $($pos:expr),+ $(,)?;
     $($ext:expr),+ $(,)?
    ) => {
        $crate::Region::new(
            $crate::pos!(saturating => $P; $($pos),+),
            $crate::ext!(saturating => $E; $($ext),+),
        )
    };
    (wrapping => $P:ty, $E:ty; $($pos:expr),+ $(,)?; $($ext:expr),+ $(,)?) => {
        $crate::Region::new(
            $crate::pos!(wrapping => $P; $($pos),+),
            $crate::ext!(wrapping => $E; $($ext),+),
        )
    };

    (
    // whole-region cast shorthand; runtime-only
     checked $from:expr => $P:ty, $E:ty
    ) => {{
        let from = $from;
        match (
            $crate::pos!(checked from.pos => $P),
            $crate::ext!(checked from.ext => $E),
        ) {
            (Ok(pos), Ok(ext)) => Ok($crate::Region::new(pos, ext)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }};
    (checked? $from:expr => $P:ty, $E:ty) => {
        $crate::unwrap![ok? $crate::_geom_region_cast_ctor!(checked $from => $P, $E)]
    };
    (checked_unwrap $from:expr => $P:ty, $E:ty) => {
        $crate::unwrap![ok $crate::_geom_region_cast_ctor!(checked $from => $P, $E)]
    };
    (checked_expect $from:expr => $P:ty, $E:ty, $msg:expr) => {
        $crate::unwrap![ok_expect
            $crate::_geom_region_cast_ctor!(checked $from => $P, $E), $msg
        ]
    };

    (saturating $from:expr => $P:ty, $E:ty) => {{
        let from = $from;
        $crate::Region::new(
            $crate::pos!(saturating from.pos => $P),
            $crate::ext!(saturating from.ext => $E),
        )
    }};
    (wrapping $from:expr => $P:ty, $E:ty) => {{
        let from = $from;
        $crate::Region::new(
            $crate::pos!(wrapping from.pos => $P),
            $crate::ext!(wrapping from.ext => $E),
        )
    }};
}
