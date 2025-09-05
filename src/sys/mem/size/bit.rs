// devela::sys::mem::size::bit
//
//! Functionality related to memory bit size.
//
// TOC
// - imports
// - fn definitions
// - trait definition
// - trait impls

#[cfg(feature = "std")]
use crate::{Arc, HashMap, HashSet, Mutex, Rc, SystemInstant, SystemTime};
use crate::{
    AsciiChar, BareBox, ByteSized, Duration, GraphemeNonul, Infallible, Mem, NonZeroI8, NonZeroI16,
    NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU128, NonZeroUsize, Ordering, PhantomData, PhantomPinned, StringNonul,
};

// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560#issuecomment-1202124275)
// use crate::{StringU16, StringU32, GraphemeU8, StringU8};
#[cfg(feature = "alloc")]
use crate::GraphemeString;
#[cfg(feature = "alloc")]
use crate::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, String, Vec, VecDeque};

#[cfg(feature = "dep_portable_atomic")]
use crate::_dep::portable_atomic::{AtomicF32, AtomicF64, AtomicI128, AtomicU128};
#[cfg(feature = "work")]
use crate::{AtomicBool, AtomicOrdering};
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "8")))]
use crate::{AtomicI8, AtomicU8};
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "16")))]
use crate::{AtomicI16, AtomicU16};
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "32")))]
use crate::{AtomicI32, AtomicU32};
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "64")))]
use crate::{AtomicI64, AtomicU64};
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "ptr")))]
use crate::{AtomicIsize, AtomicPtr, AtomicUsize};

/* trait definition */

/// Type size information in bits.
///
/// Indicates a size of exactly `LEN` bits for the relevant data part of this type.
///
/// E.g. a `bool` has a BitSized of 1 bit.
pub trait BitSized<const LEN: usize>: ByteSized {
    /// The bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    const BIT_SIZE: usize = {
        let min_byte_size = Mem::bytes_from_bits(LEN);
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSized::MIN_BYTE_SIZE > ByteSized::BYTE_SIZE"];
        }
        LEN
    };

    /// The rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    const MIN_BYTE_SIZE: usize = {
        let min_byte_size = Mem::bytes_from_bits(LEN);
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSized::MIN_BYTE_SIZE > ByteSized::BYTE_SIZE"];
        }
        min_byte_size
    };

    /// Returns the bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    #[must_use]
    fn bit_size(&self) -> usize {
        Self::BIT_SIZE
    }

    /// Returns the rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    #[must_use]
    fn min_byte_size(&self) -> usize {
        Self::MIN_BYTE_SIZE
    }
}

// Implement BitSized
macro_rules! bit_sized {
    /* primitives */

    (= $bits:expr; for $($t:ty),+) => { $( impl $crate::BitSized<$bits> for $t {} )+ };

    /* primitives generic on $T */

    (<$T:ident> = $bits:expr; for $($t:ty),+) => {
        $( impl<$T> $crate::BitSized<$bits> for $t {} )+
    };
    (<const $T:ident: $Tt:ty> = $bits:expr; for $($t:ty),+) => {
        $( impl<const $T: $Tt> $crate::BitSized<$bits> for $t {} )+
    };

    /* primitives generic on $K, $V */

    (<$K:ident, $V:ident> = $bits:expr; for $($t:ty),+) => {
        $( impl<$K, $V> $crate::BitSized<$bits> for $t {} )+
    };
    (<const $K:ident: $Kt:ty, const $V:ident: $Vt:ty> = $bits:expr; for $($t:ty),+) => {
        $( impl<const $K: $Kt, const $V: $Vt> $crate::BitSized<$bits> for $t {} )+
    };

    /* pointer primitives */

    // implements BitSized<$PTR_BITS> for pointer-sized related types.
    (pointer = $PTR_BITS:literal) => {
        bit_sized![= $PTR_BITS; for isize, usize];

        bit_sized![= $PTR_BITS; for NonZeroIsize, NonZeroUsize];

        #[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "ptr")))]
        bit_sized![= $PTR_BITS; for AtomicIsize, AtomicUsize];
        #[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "ptr")))]
        bit_sized![<T> = $PTR_BITS; for AtomicPtr<T>];

        #[cfg(feature = "std")]
        bit_sized![<T> = {$PTR_BITS * 1}; for Rc<T>, Arc<T>];

        bit_sized![= {$PTR_BITS * 2}; for &str];
        bit_sized![<T> = {$PTR_BITS * 2}; for &[T], &mut [T]];

        #[cfg(feature = "alloc")]
        bit_sized![= {$PTR_BITS * 3}; for String];

        #[cfg(feature = "alloc")]
        bit_sized![= {$PTR_BITS * 3}; for GraphemeString];

        #[cfg(feature = "alloc")]
        bit_sized![<T> = {$PTR_BITS * 3};
            for Vec<T>, LinkedList<T>, VecDeque<T>, BTreeSet<T>, BinaryHeap<T>];
        #[cfg(feature = "std")]
        bit_sized![<T> = {$PTR_BITS * 3}; for HashSet<T>, Mutex<T>];

        // K, V
        #[cfg(feature = "alloc")]
        bit_sized![<K, V> = {$PTR_BITS * 3}; for BTreeMap<K, V>];
        #[cfg(feature = "std")]
        bit_sized![<K, V> = {$PTR_BITS * 3}; for HashMap<K, V>];
    };

    /* arrays */

    (array = $bits:literal * len for T: $tsize:literal * len: $($len:literal),+) => {
        $(
        impl<T: $crate::BitSized<$tsize>> $crate::BitSized<{$bits*$len}> for [T; $len] {}
        )+
    };
}
#[allow(unused_imports)] // TEMP
pub(crate) use bit_sized;

/* impl BitSized */

impl<T: BitSized<LEN>, const LEN: usize> BitSized<LEN> for BareBox<T> {}

bit_sized![<T> = 0; for PhantomData<T>];
bit_sized![= 0; for (), Infallible, PhantomPinned];
bit_sized![= 1; for bool];
bit_sized![= 8; for i8, u8, Ordering];
bit_sized![= 16; for i16, u16];
bit_sized![= 32; for i32, u32, f32, char];
bit_sized![= 64; for i64, u64, f64];
bit_sized![= 128; for i128, u128, Duration];
#[cfg(feature = "std")]
bit_sized![= 128; for SystemInstant, SystemTime];

bit_sized![= 7; for crate::char7];
bit_sized![= 8; for crate::char8];
bit_sized![= 16; for crate::char16];

bit_sized![= 8; for NonZeroI8, NonZeroU8];
bit_sized![= 16; for NonZeroI16, NonZeroU16];
bit_sized![= 32; for NonZeroI32, NonZeroU32];
bit_sized![= 64; for NonZeroI64, NonZeroU64];
bit_sized![= 128; for NonZeroI128, NonZeroU128];

// NOTE: NonValue* types have their implementation in num/niche/impls.rs

#[cfg(feature = "work")]
bit_sized![= 1; for AtomicBool];
#[cfg(feature = "work")]
bit_sized![= 8; for AtomicOrdering];
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "8")))]
bit_sized![= 8; for AtomicI8, AtomicU8];
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "16")))]
bit_sized![= 16; for AtomicI16, AtomicU16];
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "32")))]
bit_sized![= 32; for AtomicI32, AtomicU32];
#[cfg(all(feature = "work", any(feature = "dep_portable_atomic", target_has_atomic = "64")))]
bit_sized![= 64; for AtomicI64, AtomicU64];
#[cfg(feature = "dep_portable_atomic")]
bit_sized![= 32; for AtomicF32];
#[cfg(feature = "dep_portable_atomic")]
bit_sized![= 64; for AtomicF64];
#[cfg(feature = "dep_portable_atomic")]
bit_sized![= 128; for AtomicI128, AtomicU128];

bit_sized![= 7; for AsciiChar];
bit_sized![<const LEN: usize> = LEN; for GraphemeNonul<LEN>, StringNonul<LEN>];
// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560#issuecomment-1202124275)
// bit_sized![<const LEN: usize> = { LEN + 8 }; for StringU8<LEN>, GraphemeU8<LEN>];
// bit_sized![<const LEN: usize> = { LEN + 16 }; for StringU16<LEN>];
// bit_sized![<const LEN: usize> = { LEN + 32 }; for StringU32<LEN>];

#[cfg(target_pointer_width = "8")]
bit_sized![pointer = 8];
#[cfg(target_pointer_width = "16")]
bit_sized![pointer = 16];
#[cfg(target_pointer_width = "32")]
bit_sized![pointer = 32];
#[cfg(target_pointer_width = "64")]
bit_sized![pointer = 64];
#[cfg(target_pointer_width = "128")]
bit_sized![pointer = 128];

// THINK: IMPROVE use const generics?
bit_sized![array = 8 * len for T: 8 * len: 1, 2, 4, 8, 16];
bit_sized![array = 16 * len for T: 16 * len: 1, 2, 4, 8, 16];
bit_sized![array = 24 * len for T: 24 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 32 * len for T: 32 * len: 1, 2, 4, 8, 16];
bit_sized![array = 40 * len for T: 40 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 48 * len for T: 48 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 56 * len for T: 56 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 64 * len for T: 64 * len: 1, 2, 4, 8, 16];
bit_sized![array = 72 * len for T: 72 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 80 * len for T: 80 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 88 * len for T: 88 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 96 * len for T: 96 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 104 * len for T: 104 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 112 * len for T: 112 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 120 * len for T: 120 * len: 1, 2, 4, 8, 16]; // *
bit_sized![array = 128 * len for T: 128 * len: 1, 2, 4, 8, 16];
