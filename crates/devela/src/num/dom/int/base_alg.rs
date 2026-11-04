// devela/src/num/dom/int/base_alg.rs
//
//! Shared carrier-level arithmetic for integer namespaces and generated values.
//

/// Carrier-level algorithms for the `base` integer family.
#[doc(hidden)]
#[macro_export]
macro_rules! __int_base_alg· {
    (signed digits $t:ty; $a:expr) => {{
        let a: $t = $a;
        let n = if a == <$t>::MIN { <$t>::MAX } else { a.abs() };
        if let Some(c) = n.checked_ilog10() { c as $t + 1 } else { 1 }
    }};
    (signed digits_sign $t:ty; $a:expr) => {{
        let a: $t = $a;
        let mut res = (a < 0) as $t;
        let n = if a == <$t>::MIN { <$t>::MAX } else { a.abs() };
        res += if let Some(c) = n.checked_ilog10() { c as $t + 1 } else { 1 };
        res
    }};
    (signed digits_base $t:ty; $a:expr, $base:expr) => {{
        let mut a: $t = $a;
        let mut base: $t = $base;
        if base == 0 {
            0
        } else {
            base = base.abs();
            a = if a == <$t>::MIN { <$t>::MAX } else { a.abs() };
            if let Some(c) = a.checked_ilog(base) { c as $t + 1 } else { 1 }
        }
    }};
    (signed digits_base_sign $t:ty; $a:expr, $base:expr) => {{
        let mut a: $t = $a;
        let mut base: $t = $base;
        if base == 0 {
            0
        } else {
            base = base.abs();
            let mut res = (a < 0) as $t;
            a = if a == <$t>::MIN { <$t>::MAX } else { a.abs() };
            res += if let Some(c) = a.checked_ilog(base) { c as $t + 1 } else { 1 };
            res
        }
    }};
    (signed digital_root $t:ty; $a:expr) => {{
        let mut n: $t = ($a).abs();
        let mut sum: $t = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
            if n == 0 && sum >= 10 {
                n = sum;
                sum = 0;
            }
        }
        sum
    }};
    (signed digital_root_base $t:ty; $a:expr, $base:expr) => {{
        let (mut n, base): ($t, $t) = (($a).abs(), ($base).abs());
        let mut sum: $t = 0;
        while n > 0 {
            sum += n % base;
            n /= base;
            if n == 0 && sum >= base {
                n = sum;
                sum = 0;
            }
        }
        sum
    }};

    (unsigned digits $t:ty; $a:expr) => {{
        let a: $t = $a;
        if let Some(c) = a.checked_ilog10() { c as $t + 1 } else { 1 }
    }};
    (unsigned digits_base $t:ty; $a:expr, $base:expr) => {{
        let a: $t = $a;
        let base: $t = $base;
        if base == 0 {
            0
        } else if let Some(c) = a.checked_ilog(base) {
            c as $t + 1
        } else {
            1
        }
    }};
    (unsigned digital_root $t:ty; $a:expr) => {{
        let [mut a, mut sum]: [$t; 2] = [$a, 0];
        while a > 0 {
            sum += a % 10;
            a /= 10;
            if a == 0 && sum >= 10 {
                a = sum;
                sum = 0;
            }
        }
        sum
    }};
    (unsigned digital_root_base $t:ty; $a:expr, $base:expr) => {{
        let [mut a, mut sum]: [$t; 2] = [$a, 0];
        let base: $t = $base;
        while a > 0 {
            sum += a % base;
            a /= base;
            if a == 0 && sum >= base {
                a = sum;
                sum = 0;
            }
        }
        sum
    }};
}
pub use __int_base_alg· as __int_base_alg;
