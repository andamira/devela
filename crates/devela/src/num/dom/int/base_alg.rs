//
//! Shared carrier-level arithmetic for integer namespaces and generated values.
//

/// Carrier-level algorithms for the `base` integer family.
///
///
/// Bases are interpreted by magnitude for signed carriers. Radices below 2 return
/// zero (including base 0, for compatibility with `Int::digits_base`).
/// Negative signed inputs are processed by their exact unsigned magnitude,
/// including the signed minimum.
#[doc(hidden)]
#[macro_export]
macro_rules! __int_base_alg· {
    (signed digits $t:ty; $a:expr) => {{
        let a: $t = $a;
        let magnitude = a.unsigned_abs();
        if let Some(c) = magnitude.checked_ilog10() { c as $t + 1 } else { 1 }
    }};
    (signed digits_sign $t:ty; $a:expr) => {{
        let a: $t = $a;
        let magnitude = a.unsigned_abs();
        let digits: $t = if let Some(c) = magnitude.checked_ilog10() { c as $t + 1 } else { 1 };
        digits + (a < 0) as $t
    }};
    (signed digits_base $t:ty; $a:expr, $base:expr) => {{
        let a: $t = $a;
        let base: $t = $base;
        let radix = base.unsigned_abs();
        if radix < 2 {
            0
        } else if let Some(c) = a.unsigned_abs().checked_ilog(radix) {
            c as $t + 1
        } else {
            1
        }
    }};
    (signed digits_base_sign $t:ty; $a:expr, $base:expr) => {{
        let a: $t = $a;
        let base: $t = $base;
        let radix = base.unsigned_abs();
        if radix < 2 {
            0
        } else {
            let digits: $t =
                if let Some(c) = a.unsigned_abs().checked_ilog(radix) { c as $t + 1 } else { 1 };
            digits + (a < 0) as $t
        }
    }};
    (signed digital_root $t:ty; $a:expr) => {{
        let a: $t = $a;
        let n = a.unsigned_abs();
        if n == 0 { 0 } else { (1 + (n - 1) % 9) as $t }
    }};
    (signed digital_root_base $t:ty; $a:expr, $base:expr) => {{
        let a: $t = $a;
        let base: $t = $base;
        let n = a.unsigned_abs();
        let radix = base.unsigned_abs();
        if radix < 2 || n == 0 { 0 } else { (1 + (n - 1) % (radix - 1)) as $t }
    }};

    (unsigned digits $t:ty; $a:expr) => {{
        let a: $t = $a;
        if let Some(c) = a.checked_ilog10() { c as $t + 1 } else { 1 }
    }};
    (unsigned digits_base $t:ty; $a:expr, $base:expr) => {{
        let a: $t = $a;
        let base: $t = $base;
        if base < 2 {
            0
        } else if let Some(c) = a.checked_ilog(base) {
            c as $t + 1
        } else {
            1
        }
    }};
    (unsigned digital_root $t:ty; $a:expr) => {{
        let a: $t = $a;
        if a == 0 { 0 } else { 1 + (a - 1) % 9 }
    }};
    (unsigned digital_root_base $t:ty; $a:expr, $base:expr) => {{
        let a: $t = $a;
        let base: $t = $base;
        if base < 2 || a == 0 { 0 } else { 1 + (a - 1) % (base - 1) }
    }};
}
pub use __int_base_alg· as __int_base_alg;

#[cfg(test)]
mod _test {
    use crate::Int;

    macro_rules! check_signed {
        ($t:ty) => {{
            let min = <$t>::MIN;
            let bits = <$t>::BITS as $t;
            assert_eq!(Int(min).digits_base(2).0, bits);
            assert_eq!(Int(min).digits_base_sign(-2).0, bits + 1);
            assert_eq!(Int(min).digits_base(min).0, 2);
            assert_eq!(Int(min).digits_sign().0, min.unsigned_abs().ilog10() as $t + 2);
            assert_eq!(Int(min).digital_root().0, Int(min).digital_root_base(10).0);
            assert_eq!(Int(min).digital_root_base(min).0, 1);
            for base in [-1, 0, 1] {
                assert_eq!(Int(min).digits_base(base).0, 0);
                assert_eq!(Int(min).digits_base_sign(base).0, 0);
                assert_eq!(Int(min).digital_root_base(base).0, 0);
            }
            assert_eq!(Int(0 as $t).digital_root_base(2).0, 0);
        }};
    }
    macro_rules! check_unsigned {
        ($t:ty) => {{
            let max = <$t>::MAX;
            assert_eq!(Int(max).digits_base(2).0, <$t>::BITS as $t);
            assert_eq!(Int(max).digital_root().0, Int(max).digital_root_base(10).0);
            for base in [0, 1] {
                assert_eq!(Int(max).digits_base(base).0, 0);
                assert_eq!(Int(max).digital_root_base(base).0, 0);
            }
        }};
    }
    #[test]
    fn extrema_and_invalid_radices() {
        check_signed!(i8);
        check_signed!(i16);
        check_signed!(i32);
        check_signed!(i64);
        check_signed!(i128);
        check_signed!(isize);
        check_unsigned!(u8);
        check_unsigned!(u16);
        check_unsigned!(u32);
        check_unsigned!(u64);
        check_unsigned!(u128);
        check_unsigned!(usize);
    }
}
