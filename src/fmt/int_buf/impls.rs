// devela::fmt::int_buf::impls
//
//!
//

use super::*;
use crate::num::{
    NonSpecificI128, NonSpecificI16, NonSpecificI32, NonSpecificI64, NonSpecificI8,
    NonSpecificIsize, NonSpecificU128, NonSpecificU16, NonSpecificU32, NonSpecificU64,
    NonSpecificU8, NonSpecificUsize,
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

// Adaptation of the original implementation at
// https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L188-L266
macro_rules! impl_IntBufAble {
    // Implementation for integer primitives
    (int: $($max_len:expr => $t:ident),* as $conv_fn:ident) => {$(
        impl IntBufAble for $t {}

        impl private::Sealed for $t {
            type IntBuf = [MaybeUninit<u8>; $max_len];

            #[allow(unused_comparisons)]
            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &str {
                let bytes = self.write_bytes(buf);
                unsafe { str::from_utf8_unchecked(bytes) }
            }

            #[allow(unused_comparisons)]
            #[inline]
            fn write_bytes(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &[u8] {
                let is_nonnegative = self >= 0;
                let mut n = if is_nonnegative {
                    self as $conv_fn
                } else {
                    // convert the negative num to positive by summing 1 to it's 2 complement
                    (!(self as $conv_fn)).wrapping_add(1)
                };
                let mut curr = buf.len() as isize;
                let buf_ptr = buf.as_mut_ptr() as *mut u8;
                let lut_ptr = DEC_DIGITS_LUT.as_ptr();

                unsafe {
                    // need at least 16 bits for the 4-characters-at-a-time to work.
                    if mem::size_of::<$t>() >= 2 {
                        // eagerly decode 4 characters at a time
                        while n >= 10000 {
                            let rem = (n % 10000) as isize;
                            n /= 10000;

                            let d1 = (rem / 100) << 1;
                            let d2 = (rem % 100) << 1;
                            curr -= 4;
                            ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                            ptr::copy_nonoverlapping(lut_ptr.offset(d2), buf_ptr.offset(curr + 2), 2);
                        }
                    }

                    // if we reach here numbers are <= 9999, so at most 4 chars long
                    let mut n = n as isize; // possibly reduce 64bit math

                    // decode 2 more chars, if > 2 chars
                    if n >= 100 {
                        let d1 = (n % 100) << 1;
                        n /= 100;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    }

                    // decode last 1 or 2 chars
                    if n < 10 {
                        curr -= 1;
                        *buf_ptr.offset(curr) = (n as u8) + b'0';
                    } else {
                        let d1 = n << 1;
                        curr -= 2;
                        ptr::copy_nonoverlapping(lut_ptr.offset(d1), buf_ptr.offset(curr), 2);
                    }

                    if !is_nonnegative {
                        curr -= 1;
                        *buf_ptr.offset(curr) = b'-';
                    }
                }

                let len = buf.len() - curr as usize;
                unsafe { slice::from_raw_parts(buf_ptr.offset(curr), len) }
            }
        }
    )*};

    // Implementation for NonZero* which defers to the implementation for integers.
    (non_zero: $($max_len:expr => $t:ident($inner_t:ident)),*) => {$(
        impl IntBufAble for $t {}

        impl private::Sealed for $t {
            type IntBuf = [MaybeUninit<u8>; $max_len];

            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &str {
                <$inner_t as private::Sealed>::write(self.get(), buf)
            }
            #[inline]
            fn write_bytes(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &[u8] {
                <$inner_t as private::Sealed>::write_bytes(self.get(), buf)
            }
        }
    )*};
    // Implementation for NonSpecific* which defers to the implementation for integers.
    (non_specific: $($max_len:expr => $t:ident($inner_t:ident)),*) => {$(
        impl<const V: $inner_t> IntBufAble for $t<V> {}

        impl<const V: $inner_t> private::Sealed for $t<V> {
            type IntBuf = [MaybeUninit<u8>; $max_len];

            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &str {
                <$inner_t as private::Sealed>::write(self.get(), buf)
            }
            #[inline]
            fn write_bytes(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &[u8] {
                <$inner_t as private::Sealed>::write_bytes(self.get(), buf)
            }
        }
    )*};
}

macro_rules! impl_IntBufAble128 {
    (int: $($max_len:expr => $t:ident),*) => {$(
        impl IntBufAble for $t {}

        impl private::Sealed for $t {
            type IntBuf = [MaybeUninit<u8>; $max_len];

            #[allow(unused_comparisons)]
            #[inline]
            fn write(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &str {
                let bytes = self.write_bytes(buf);
                unsafe { str::from_utf8_unchecked(bytes) }
            }

            #[allow(unused_comparisons)]
            #[inline]
            fn write_bytes(self, buf: &mut [MaybeUninit<u8>; $max_len]) -> &[u8] {
                let is_nonnegative = self >= 0;
                let n = if is_nonnegative {
                    self as u128
                } else {
                    // convert the negative num to positive by summing 1 to it's 2 complement
                    (!(self as u128)).wrapping_add(1)
                };
                let mut curr = buf.len() as isize;
                let buf_ptr = buf.as_mut_ptr() as *mut u8;

                unsafe {
                    // Divide by 10^19 which is the highest power less than 2^64.
                    let (n, rem) = udivmod_1e19(n);
                    let buf1 = buf_ptr.offset(curr - U64_MAX_LEN as isize)
                        as *mut [MaybeUninit<u8>; U64_MAX_LEN];
                    curr -= rem.write(&mut *buf1).len() as isize;

                    if n != 0 {
                        // Memset the base10 leading zeros of rem.
                        let target = buf.len() as isize - 19;
                        ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                        curr = target;

                        // Divide by 10^19 again.
                        let (n, rem) = udivmod_1e19(n);
                        let buf2 = buf_ptr.offset(curr - U64_MAX_LEN as isize)
                            as *mut [MaybeUninit<u8>; U64_MAX_LEN];
                        curr -= rem.write(&mut *buf2).len() as isize;

                        if n != 0 {
                            // Memset the leading zeros.
                            let target = buf.len() as isize - 38;
                            ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                            curr = target;

                            // There is at most one digit left
                            // because u128::max / 10^19 / 10^19 is 3.
                            curr -= 1;
                            *buf_ptr.offset(curr) = (n as u8) + b'0';
                        }
                    }

                    if !is_nonnegative {
                        curr -= 1;
                        *buf_ptr.offset(curr) = b'-';
                    }

                    let len = buf.len() - curr as usize;
                    slice::from_raw_parts(buf_ptr.offset(curr), len)
                }
            }
        }
    )*};
}

/* impl for primitives */

impl_IntBufAble!(int:
    I8_MAX_LEN => i8,
    U8_MAX_LEN => u8,
    I16_MAX_LEN => i16,
    U16_MAX_LEN => u16,
    I32_MAX_LEN => i32,
    U32_MAX_LEN => u32
    as u32
);
impl_IntBufAble!(int: I64_MAX_LEN => i64, U64_MAX_LEN => u64 as u64);
impl_IntBufAble128!(int: I128_MAX_LEN => i128, U128_MAX_LEN => u128);

#[cfg(target_pointer_width = "16")]
impl_IntBufAble!(int: I16_MAX_LEN => isize, U16_MAX_LEN => usize as u16);
#[cfg(target_pointer_width = "32")]
impl_IntBufAble!(int: I32_MAX_LEN => isize, U32_MAX_LEN => usize as u32);
#[cfg(target_pointer_width = "64")]
impl_IntBufAble!(int: I64_MAX_LEN => isize, U64_MAX_LEN => usize as u64);

/* impl for NonZero* */

impl_IntBufAble!(non_zero:
    I8_MAX_LEN => NonZeroI8(i8),
    U8_MAX_LEN => NonZeroU8(u8),
    I16_MAX_LEN => NonZeroI16(i16),
    U16_MAX_LEN => NonZeroU16(u16),
    I32_MAX_LEN => NonZeroI32(i32),
    U32_MAX_LEN => NonZeroU32(u32),
    I64_MAX_LEN => NonZeroI64(i64),
    U64_MAX_LEN => NonZeroU64(u64),
    I128_MAX_LEN => NonZeroI128(i128),
    U128_MAX_LEN => NonZeroU128(u128)
);

#[cfg(target_pointer_width = "16")]
impl_IntBufAble!(non_zero:
    I16_MAX_LEN => NonZeroIsize(isize),
    U16_MAX_LEN => NonZeroUsize(usize)
);
#[cfg(target_pointer_width = "32")]
impl_IntBufAble!(non_zero:
    I32_MAX_LEN => NonZeroIsize(isize),
    U32_MAX_LEN => NonZeroUsize(usize)
);
#[cfg(target_pointer_width = "64")]
impl_IntBufAble!(non_zero:
    I64_MAX_LEN => NonZeroIsize(isize),
    U64_MAX_LEN => NonZeroUsize(usize)
);
/* impl for NonSpecific* */

impl_IntBufAble!(non_specific:
    I8_MAX_LEN => NonSpecificI8(i8),
    U8_MAX_LEN => NonSpecificU8(u8),
    I16_MAX_LEN => NonSpecificI16(i16),
    U16_MAX_LEN => NonSpecificU16(u16),
    I32_MAX_LEN => NonSpecificI32(i32),
    U32_MAX_LEN => NonSpecificU32(u32),
    I64_MAX_LEN => NonSpecificI64(i64),
    U64_MAX_LEN => NonSpecificU64(u64),
    I128_MAX_LEN => NonSpecificI128(i128),
    U128_MAX_LEN => NonSpecificU128(u128)
);
#[cfg(target_pointer_width = "16")]
impl_IntBufAble!(non_specific:
    I16_MAX_LEN => NonSpecificIsize(isize),
    U16_MAX_LEN => NonSpecificUsize(usize)
);
#[cfg(target_pointer_width = "32")]
impl_IntBufAble!(non_specific:
    I32_MAX_LEN => NonSpecificIsize(isize),
    U32_MAX_LEN => NonSpecificUsize(usize)
);
#[cfg(target_pointer_width = "64")]
impl_IntBufAble!(non_specific:
    I64_MAX_LEN => NonSpecificIsize(isize),
    U64_MAX_LEN => NonSpecificUsize(usize)
);
