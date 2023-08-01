// devela::fmt::int_buf
//
//!
//

use core::{
    mem::{self, MaybeUninit},
    ptr, slice, str,
};

const I8_MAX_LEN: usize = 4;
const U8_MAX_LEN: usize = 3;
const I16_MAX_LEN: usize = 6;
const U16_MAX_LEN: usize = 5;
const I32_MAX_LEN: usize = 11;
const U32_MAX_LEN: usize = 10;
const I64_MAX_LEN: usize = 20;
const U64_MAX_LEN: usize = 20;
const U128_MAX_LEN: usize = 39;
const I128_MAX_LEN: usize = 40;
const DEC_DIGITS_LUT: &[u8] = b"\
      0001020304050607080910111213141516171819\
      2021222324252627282930313233343536373839\
      4041424344454647484950515253545556575859\
      6061626364656667686970717273747576777879\
      8081828384858687888990919293949596979899";

/// An integer that can be written into an [`IntBuf`].
///
/// This is a fork of []
///
/// This trait is sealed and cannot be implemented for external types.
pub trait IntBufAble: private::Sealed {}

// Seal to prevent downstream implementations of the `IntBufAble` trait.
mod private {
    pub trait Sealed: Copy {
        type IntBuf: 'static;
        fn write_bytes(self, buf: &mut Self::IntBuf) -> &[u8];
        fn write(self, buf: &mut Self::IntBuf) -> &str;
    }
}

/// A correctly sized stack allocation for the formatted integer to be written into.
///
/// This is a fork from the [`itoa`](https://crates.io/crates/itoa/1.0.9) crate,
/// with the following changes:
/// - renamed `Buffer` to [`IntBuf`].
/// - renamed `Integer` to [`IntBufAble`].
/// - divided fn `write` into [`write_bytes`][Self#method.write_bytes] and [`write_str`][Self#method.write_str].
/// - renamed fn [`format`] to [`to_str`][Self#method.str].
/// - new fn [`to_bytes`][Self#method.to_bytes].
/// - refactored; updated docs and examples.
/// - removed `no_panic`.
pub struct IntBuf {
    bytes: [MaybeUninit<u8>; I128_MAX_LEN],
}

impl Clone for IntBuf {
    #[inline]
    fn clone(&self) -> Self {
        IntBuf::new()
    }
}

impl Copy for IntBuf {}

impl Default for IntBuf {
    #[inline]
    fn default() -> IntBuf {
        IntBuf::new()
    }
}

impl IntBuf {
    /// This is a cheap operation; you don't need to worry about reusing buffers
    /// for efficiency.
    // #[inline]
    pub fn new() -> IntBuf {
        let bytes = [MaybeUninit::<u8>::uninit(); I128_MAX_LEN];
        IntBuf { bytes }
    }

    /// Print an integer into this buffer and return a reference to its string
    /// representation within the buffer.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!("1234", devela::fmt::IntBuf::new().to_str(1234));
    /// ```
    pub fn to_str<I: IntBufAble>(&mut self, i: I) -> &str {
        i.write(unsafe {
            &mut *(&mut self.bytes as *mut [MaybeUninit<u8>; I128_MAX_LEN]
                as *mut <I as private::Sealed>::IntBuf)
        })
    }

    /// Print an integer into this buffer and return a reference to its bytes
    /// within the buffer.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(&[49, 50, 51, 52], devela::fmt::IntBuf::new().to_bytes(1234));
    /// ```
    pub fn to_bytes<I: IntBufAble>(&mut self, i: I) -> &[u8] {
        i.write_bytes(unsafe {
            &mut *(&mut self.bytes as *mut [MaybeUninit<u8>; I128_MAX_LEN]
                as *mut <I as private::Sealed>::IntBuf)
        })
    }
}

// Multiply unsigned 128 bit integers, return upper 128 bits of the result
#[inline]
fn u128_mulhi(x: u128, y: u128) -> u128 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let y_lo = y as u64;
    let y_hi = (y >> 64) as u64;

    // handle possibility of overflow
    let carry = (x_lo as u128 * y_lo as u128) >> 64;
    let m = x_lo as u128 * y_hi as u128 + carry;
    let high1 = m >> 64;

    let m_lo = m as u64;
    let high2 = (x_hi as u128 * y_lo as u128 + m_lo as u128) >> 64;

    x_hi as u128 * y_hi as u128 + high1 + high2
}

// Divide `n` by 1e19 and return quotient and remainder
//
// Integer division algorithm is based on the following paper:
//
//   T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”
//   in Proc. of the SIGPLAN94 Conference on Programming Language Design and
//   Implementation, 1994, pp. 61–72
//
#[inline]
fn udivmod_1e19(n: u128) -> (u128, u64) {
    let d = 10_000_000_000_000_000_000_u64; // 10^19

    let quot = if n < 1 << 83 {
        ((n >> 19) as u64 / (d >> 19)) as u128
    } else {
        u128_mulhi(n, 156927543384667019095894735580191660403) >> 62
    };

    let rem = (n - quot * d as u128) as u64;
    debug_assert_eq!(quot, n / d as u128);
    debug_assert_eq!(rem as u128, n % d as u128);

    (quot, rem)
}

// Adaptation of the original implementation at
// https://github.com/rust-lang/rust/blob/b8214dc6c6fc20d0a660fb5700dca9ebf51ebe89/src/libcore/fmt/num.rs#L188-L266
macro_rules! impl_IntBufAble {
    ($($max_len:expr => $t:ident),* as $conv_fn:ident) => {$(
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
}

macro_rules! impl_IntBufAble128 {
    ($($max_len:expr => $t:ident),*) => {$(
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
                    let buf1 = buf_ptr.offset(curr - U64_MAX_LEN as isize) as *mut [MaybeUninit<u8>; U64_MAX_LEN];
                    curr -= rem.write(&mut *buf1).len() as isize;

                    if n != 0 {
                        // Memset the base10 leading zeros of rem.
                        let target = buf.len() as isize - 19;
                        ptr::write_bytes(buf_ptr.offset(target), b'0', (curr - target) as usize);
                        curr = target;

                        // Divide by 10^19 again.
                        let (n, rem) = udivmod_1e19(n);
                        let buf2 = buf_ptr.offset(curr - U64_MAX_LEN as isize) as *mut [MaybeUninit<u8>; U64_MAX_LEN];
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

impl_IntBufAble!(
    I8_MAX_LEN => i8,
    U8_MAX_LEN => u8,
    I16_MAX_LEN => i16,
    U16_MAX_LEN => u16,
    I32_MAX_LEN => i32,
    U32_MAX_LEN => u32
    as u32);

impl_IntBufAble128!(I128_MAX_LEN => i128, U128_MAX_LEN => u128);

impl_IntBufAble!(I64_MAX_LEN => i64, U64_MAX_LEN => u64 as u64);

#[cfg(target_pointer_width = "16")]
impl_IntBufAble!(I16_MAX_LEN => isize, U16_MAX_LEN => usize as u16);

#[cfg(target_pointer_width = "32")]
impl_IntBufAble!(I32_MAX_LEN => isize, U32_MAX_LEN => usize as u32);

#[cfg(target_pointer_width = "64")]
impl_IntBufAble!(I64_MAX_LEN => isize, U64_MAX_LEN => usize as u64);
