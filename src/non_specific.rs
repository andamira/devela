// devela::non_specific
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//

use crate::paste;
use core::{fmt, num::*};

macro_rules! impl_non_specific {
    ($name:ident) => {
        impl_non_specific![NonSpecific, i, I, 8, 16, 32, 64, 128];
        impl_non_specific![NonSpecific, u, U, 8, 16, 32, 64, 128];
    };
    ($name:ident, $s:ident, $S:ident, $( $b:literal ),+) => {
        $( impl_non_specific![@NonSpecific, $s, $S, $b]; )+
    };

    // $name: the base name of the new type. E.g. NonSpecific.
    // $s: the sign identifier, lowercase: i or u.
    // $S: the sign identifier, uppercase: I or U.
    // $b: the bits of the type, from 8 to 128.
    (@$name:ident, $s:ident, $S:ident, $b:literal) => { paste! {
        /// An integer that is known not to equal the specific value `V`.
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::" [<$name $S $b>] ";"]
        ///
        #[doc = "assert![" [<$name $S $b>] "::<13>::new(13).is_none()];"]
        #[doc = "assert![" [<$name $S $b>] "::<13>::new(12).unwrap().get() == 12];"]
        /// ```
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name $S $b>]<const V: [<$s $b>]>([<NonZero $S $b>]);

        impl<const V: [<$s $b>]> fmt::Display for [<$name $S $b>]<V> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.get())
            }
        }
        impl<const V: [<$s $b>]> fmt::Debug for [<$name $S $b>]<V> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}({})", stringify!([<$name $S $b>]), self.get())
            }
        }

        impl<const V: [<$s $b>]> [<$name $S $b>]<V> {
            #[doc = "Creates a `" [<$name $S $b>] "` if the given value is not `V`."]
            pub const fn new(value: [<$s $b>]) -> Option<Self> {
                // [<NonZero $S $b>]::new(value ^ V).map(Self) // non-const
                match [<NonZero $S $b>]::new(value ^ V) {
                    None => None,
                    Some(v) => Some(Self(v)),
                }
            }

            #[doc = "Creates a `" [<$name $S $b>] "` if the given value is not `V`."]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            pub const unsafe fn new_unchecked(value: [<$s $b>]) -> Self {
                Self([<NonZero $S $b>]::new_unchecked(value ^ V))
            }

            /// Returns the value as a primitive type.
            pub const fn get(&self) -> [<$s $b>] {
                self.0.get() ^ V
            }
        }
    }};
}
impl_non_specific![NonSpecific];
