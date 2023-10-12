// devela::mem::size::bit
//
//! Traits related to stack memory bit size.
//

use super::Size;

use core::{
    cmp,
    convert::Infallible,
    marker::{PhantomData, PhantomPinned},
    time::Duration,
};
#[cfg(feature = "std")]
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Instant, SystemTime},
};

#[cfg(feature = "num")]
use crate::num::{
    NonRangeI128, NonRangeI16, NonRangeI32, NonRangeI64, NonRangeI8, NonRangeIsize, NonRangeU128,
    NonRangeU16, NonRangeU32, NonRangeU64, NonRangeU8, NonRangeUsize, NonSpecificI128,
    NonSpecificI16, NonSpecificI32, NonSpecificI64, NonSpecificI8, NonSpecificIsize,
    NonSpecificU128, NonSpecificU16, NonSpecificU32, NonSpecificU64, NonSpecificU8,
    NonSpecificUsize, NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
    NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, RangeI128, RangeI16,
    RangeI32, RangeI64, RangeI8, RangeIsize, RangeU128, RangeU16, RangeU32, RangeU64, RangeU8,
    RangeUsize,
};

#[cfg(feature = "ascii")]
use crate::ascii::AsciiChar;
#[cfg(feature = "char")]
use crate::char::{Char16, Char24, Char32, Char7, Char8};

// TODO WAITING for: https://github.com/rust-lang/rust/issues/76560#issuecomment-1202124275
// #[cfg(feature = "string")]
// use crate::string::{ArrayU16String, ArrayU32String, ArrayU8Egc, ArrayU8String};
#[cfg(all(feature = "alloc", feature = "string"))]
use crate::string::StringEgc;
#[cfg(feature = "string")]
use crate::string::{ArrayU8NonNulEgc, ArrayU8NonNulString};
#[cfg(feature = "alloc")]
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    string::String,
    vec::Vec,
};

#[cfg(any(all(feature = "sync", feature = "depend"), feature = "portable-atomic"))]
use crate::depend::portable_atomic::{AtomicF32, AtomicF64, AtomicI128, AtomicU128};
#[cfg(feature = "sync")]
use crate::sync::atomic::{AtomicBool, AtomicOrdering};
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "16"
    ),
))]
use crate::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "32"
    ),
))]
use crate::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "64"
    ),
))]
use crate::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "8"
    ),
))]
use crate::sync::atomic::{AtomicI8, AtomicU8};
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "ptr"
    ),
))]
use crate::sync::atomic::{AtomicIsize, AtomicPtr, AtomicUsize};

#[cfg(feature = "term")]
use crate::os::term::{Ansi, AnsiColor3, AnsiColor8};

/* definitions */

/// Indicates a size of exactly `LEN` bits for the relevant data part of this type.
///
/// E.g. a `bool` has a BitSize of 1 bit.
pub trait BitSize<const LEN: usize>: Size {
    /// The bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][Size::BYTE_SIZE],
    const BIT_SIZE: usize = {
        let min_byte_size = if let Some(t) = LEN.checked_add(8 - 1) {
            t / 8
        } else {
            usize::MAX / 8
        };
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSize::MIN_BYTE_SIZE > Size::BYTE_SIZE"];
        }
        LEN
    };

    /// The rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][Size::BYTE_SIZE],
    const MIN_BYTE_SIZE: usize = {
        let min_byte_size = if let Some(t) = LEN.checked_add(8 - 1) {
            t / 8
        } else {
            usize::MAX / 8
        };
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSize::MIN_BYTE_SIZE > Size::BYTE_SIZE"];
        }
        min_byte_size
    };

    /// Returns the bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][Size::BYTE_SIZE],
    #[inline]
    fn bit_size(&self) -> usize {
        Self::BIT_SIZE
    }

    /// Returns the rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][Size::BYTE_SIZE],
    #[inline]
    fn min_byte_size(&self) -> usize {
        Self::MIN_BYTE_SIZE
    }
}

macro_rules! bit_size {
    /* primitives */

    (= $bits:expr; for $($t:ty),+) => { $( impl BitSize<$bits> for $t {} )+ };

    /* primitives generic on $T */

    (<$T:ident> = $bits:expr; for $($t:ty),+) => {
        $( impl<$T> BitSize<$bits> for $t {} )+
    };
    (<const $T:ident: $Tt:ty> = $bits:expr; for $($t:ty),+) => {
        $( impl<const $T: $Tt> BitSize<$bits> for $t {} )+
    };


    /* primitives generic on $K, $V */

    (<$K:ident, $V:ident> = $bits:expr; for $($t:ty),+) => {
        $( impl<$K, $V> BitSize<$bits> for $t {} )+
    };
    (<const $K:ident: $Kt:ty, const $V:ident: $Vt:ty> = $bits:expr; for $($t:ty),+) => {
        $( impl<const $K: $Kt, const $V: $Vt> BitSize<$bits> for $t {} )+
    };


    /* pointer primitives */

    // implements BitSize<$PTR_BITS> for pointer-sized related types.
    (pointer = $PTR_BITS:literal) => {
        bit_size![= $PTR_BITS; for isize, usize];

        #[cfg(feature = "num")]
        bit_size![= $PTR_BITS; for NonZeroIsize, NonZeroUsize];
        #[cfg(feature = "num")]
        bit_size![<const V: isize> = $PTR_BITS; for NonSpecificIsize<V>];
        #[cfg(feature = "num")]
        bit_size![<const V: usize> = $PTR_BITS; for NonSpecificUsize<V>];
        #[cfg(feature = "num")]
        bit_size![<const RMIN: isize, const RMAX: isize> = 8; for NonRangeIsize<RMIN, RMAX>];
        #[cfg(feature = "num")]
        bit_size![<const RMIN: usize, const RMAX: usize> = 8; for NonRangeUsize<RMIN, RMAX>];
        #[cfg(feature = "num")]
        bit_size![<const RMIN: isize, const RMAX: isize> = 8; for RangeIsize<RMIN, RMAX>];
        #[cfg(feature = "num")]
        bit_size![<const RMIN: usize, const RMAX: usize> = 8; for RangeUsize<RMIN, RMAX>];

        #[cfg(all(
            feature = "sync",
            any(
                feature = "depend",
                feature = "portable-atomic",
                target_has_atomic = "ptr"
            ),
        ))]
        bit_size![= $PTR_BITS; for AtomicIsize, AtomicUsize];
        #[cfg(all(
            feature = "sync",
            any(
                feature = "depend",
                feature = "portable-atomic",
                target_has_atomic = "ptr"
            ),
        ))]
        bit_size![<T> = $PTR_BITS; for AtomicPtr<T>];

        #[cfg(feature = "std")]
        bit_size![<T> = {$PTR_BITS * 1}; for Rc<T>, Arc<T>];

        bit_size![= {$PTR_BITS * 2}; for &str];
        bit_size![<T> = {$PTR_BITS * 2}; for &[T], &mut [T]];

        #[cfg(feature = "alloc")]
        bit_size![= {$PTR_BITS * 3}; for String];

        #[cfg(all(feature = "alloc", feature = "string"))]
        bit_size![= {$PTR_BITS * 3}; for StringEgc];

        #[cfg(feature = "alloc")]
        bit_size![<T> = {$PTR_BITS * 3};
            for Vec<T>, LinkedList<T>, VecDeque<T>, BTreeSet<T>, BinaryHeap<T>];
        #[cfg(feature = "std")]
        bit_size![<T> = {$PTR_BITS * 3}; for HashSet<T>, Mutex<T>];

        // K, V
        #[cfg(feature = "alloc")]
        bit_size![<K, V> = {$PTR_BITS * 3}; for BTreeMap<K, V>];
        #[cfg(feature = "std")]
        bit_size![<K, V> = {$PTR_BITS * 3}; for HashMap<K, V>];
    };

    /* arrays */

    (array = $bits:literal * len for T: $tsize:literal * len: $($len:literal),+) => {
        $( impl<T: BitSize<$tsize>> BitSize<{$bits*$len}> for [T; $len] {} )+
    };
}

/* impl BitSize */

bit_size![<T> = 0; for PhantomData<T>];
bit_size![= 0; for (), Infallible, PhantomPinned];
bit_size![= 1; for bool];
bit_size![= 8; for i8, u8, cmp::Ordering];
bit_size![= 16; for i16, u16];
bit_size![= 32; for i32, u32, f32, char];
bit_size![= 64; for i64, u64, f64];
bit_size![= 128; for i128, u128, Duration];
#[cfg(feature = "std")]
bit_size![= 128; for Instant, SystemTime];

#[cfg(feature = "ascii")]
bit_size![= 7; for AsciiChar];

#[cfg(feature = "char")]
bit_size![= 7; for Char7];
#[cfg(feature = "char")]
bit_size![= 8; for Char8];
#[cfg(feature = "char")]
bit_size![= 16; for Char16];
#[cfg(feature = "char")]
bit_size![= 24; for Char24];
#[cfg(feature = "char")]
bit_size![= 32; for Char32];

#[cfg(feature = "num")]
bit_size![= 8; for NonZeroI8, NonZeroU8];
#[cfg(feature = "num")]
bit_size![= 16; for NonZeroI16, NonZeroU16];
#[cfg(feature = "num")]
bit_size![= 32; for NonZeroI32, NonZeroU32];
#[cfg(feature = "num")]
bit_size![= 64; for NonZeroI64, NonZeroU64];
#[cfg(feature = "num")]
bit_size![= 128; for NonZeroI128, NonZeroU128];
#[cfg(feature = "num")]
bit_size![<const V: i8> = 8; for NonSpecificI8<V>];
#[cfg(feature = "num")]
bit_size![<const V: u8> = 8; for NonSpecificU8<V>];
#[cfg(feature = "num")]
bit_size![<const V: i16> = 16; for NonSpecificI16<V>];
#[cfg(feature = "num")]
bit_size![<const V: u16> = 16; for NonSpecificU16<V>];
#[cfg(feature = "num")]
bit_size![<const V: i32> = 32; for NonSpecificI32<V>];
#[cfg(feature = "num")]
bit_size![<const V: u32> = 32; for NonSpecificU32<V>];
#[cfg(feature = "num")]
bit_size![<const V: i64> = 64; for NonSpecificI64<V>];
#[cfg(feature = "num")]
bit_size![<const V: u64> = 64; for NonSpecificU64<V>];
#[cfg(feature = "num")]
bit_size![<const V: i128> = 128; for NonSpecificI128<V>];
#[cfg(feature = "num")]
bit_size![<const V: u128> = 128; for NonSpecificU128<V>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i8, const RMAX:i8> = 8; for NonRangeI8<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u8, const RMAX:u8> = 8; for NonRangeU8<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i16, const RMAX:i16> = 16; for NonRangeI16<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u16, const RMAX:u16> = 16; for NonRangeU16<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i32, const RMAX:i32> = 32; for NonRangeI32<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u32, const RMAX:u32> = 32; for NonRangeU32<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i64, const RMAX:i64> = 64; for NonRangeI64<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u64, const RMAX:u64> = 64; for NonRangeU64<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i128, const RMAX:i128> = 128; for NonRangeI128<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u128, const RMAX:u128> = 128; for NonRangeU128<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i8, const RMAX:i8> = 8; for RangeI8<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u8, const RMAX:u8> = 8; for RangeU8<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i16, const RMAX:i16> = 16; for RangeI16<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u16, const RMAX:u16> = 16; for RangeU16<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i32, const RMAX:i32> = 32; for RangeI32<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u32, const RMAX:u32> = 32; for RangeU32<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i64, const RMAX:i64> = 64; for RangeI64<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u64, const RMAX:u64> = 64; for RangeU64<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: i128, const RMAX:i128> = 128; for RangeI128<RMIN, RMAX>];
#[cfg(feature = "num")]
bit_size![<const RMIN: u128, const RMAX:u128> = 128; for RangeU128<RMIN, RMAX>];

#[cfg(feature = "sync")]
bit_size![= 1; for AtomicBool];
#[cfg(feature = "sync")]
bit_size![= 8; for AtomicOrdering];
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "8"
    ),
))]
bit_size![= 8; for AtomicI8, AtomicU8];
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "16"
    ),
))]
bit_size![= 16; for AtomicI16, AtomicU16];
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "32"
    ),
))]
bit_size![= 32; for AtomicI32, AtomicU32];
#[cfg(all(
    feature = "sync",
    any(
        feature = "depend",
        feature = "portable-atomic",
        target_has_atomic = "64"
    ),
))]
bit_size![= 64; for AtomicI64, AtomicU64];
#[cfg(any(all(feature = "sync", feature = "depend"), feature = "portable-atomic"))]
bit_size![= 32; for AtomicF32];
#[cfg(any(all(feature = "sync", feature = "depend"), feature = "portable-atomic"))]
bit_size![= 64; for AtomicF64];
#[cfg(any(all(feature = "sync", feature = "depend"), feature = "portable-atomic"))]
bit_size![= 128; for AtomicI128, AtomicU128];

#[cfg(feature = "term")]
bit_size![= 0; for Ansi];
#[cfg(feature = "term")]
bit_size![= 3; for AnsiColor3];
#[cfg(feature = "term")]
bit_size![= 8; for AnsiColor8];

#[cfg(feature = "string")]
bit_size![<const LEN: usize> = LEN; for ArrayU8NonNulString<LEN>, ArrayU8NonNulEgc<LEN>];
// TODO WAITING for: https://github.com/rust-lang/rust/issues/76560#issuecomment-1202124275
// #[cfg(feature = "string")]
// bit_size![<const LEN: usize> = { LEN + 8 }; for ArrayU8String<LEN>, ArrayU8Egc<LEN>];
// #[cfg(feature = "string")]
// bit_size![<const LEN: usize> = { LEN + 16 }; for ArrayU16String<LEN>];
// #[cfg(feature = "string")]
// bit_size![<const LEN: usize> = { LEN + 32 }; for ArrayU32String<LEN>];

#[cfg(target_pointer_width = "8")]
bit_size![pointer = 8];
#[cfg(target_pointer_width = "16")]
bit_size![pointer = 16];
#[cfg(target_pointer_width = "32")]
bit_size![pointer = 32];
#[cfg(target_pointer_width = "64")]
bit_size![pointer = 64];
#[cfg(target_pointer_width = "128")]
bit_size![pointer = 128];

// THINK: IMPROVE use const generics?
bit_size![array = 8 * len for T: 8 * len: 1, 2, 4, 8, 16];
bit_size![array = 16 * len for T: 16 * len: 1, 2, 4, 8, 16];
bit_size![array = 24 * len for T: 24 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 32 * len for T: 32 * len: 1, 2, 4, 8, 16];
bit_size![array = 40 * len for T: 40 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 48 * len for T: 48 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 56 * len for T: 56 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 64 * len for T: 64 * len: 1, 2, 4, 8, 16];
bit_size![array = 72 * len for T: 72 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 80 * len for T: 80 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 88 * len for T: 88 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 96 * len for T: 96 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 104 * len for T: 104 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 112 * len for T: 112 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 120 * len for T: 120 * len: 1, 2, 4, 8, 16]; // *
bit_size![array = 128 * len for T: 128 * len: 1, 2, 4, 8, 16];
