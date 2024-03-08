// devela::time::split
//
//! Splitting and decomposing time.
//

use crate::code::paste;

/// A time split in milliseconds, seconds, minutes and hours.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HourMilliSplit<H, M, S, MS> {
    /// Hours.
    pub h: H,
    /// Minutes.
    pub m: M,
    /// Seconds.
    pub s: S,
    /// Milliseconds.
    pub ms: MS,
}

/// A time split in nanoseconds, microseconds, milliseconds and seconds.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecNanoSplit<S, MS, US, NS> {
    /// Seconds.
    pub s: S,
    /// Milliseconds.
    pub ms: MS,
    /// Microseconds.
    pub us: US,
    /// Nanoseconds.
    pub ns: NS,
}

/// A time split in seconds, minutes, hours, days, months and years.
#[repr(Rust)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YearSecSplit<Y, MO, D, H, M, S> {
    /// Years.
    pub y: Y,
    /// Months.
    pub mo: MO,
    /// Days.
    pub d: D,
    /// Hours.
    pub h: H,
    /// Minutes.
    pub m: M,
    /// Seconds.
    pub s: S,
}

// # Arguments
// - $name: the type name
// - $LEN:  the number of generics
// - $T:    the generic T (repeated type)
// - $G:    the generic type (different), and the field name (in lower case)
macro_rules! impl_as_to {
    ($name:ident: $LEN:literal, <$($T:ident+$G:ident),+>) => { paste ! {
        impl<$($G),+> $name<$($G),+> {
            /// Returns the fields as a tuple.
            #[inline] #[must_use]
            pub fn as_tuple(self) -> ( $($G),+ ) {
                ( $( self.[<$G:lower>] ),+ )
            }
        }
        impl<$($G: Copy),+> $name<$($G),+> {
            /// Returns the fields as a tuple in compile-time.
            #[inline] #[must_use]
            pub const fn to_tuple(self) -> ( $($G),+ ) {
                ( $( self.[<$G:lower>] ),+ )
            }
        }
        impl<T> $name<$($T),+> {
            /// Returns the fields as an array.
            #[inline] #[must_use]
            pub fn as_array(self) -> [T; $LEN] {
                [ $( self.[<$G:lower>] ),+ ]
            }
        }
        impl<T: Copy> $name<$($T),+> {
            /// Returns the fields as an array in compile-time.
            #[inline] #[must_use]
            pub const fn to_array(self) -> [T; $LEN] {
                [ $( self.[<$G:lower>] ),+ ]
            }
        }

    }};
}
impl_as_to![SecNanoSplit: 4, <T+S, T+MS, T+US, T+NS>];
impl_as_to![HourMilliSplit: 4, <T+H, T+M, T+S, T+MS>];
impl_as_to![YearSecSplit: 6, <T+Y, T+MO, T+D, T+H, T+M, T+S>];
