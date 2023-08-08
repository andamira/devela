// devela::fmt::int_buf
//
//!
//

use core::{
    mem::{self, MaybeUninit},
    ptr, slice, str,
};

mod impls;

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
/// This trait is sealed and cannot be implemented for external types.
///
/// # Derived Work
#[doc = include_str!("./MODIFICATIONS.md")]
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
/// # Derived Work
#[doc = include_str!("./MODIFICATIONS.md")]
#[repr(C)]
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
// T. Granlund and P. Montgomery, “Division by Invariant Integers Using Multiplication”
// in Proc. of the SIGPLAN94 Conference on Programming Language Design and
// Implementation, 1994, pp. 61–72
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
