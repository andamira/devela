// devela::data::mem::size::bit
//
//! Functionality related to memory bit size.
//
// TOC
// - imports
// - fn definitions
// - trait definition
// - trait impls

use crate::data::{BareBox, ByteSize};

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

use crate::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::lex::AsciiChar;
#[cfg(feature = "lex")]
use crate::lex::{Char16, Char24, Char32, Char7, Char8};

// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560#issuecomment-1202124275)
// #[cfg(feature = "lex")]
// use crate::lex::{StringU16, StringU32, EgcU8, StringU8};
#[cfg(feature = "alloc")]
use crate::_alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    string::String,
    vec::Vec,
};
#[cfg(all(feature = "lex", feature = "alloc"))]
use crate::lex::StringEgc;
#[cfg(feature = "lex")]
use crate::lex::{EgcNonul, StringNonul};

#[cfg(feature = "portable-atomic")]
use crate::_deps::portable_atomic::{AtomicF32, AtomicF64, AtomicI128, AtomicU128};
#[cfg(feature = "work")]
use crate::work::sync::atomic::{AtomicBool, AtomicOrdering};
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "16")
))]
use crate::work::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "32")
))]
use crate::work::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "64")
))]
use crate::work::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "8")
))]
use crate::work::sync::atomic::{AtomicI8, AtomicU8};
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "ptr")
))]
use crate::work::sync::atomic::{AtomicIsize, AtomicPtr, AtomicUsize};

/* fn definitions */

/// Returns the rounded up size in bytes from a size in bits.
#[must_use]
#[inline]
pub const fn bytes_from_bits(bit_size: usize) -> usize {
    if let Some(t) = bit_size.checked_add(8 - 1) {
        t / 8
    } else {
        usize::MAX / 8
    }
}

/* trait definition */

/// Type size information in bits.
///
/// Indicates a size of exactly `LEN` bits for the relevant data part of this type.
///
/// E.g. a `bool` has a BitSize of 1 bit.
pub trait BitSize<const LEN: usize>: ByteSize {
    /// The bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSize::BYTE_SIZE],
    const BIT_SIZE: usize = {
        let min_byte_size = bytes_from_bits(LEN);
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSize::MIN_BYTE_SIZE > ByteSize::BYTE_SIZE"];
        }
        LEN
    };

    /// The rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSize::BYTE_SIZE],
    const MIN_BYTE_SIZE: usize = {
        let min_byte_size = bytes_from_bits(LEN);
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSize::MIN_BYTE_SIZE > ByteSize::BYTE_SIZE"];
        }
        min_byte_size
    };

    /// Returns the bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][Size::BYTE_SIZE],
    #[must_use]
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
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSize::BYTE_SIZE],
    #[must_use]
    #[inline]
    fn min_byte_size(&self) -> usize {
        Self::MIN_BYTE_SIZE
    }
}

// Implement BitSize
macro_rules! bit_size {
    /* primitives */

    (= $bits:expr; for $($t:ty),+) => { $( impl BitSize<$bits> for $t {} )+ };

    /* primitives generic on $T */

    (<$T:ident> = $bits:expr; for $($t:ty),+) => {
        $( impl<$T> $crate::data::BitSize<$bits> for $t {} )+
    };
    (<const $T:ident: $Tt:ty> = $bits:expr; for $($t:ty),+) => {
        $( impl<const $T: $Tt> $crate::data::BitSize<$bits> for $t {} )+
    };

    /* primitives generic on $K, $V */

    (<$K:ident, $V:ident> = $bits:expr; for $($t:ty),+) => {
        $( impl<$K, $V> $crate::data::BitSize<$bits> for $t {} )+
    };
    (<const $K:ident: $Kt:ty, const $V:ident: $Vt:ty> = $bits:expr; for $($t:ty),+) => {
        $( impl<const $K: $Kt, const $V: $Vt> $crate::data::BitSize<$bits> for $t {} )+
    };

    /* pointer primitives */

    // implements BitSize<$PTR_BITS> for pointer-sized related types.
    (pointer = $PTR_BITS:literal) => {
        bit_size![= $PTR_BITS; for isize, usize];

        bit_size![= $PTR_BITS; for NonZeroIsize, NonZeroUsize];

        #[cfg(all(feature = "work", any(feature = "portable-atomic", target_has_atomic = "ptr")))]
        bit_size![= $PTR_BITS; for AtomicIsize, AtomicUsize];
        #[cfg(all(feature = "work", any(feature = "portable-atomic", target_has_atomic = "ptr")))]
        bit_size![<T> = $PTR_BITS; for AtomicPtr<T>];

        #[cfg(feature = "std")]
        bit_size![<T> = {$PTR_BITS * 1}; for Rc<T>, Arc<T>];

        bit_size![= {$PTR_BITS * 2}; for &str];
        bit_size![<T> = {$PTR_BITS * 2}; for &[T], &mut [T]];

        #[cfg(feature = "alloc")]
        bit_size![= {$PTR_BITS * 3}; for String];

        #[cfg(all(feature = "alloc", feature = "lex"))]
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
        $(
        impl<T: $crate::data::BitSize<$tsize>> $crate::data::BitSize<{$bits*$len}> for [T; $len] {}
        )+
    };
}
#[allow(unused_imports)] // TEMP
pub(crate) use bit_size;

/* impl BitSize */

impl<T: BitSize<LEN>, const LEN: usize> BitSize<LEN> for BareBox<T> {}

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

bit_size![= 7; for AsciiChar];

#[cfg(feature = "lex")]
bit_size![= 7; for Char7];
#[cfg(feature = "lex")]
bit_size![= 8; for Char8];
#[cfg(feature = "lex")]
bit_size![= 16; for Char16];
#[cfg(feature = "lex")]
bit_size![= 24; for Char24];
#[cfg(feature = "lex")]
bit_size![= 32; for Char32];

bit_size![= 8; for NonZeroI8, NonZeroU8];
bit_size![= 16; for NonZeroI16, NonZeroU16];
bit_size![= 32; for NonZeroI32, NonZeroU32];
bit_size![= 64; for NonZeroI64, NonZeroU64];
bit_size![= 128; for NonZeroI128, NonZeroU128];

#[cfg(feature = "work")]
bit_size![= 1; for AtomicBool];
#[cfg(feature = "work")]
bit_size![= 8; for AtomicOrdering];
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "8")
))]
bit_size![= 8; for AtomicI8, AtomicU8];
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "16")
))]
bit_size![= 16; for AtomicI16, AtomicU16];
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "32")
))]
bit_size![= 32; for AtomicI32, AtomicU32];
#[cfg(all(
    feature = "work",
    any(feature = "portable-atomic", target_has_atomic = "64")
))]
bit_size![= 64; for AtomicI64, AtomicU64];
#[cfg(feature = "portable-atomic")]
bit_size![= 32; for AtomicF32];
#[cfg(feature = "portable-atomic")]
bit_size![= 64; for AtomicF64];
#[cfg(feature = "portable-atomic")]
bit_size![= 128; for AtomicI128, AtomicU128];

#[cfg(feature = "lex")]
bit_size![<const LEN: usize> = LEN; for StringNonul<LEN>, EgcNonul<LEN>];
// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560#issuecomment-1202124275)
// bit_size![<const LEN: usize> = { LEN + 8 }; for StringU8<LEN>, EgcU8<LEN>];
// bit_size![<const LEN: usize> = { LEN + 16 }; for StringU16<LEN>];
// bit_size![<const LEN: usize> = { LEN + 32 }; for StringU32<LEN>];

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
