// devela::data::conversion::primitive::cast
//
//! fns to cast between primitives in a checked manner.
//

use crate::{
    data::{DataErrors as E, DataResult as Result},
    meta::{iif, paste},
};

// Implements casting functions between primitives
//
// `$f`: the type to cast from
// `$t`: the type to cast to
macro_rules! impl_casts {
    () => {
        /* independent of pointer width */

        impl_casts![can_overunderflow
            // from bigger signed to unsigned
            i16:u8, i32:u8, i64:u8, i128:u8,
            i32:u16, i64:u16, i128:u16,
            i64:u32, i128:u32,
            i128:u64,
            // from bigger signed to signed
            i16:i8, i32:i8, i64:i8, i128:i8,
            i32:i16, i64:i16, i128:i16,
            i64:i32, i128:i32,
            i128:i64
        ];
        impl_casts![can_overflow
            // from bigger unsigned to unsigned
            u16:u8, u32:u8, u64:u8, u128:u8,
            u32:u16, u64:u16, u128:u16,
            u64:u32, u128:u32,
            u128:u64,
            // from bigger unsigned to signed
            u16:i8, u32:i8, u64:i8, u128:i8,
            u32:i16, u64:i16, u128:i16,
            u64:i32, u128:i32,
            u128:i64,
            // from equalsized unsigned to signed
            u8:i8, u16:i16, u32:i32, u64:i64, u128:i128, usize:isize
        ];
        impl_casts![can_underflow
            // from smaller signed to unsigned
            i8:u16, i8:u32, i8:u64, i8:u128,
            i16:u32, i16:u64, i16:u128,
            i32:u64, i32:u128,
            i64:u128,
            // from equalsized signed to unsigned
            i8:u8, i16:u16, i32:u32, i64:u64, i128:u128, isize:usize
        ];
        impl_casts![cant_fail
            // from smaller unsigned to unsigned
            u8:u16, u8:u32, u8:u64, u8:u128,
            u16:u32, u16:u64, u16:u128,
            u32:u64, u32:u128,
            u64:u128,
            // from smaller unsigned to signed
            u8:i16, u8:i32, u8:i64, u8:i128,
            u16:i32, u16:i64, u16:i128,
            u32:i64, u32:i128,
            u64:i128,
            // from smaller signed to signed
            i8:i16, i8:i32, i8:i64, i8:i128,
            i16:i32, i16:i64, i16:i128,
            i32:i64, i32:i128,
            i64:i128,
            // from equalsized unsigned to unsigned
            u8:u8, u16:u16, u32:u32, u64:u64, u128:u128, usize:usize,
            // from equalsized signed to signed
            i8:i8, i16:i16, i32:i32, i64:i64, i128:i128, isize:isize
        ];

        /* dependent on pointer width */

        #[cfg(target_pointer_width = "32")]
        impl_casts![can_overunderflow
            // from bigger signed to unsigned
            isize:u8, isize:u16,
            i64:usize, i128:usize,
            // from bigger signed to signed
            isize:i8, isize:i16,
            i64:isize, i128:isize
        ];
        #[cfg(target_pointer_width = "32")]
        impl_casts![can_overflow
            // from bigger unsigned to unsigned
            usize:u8, usize:u16,
            u64:usize, u128:usize,
            // from bigger unsigned to signed
            usize:i8, usize:i16,
            u64:isize, u128:isize,
            // from equalsized unsigned to signed
            u32:isize, usize:i32
        ];
        #[cfg(target_pointer_width = "32")]
        impl_casts![can_underflow
            // from smaller signed to unsigned
            isize:u64, isize:u128,
            i8:usize, i16:usize,
            // from equalsized signed to unsigned
            i32:usize, isize:u32
        ];
        #[cfg(target_pointer_width = "32")]
        impl_casts![cant_fail ptr:32
            // from smaller unsigned to unsigned
            usize:u64, usize:u128,
            u8:usize, u16:usize,
            // from smaller unsigned to signed
            usize:i64, usize:i128,
            u8:isize, u16:isize,
            // from smaller signed to signed
            isize:i64, isize:i128,
            i8:isize, i16:isize,
            // from equalsized unsigned to unsigned
            usize:u32, u32:usize,
            // from equalsized signed to signed
            isize:i32, i32:isize
        ];

        #[cfg(target_pointer_width = "64")]
        impl_casts![can_overunderflow
            // from bigger signed to unsigned
            isize:u8, isize:u16, isize:u32,
            i128:usize,
            // from bigger signed to signed
            isize:i8, isize:i16, isize:i32,
            i128:isize
        ];
        #[cfg(target_pointer_width = "64")]
        impl_casts![can_overflow
            // from bigger unsigned to unsigned
            usize:u8, usize:u16, usize:u32,
            u128:usize,
            // from bigger unsigned to signed
            usize:i8, usize:i16, usize:i32,
            u128:isize,
            // from equalsized unsigned to signed
            u64:isize, usize:i64
        ];
        #[cfg(target_pointer_width = "64")]
        impl_casts![can_underflow
            // from smaller signed to unsigned
            isize:u128,
            i8:usize, i16:usize, i32:usize,
            // from equalsized signed to unsigned
            i64:usize, isize:u64
        ];
        #[cfg(target_pointer_width = "64")]
        impl_casts![cant_fail ptr:64
            // from smaller unsigned to unsigned
            usize:u128,
            u8:usize, u16:usize, u32:usize,
            // from smaller unsigned to signed
            usize:i128,
            u8:isize, u16:isize, u32:isize,
            // from smaller signed to signed
            isize:i128,
            i8:isize, i16:isize, i32:isize,
            // from equalsized unsigned to unsigned
            usize:u64, u64:usize,
            // from equalsized signed to signed
            isize:i64, i64:isize
        ];
    };
    (can_overunderflow $( $f:ty:$t:ty ),+) => { $( impl_casts![@can_overunderflow $f:$t]; )+ };
    (@can_overunderflow $f:ty:$t:ty) => { paste! {
        #[doc = "Safely casts `" $f "` to `" $t "` with range check."]
        ///
        /// # Errors
        #[doc = "Returns an [`Overflow`][E::Overflow] error if `p` > [`" $t "::MAX`]"]
        #[doc = "and an [`Underflow`][E::Overflow] error if `p` < [`" $t "::MIN`]."]
        #[inline]
        pub const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            if p < <$t>::MIN as $f {
                Err(E::Underflow)
            } else if p > $t::MAX as $f {
                Err(E::Overflow)
            } else {
                Ok(p as $t)
            }
        }
        #[doc = "Saturating casts `" $f "` to `" $t "` clamping at the numeric bounds."]
        #[must_use]
        #[inline]
        pub const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            if p < <$t>::MIN as $f {
                <$t>::MIN
            } else if p > $t::MAX as $f {
                <$t>::MAX
            } else {
                p as $t
            }
        }
    }};
    (can_overflow $( $f:ty:$t:ty ),+) => { $( impl_casts![@can_overflow $f:$t]; )+ };
    (@can_overflow $f:ty:$t:ty) => { paste! {
        #[doc = "Safely casts `" $f "` to `" $t "` with range check."]
        ///
        /// # Errors
        #[doc = "Returns an [`Overflow`][E::Overflow] error if `p` > [`" $t "::MAX`]."]
        #[inline]
        pub const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            iif![p > <$t>::MAX as $f; Err(E::Overflow); Ok(p as $t)]
        }
        #[doc = "Saturating casts `" $f "` to `" $t "` clamping at the numeric bounds."]
        #[must_use]
        #[inline]
        pub const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            iif![p > <$t>::MAX as $f; <$t>::MAX; p as $t]
        }
    }};
    (can_underflow $( $f:ty:$t:ty ),+) => { $( impl_casts![@can_underflow $f:$t]; )+ };
    (@can_underflow $f:ty:$t:ty) => { paste! {
        #[doc = "Safely casts `" $f "` to `" $t "` with range check."]
        ///
        /// # Errors
        #[doc = "Returns an [`Underflow`][E::Underflow] error if `p` < `0`."]
        #[inline]
        pub const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            iif![p < 0; Err(E::Underflow); Ok(p as $t)]
        }
        #[doc = "Saturating casts `" $f "` to `" $t "` clamping at the numeric bounds."]
        #[must_use]
        #[inline]
        pub const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            iif![p < 0; 0; p as $t]
        }
    }};
    (cant_fail $( $f:ty:$t:ty ),+) => { $( impl_casts![@cant_fail $f:$t]; )+ };
    (@cant_fail $f:ty:$t:ty) => { paste! {
        #[doc = "Safely casts `" $f "` to `" $t "` *(never fails)*."]
        ///
        /// # Errors
        /// This function always succeeds.
        #[inline(always)]
        pub const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            Ok(p as $t)
        }
        #[doc = "Saturating casts `" $f "` to `" $t "` *(never clamps)*."]
        #[must_use]
        #[inline]
        pub const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
    }};
    (cant_fail ptr:$ptr:literal $( $f:ty:$t:ty ),+) => {
        $( impl_casts![@cant_fail ptr:$ptr $f:$t]; )+
    };
    (@cant_fail ptr:$ptr:literal $f:ty:$t:ty) => { paste! {
        #[doc = "Safely casts `" $f "` to `" $t "` *(never fails in " $ptr "-bit)*."]
        ///
        /// # Errors
        #[doc = "This function always succeeds for the current pointer size of " $ptr "-bit."]
        #[inline(always)]
        pub const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            Ok(p as $t)
        }
        #[doc = "Saturating casts `" $f "` to `" $t "` *(never clamps in " $ptr "-bit)*."]
        #[must_use]
        #[inline]
        pub const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
    }};
}
impl_casts![];
