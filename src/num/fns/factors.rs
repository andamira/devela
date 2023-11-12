// devela::num::fns::factors
//
//! Functions for numeric operations.
//
// TOC
// - sint|uint:
//   - allocating:
//     - factors
//     - factors_proper
//     - factors_prime
//     - factors_prime_unique
//   - non_allocating:
//     - factors_buf
//     - factors_proper_buf
//     - factors_prime_buf
//     - factors_prime_unique_buf
//     - factors_prime_unique_plus_buf

use crate::meta::{iif, paste};

#[cfg(feature = "alloc")]
use crate::result::Also;
#[cfg(feature = "alloc")]
use ::_alloc::{collections::BTreeSet, vec, vec::Vec};

// $t:   the input/output type
macro_rules! impl_ops {
    (signed $( $t:ty ),+) => { $( impl_ops![@signed $t]; )+ };
    (unsigned $( $t:ty ),+) => { $( impl_ops![@unsigned $t]; )+ };

    // implements signed ops
    (@signed $t:ty) => { paste! {
        /* signed factors alloc */

        #[doc = "Returns the factors of an [`" $t "`] (including 1 and itself)."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_" $t ";\n\n"]
        #[doc = "assert_eq![factors_" $t "(24), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
        #[doc = "assert_eq![factors_" $t "(-24), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
        #[doc = "assert_eq![factors_" $t "(0), vec![]];"]
        #[doc = "assert_eq![factors_" $t "(1), vec![1]];"]
        #[doc = "assert_eq![factors_" $t "(7), vec![1, 7]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_ $t>](mut n: $t) -> Vec<$t> {
            n = n.abs();
            iif![n == 0; return vec![];
            iif![n == 1; return vec![1]]];
            let mut set = BTreeSet::new();
            set.insert(1);
            for p in [<factors_prime_unique_ $t>](n) {
                let temp = set.clone();
                let mut x = p;
                while x <= n {
                    for &num in &temp {
                        let new_num = num * x;
                        iif!{n % new_num == 0; {set.insert(new_num);} }
                    }
                    x *= p;
                }
            }
            set.into_iter().collect()
        }

        #[doc = "Returns the proper factors of an [`" $t "`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_proper_" $t ";\n\n"]
        #[doc = "assert_eq![factors_proper_" $t "(24), vec![2, 3, 4, 6, 8, 12]];"]
        #[doc = "assert_eq![factors_proper_" $t "(-24), vec![2, 3, 4, 6, 8, 12]];"]
        #[doc = "assert_eq![factors_proper_" $t "(0), vec![]];"]
        #[doc = "assert_eq![factors_proper_" $t "(1), vec![]];"]
        #[doc = "assert_eq![factors_proper_" $t "(7), vec![]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_proper_ $t>](mut n: $t) -> Vec<$t> {
            n = n.abs();
            iif![n == 0; return vec![]];
            let mut set = BTreeSet::new();
            set.insert(1);
            for p in [<factors_prime_unique_ $t>](n) {
                let temp = set.clone();
                let mut x = p;
                while x <= n {
                    for &num in &temp {
                        let new_num = num * x;
                        if n % new_num == 0 {
                            set.insert(new_num);
                        }
                    }
                    x *= p;
                }
            }
            set.remove(&1);
            set.remove(&n);
            set.into_iter().collect()
        }

        #[doc = "Returns the prime factors of an [`" $t "`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_" $t ";\n\n"]
        #[doc = "assert_eq![factors_prime_" $t "(24), vec![2, 2, 2, 3]];"]
        #[doc = "assert_eq![factors_prime_" $t "(-24), vec![2, 2, 2, 3]];"]
        #[doc = "assert_eq![factors_prime_" $t "(0), vec![]];"]
        #[doc = "assert_eq![factors_prime_" $t "(1), vec![]];"]
        #[doc = "assert_eq![factors_prime_" $t "(7), vec![7]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_prime_ $t>](n: $t) -> Vec<$t> {
            let mut factors = Vec::new();
            iif![n == 0; return factors];
            let mut n = n.abs();
            // Divide by 2 until the number is odd
            while n % 2 == 0 {
                factors.push(2);
                n /= 2;
            }
            // Divide by odd numbers starting from 3
            let mut i = 3;
            while i * i <= n {
                while n % i == 0 {
                    factors.push(i);
                    n /= i;
                }
                i += 2;
            }
            // If the remaining number is greater than 2, it's a prime factor
            iif![n > 2; factors.push(n)];
            factors
        }

        #[doc = "Returns the unique prime factors of an [`" $t "`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_unique_" $t ";\n\n"]
        #[doc = "assert_eq![factors_prime_unique_" $t "(24), vec![2, 3]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_prime_unique_ $t>](n: $t) -> Vec<$t> {
            [<factors_prime_ $t>](n).also_mut(|v| v.dedup())
        }

        /* signed factors non_alloc */

        #[doc = "Writes the factors of an [`" $t
            "`] in `fbuf`, and the unique prime factors in `upfbuf`."]
        ///
        /// Returns a tuple with the number of factors and unique prime factors found.
        ///
        /// Or `None` if the total number of factors is greater than the length of any buffer.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_" $t "_buf;\n\n"]
        /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
        #[doc = "assert_eq![factors_" $t "_buf(24, &mut fbuf, &mut upbuf), Some((8, 2))];"]
        ///
        /// assert_eq![fbuf[..8], [1, 2, 3, 4, 6, 8, 12, 24]];
        /// assert_eq![upbuf[..2], [2, 3]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_ $t _buf>](
            mut n: $t,
            fbuf: &mut [$t],
            upfbuf: &mut [$t]
        ) -> Option<(usize, usize)> {
            n = n.abs();
            iif![n == 0; return Some((0, 0))];
            iif![n == 1; { fbuf[0] = 1; return Some((1, 0)); }];
            let mut f_count = 0;
            fbuf[f_count] = 1;
            f_count += 1;
            let prime_factors_count = [<factors_prime_unique_ $t _buf>](n, upfbuf)?;
            for i in 2..=n {
                if n % i == 0 {
                    iif![f_count < fbuf.len(); { fbuf[f_count] = i; f_count += 1; }; return None];
                }
            }
            Some((f_count, prime_factors_count))
        }

        #[doc = "Writes the proper factors of an [`" $t
            "`] in `fbuf`, and the unique prime factors in `upfbuf`."]
        ///
        /// Returns a tuple with the number of factors and unique prime factors found.
        ///
        /// Or `None` if the total number of factors is greater than the length of any buffer.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_proper_" $t "_buf;\n\n"]
        /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
        #[doc = "assert_eq![factors_proper_" $t "_buf(24, &mut fbuf, &mut upbuf), Some((6, 2))];"]
        ///
        /// assert_eq![fbuf[..6], [2, 3, 4, 6, 8, 12,]];
        /// assert_eq![upbuf[..2], [2, 3]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_proper_ $t _buf>](
            mut n: $t,
            fbuf: &mut [$t],
            upfbuf: &mut [$t]
        ) -> Option<(usize, usize)> {
            n = n.abs();
            iif![n == 0; return Some((0, 0))];
            iif![n == 1; { fbuf[0] = 1; return Some((1, 0)); }];
            let mut f_count = 0;
            let prime_factors_count = [<factors_prime_unique_ $t _buf>](n, upfbuf)?;
            for i in 2..n {
                if n % i == 0 {
                    if f_count < fbuf.len() {
                        fbuf[f_count] = i;
                        f_count += 1;
                    } else {
                        return None;
                    }
                }
            }
            Some((f_count, prime_factors_count))
        }

        #[doc = "Writes the prime factors of an [`" $t "`] in the given `buffer`."]
        ///
        /// Returns the number of factors found, or `None` if the total number
        /// of factors is greater than the length of the `buffer`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_" $t "_buf;\n\n"]
        /// let mut buf = [0; 5];
        #[doc = "assert_eq![factors_prime_" $t "_buf(24, &mut buf), Some(4)];"]
        ///
        /// assert_eq![buf[..4], [2, 2, 2, 3]];
        #[doc = "assert_eq![factors_prime_" $t "_buf(24 * 4, &mut buf), None];"]
        /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the 3 didn't fit
        ///
        #[doc = "assert_eq![factors_prime_" $t "_buf(0, &mut buf), Some(0)];"]
        #[doc = "assert_eq![factors_prime_" $t "_buf(1, &mut buf), Some(0)];"]
        #[doc = "assert_eq![factors_prime_" $t "_buf(7, &mut buf), Some(1)];"]
        /// assert_eq![buf[..1], [7]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_prime_ $t _buf>](n: $t, buffer: &mut [$t]) -> Option<usize> {
            iif![n == 0; return Some(0)];
            let (mut n, mut idx) = (n.abs(), 0);
            while n % 2 == 0 {
                iif![idx < buffer.len(); { buffer[idx] = 2; idx += 1; n /= 2; }; return None];
            }
            let mut i = 3;
            while i * i <= n {
                while n % i == 0 {
                    iif![idx < buffer.len(); { buffer[idx] = i; idx += 1; n /= i; }; return None];
                }
                i += 2;
            }
            if n > 2 {
                iif![idx < buffer.len(); { buffer[idx] = n; idx += 1; }; return None];
            }
            Some(idx)
        }

        #[doc = "Writes the prime factors of an [`" $t "`] in the given `buffer`."]
        ///
        /// The buffer must be large enough to hold all the non-unique factors of `n`.
        /// In that case the function will return the number of unique factors found.
        ///
        /// Otherwise it will return `None`, and the buffer will only contain
        /// the non-unique factors that could fit. Same as calling the function
        #[doc = "[`factors_prime_" $t "_buf`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_unique_" $t "_buf;\n\n"]
        /// let mut uniq = [0; 5];
        #[doc = "assert_eq![factors_prime_unique_" $t "_buf(24, &mut uniq), Some(2)];"]
        /// assert_eq![uniq, [2, 3, 2, 3, 0]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_prime_unique_ $t _buf>](n: $t, buffer: &mut [$t]) -> Option<usize> {
            let prime_factors_count = [<factors_prime_ $t _buf>](n, buffer)?;
            let mut unique_count = 1;
            let mut last_unique = buffer[0];
            for i in 1..prime_factors_count {
                if buffer[i] != last_unique {
                    if unique_count < buffer.len() {
                        buffer[unique_count] = buffer[i];
                        last_unique = buffer[i];
                        unique_count += 1;
                    } else {
                        return None;
                    }
                }
            }
            Some(unique_count)
        }

        #[doc = "Writes the prime factors of an [`" $t
        "`] in `pfbuf`, and the unique prime factors in `upfbuf`."]
        ///
        /// Returns the number of factors found, or `None` if the total number
        /// of factors is greater than the length of the `buffer`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_unique_plus_" $t "_buf;\n\n"]
        /// let (mut fac, mut uniq) = ([0; 5], [0; 5]);
        #[doc = "assert_eq![factors_prime_unique_plus_" $t
            "_buf(24, &mut fac, &mut uniq), Some((4, 2))];"]
        /// assert_eq![fac, [2, 2, 2, 3, 0]];
        /// assert_eq![uniq, [2, 3, 0, 0, 0]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_prime_unique_plus_ $t _buf >](
            n: $t,
            pfbuf: &mut [$t],
            upfbuf: &mut [$t]
        ) -> Option<(usize, usize)> {
            let prime_factors_count = [<factors_prime_ $t _buf>](n, pfbuf)?;
            let mut unique_count = 0;
            for i in 0..prime_factors_count {
                let mut unique = true;
                for j in 0..unique_count {
                    if pfbuf[i] == upfbuf[j] {
                        unique = false;
                        break;
                    }
                }
                if unique {
                    if unique_count < upfbuf.len() {
                        upfbuf[unique_count] = pfbuf[i];
                        unique_count += 1;
                    } else {
                        return None;
                    }
                }
            }
            Some((prime_factors_count, unique_count))
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty) => { paste! {
        /* unsigned factors alloc */

        #[doc = "Returns the factors of a [`" $t "`] (including 1 and itself)."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_" $t ";\n\n"]
        #[doc = "assert_eq![factors_" $t "(24), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
        #[doc = "assert_eq![factors_" $t "(0), vec![]];"]
        #[doc = "assert_eq![factors_" $t "(1), vec![1]];"]
        #[doc = "assert_eq![factors_" $t "(7), vec![1, 7]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_ $t>](n: $t) -> Vec<$t> {
            iif![n == 0; return vec![]; iif![n == 1; return vec![1]]];
            let mut set = BTreeSet::new();
            set.insert(1);
            for p in [<factors_prime_unique_ $t>](n) {
                let temp = set.clone();
                let mut x = p;
                while x <= n {
                    for &num in &temp {
                        let new_num = num * x;
                        if n % new_num == 0 {
                            set.insert(new_num);
                        }
                    }
                    x *= p;
                }
            }
            set.into_iter().collect()
        }

        #[doc = "Returns the proper factors of a [`" $t "`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_proper_" $t ";\n\n"]
        #[doc = "assert_eq![factors_proper_" $t "(24), vec![2, 3, 4, 6, 8, 12]];"]
        #[doc = "assert_eq![factors_proper_" $t "(0), vec![]];"]
        #[doc = "assert_eq![factors_proper_" $t "(1), vec![]];"]
        #[doc = "assert_eq![factors_proper_" $t "(7), vec![]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_proper_ $t>](n: $t) -> Vec<$t> {
            iif![n == 0; return vec![]];
            let mut set = BTreeSet::new();
            set.insert(1);
            for p in [<factors_prime_unique_ $t>](n) {
                let temp = set.clone();
                let mut x = p;
                while x <= n {
                    for &num in &temp {
                        let new_num = num * x;
                        if n % new_num == 0 {
                            set.insert(new_num);
                        }
                    }
                    x *= p;
                }
            }
            set.remove(&1);
            set.remove(&n);
            set.into_iter().collect()
        }

        #[doc = "Returns the prime factors of a [`" $t "`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_" $t ";\n\n"]
        #[doc = "assert_eq![factors_prime_" $t "(24), vec![2, 2, 2, 3]];"]
        #[doc = "assert_eq![factors_prime_" $t "(0), vec![]];"]
        #[doc = "assert_eq![factors_prime_" $t "(1), vec![]];"]
        #[doc = "assert_eq![factors_prime_" $t "(7), vec![7]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_prime_ $t>](mut n: $t) -> Vec<$t> {
            let mut factors = Vec::new();
            iif![n == 0; return factors];
            // Divide by 2 until the number is odd
            while n % 2 == 0 {
                factors.push(2);
                n /= 2;
            }
            // Divide by odd numbers starting from 3
            let mut i = 3;
            while i * i <= n {
                while n % i == 0 {
                    factors.push(i);
                    n /= i;
                }
                i += 2;
            }
            // If the remaining number is greater than 2, it's a prime factor
            iif![n > 2; factors.push(n)];
            factors
        }

        #[doc = "Returns the unique prime factors of a [`" $t "`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_unique_" $t ";\n\n"]
        #[doc = "assert_eq![factors_prime_unique_" $t "(24), vec![2, 3]];"]
        /// ```
        #[inline]
        #[must_use]
        #[cfg(feature = "alloc")]
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
        pub fn [<factors_prime_unique_ $t>](n: $t) -> Vec<$t> {
            [<factors_prime_ $t>](n).also_mut(|v| v.dedup())
        }

        /* unsigned factors non_alloc */

        #[doc = "Writes the factors of a [`" $t
            "`] in `fbuf`, and the unique prime factors in `upfbuf`."]
        ///
        /// Returns a tuple with the number of factors and unique prime factors found.
        ///
        /// Or `None` if the total number of factors is greater than the length of any buffer.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_" $t "_buf;\n\n"]
        /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
        #[doc = "assert_eq![factors_" $t "_buf(24, &mut fbuf, &mut upbuf), Some((8, 2))];"]
        ///
        /// assert_eq![fbuf[..8], [1, 2, 3, 4, 6, 8, 12, 24]];
        /// assert_eq![upbuf[..2], [2, 3]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_ $t _buf>](
            n: $t,
            fbuf: &mut [$t],
            upfbuf: &mut [$t]
        ) -> Option<(usize, usize)> {
            iif![n == 0; return Some((0, 0))];
            iif![n == 1; { fbuf[0] = 1; return Some((1, 0)); }];
            let mut f_count = 0;
            fbuf[f_count] = 1;
            f_count += 1;
            let prime_factors_count = [<factors_prime_unique_ $t _buf>](n, upfbuf)?;
            for i in 2..=n {
                if n % i == 0 {
                    iif![f_count < fbuf.len(); { fbuf[f_count] = i; f_count += 1; }; return None];
                }
            }
            Some((f_count, prime_factors_count))
        }

        #[doc = "Writes the proper factors of a [`" $t
            "`] in `fbuf`, and the unique prime factors in `upfbuf`."]
        ///
        /// Returns a tuple with the number of factors and unique prime factors found.
        ///
        /// Or `None` if the total number of factors is greater than the length of any buffer.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_proper_" $t "_buf;\n\n"]
        /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
        #[doc = "assert_eq![factors_proper_" $t "_buf(24, &mut fbuf, &mut upbuf), Some((6, 2))];"]
        ///
        /// assert_eq![fbuf[..6], [2, 3, 4, 6, 8, 12,]];
        /// assert_eq![upbuf[..2], [2, 3]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_proper_ $t _buf>](
            n: $t,
            fbuf: &mut [$t],
            upfbuf: &mut [$t]
        ) -> Option<(usize, usize)> {
            iif![n == 0; return Some((0, 0))];
            iif![n == 1; { fbuf[0] = 1; return Some((1, 0)); }];
            let mut f_count = 0;
            let prime_factors_count = [<factors_prime_unique_ $t _buf>](n, upfbuf)?;
            for i in 2..n {
                if n % i == 0 {
                    if f_count < fbuf.len() {
                        fbuf[f_count] = i;
                        f_count += 1;
                    } else {
                        return None;
                    }
                }
            }
            Some((f_count, prime_factors_count))
        }


        #[doc = "Writes the prime factors of a [`" $t "`] in the given `buffer`."]
        ///
        /// Returns the number of factors found, or `None` if the total number
        /// of factors is greater than the length of the `buffer`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_" $t "_buf;\n\n"]
        /// let mut buf = [0; 5];
        #[doc = "assert_eq![factors_prime_" $t "_buf(24, &mut buf), Some(4)];"]
        ///
        /// assert_eq![buf[..4], [2, 2, 2, 3]];
        #[doc = "assert_eq![factors_prime_" $t "_buf(24 * 4, &mut buf), None];"]
        /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the 3 didn't fit
        ///
        #[doc = "assert_eq![factors_prime_" $t "_buf(0, &mut buf), Some(0)];"]
        #[doc = "assert_eq![factors_prime_" $t "_buf(1, &mut buf), Some(0)];"]
        #[doc = "assert_eq![factors_prime_" $t "_buf(7, &mut buf), Some(1)];"]
        /// assert_eq![buf[..1], [7]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_prime_ $t _buf>](n: $t, buffer: &mut [$t]) -> Option<usize> {
            iif![n == 0; return Some(0)];
            let (mut n, mut idx) = (n, 0);
            while n % 2 == 0 {
                iif![idx < buffer.len(); { buffer[idx] = 2; idx += 1; n /= 2; }; return None];
            }
            let mut i = 3;
            while i * i <= n {
                while n % i == 0 {
                    iif![idx < buffer.len(); { buffer[idx] = i; idx += 1; n /= i; }; return None];
                }
                i += 2;
            }
            if n > 2 {
                iif![idx < buffer.len(); { buffer[idx] = n; idx += 1; }; return None];
            }
            Some(idx)
        }

        #[doc = "Writes the prime factors of a [`" $t "`] in the given `buffer`."]
        ///
        /// The buffer must be large enough to hold all the non-unique factors of `n`.
        /// In that case the function will return the number of unique factors found.
        ///
        /// Otherwise it will return `None`, and the buffer will only contain
        /// the non-unique factors that could fit. Same as calling the function
        #[doc = "[`factors_prime_" $t "_buf`]."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_unique_" $t "_buf;\n\n"]
        /// let mut uniq = [0; 5];
        #[doc = "assert_eq![factors_prime_unique_" $t "_buf(24, &mut uniq), Some(2)];"]
        /// assert_eq![uniq, [2, 3, 2, 3, 0]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_prime_unique_ $t _buf>](n: $t, buffer: &mut [$t]) -> Option<usize> {
            let prime_factors_count = [<factors_prime_ $t _buf>](n, buffer)?;
            let mut unique_count = 1;
            let mut last_unique = buffer[0];
            for i in 1..prime_factors_count {
                if buffer[i] != last_unique {
                    if unique_count < buffer.len() {
                        buffer[unique_count] = buffer[i];
                        last_unique = buffer[i];
                        unique_count += 1;
                    } else {
                        return None;
                    }
                }
            }
            Some(unique_count)
        }

        #[doc = "Writes the prime factors of a [`" $t
        "`] in `pfbuf`, and the unique factors in `upfbuf`."]
        ///
        /// Returns the number of factors found, or `None` if the total number
        /// of factors is greater than the length of the `buffer`.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::num::factors_prime_unique_plus_" $t "_buf;\n\n"]
        /// let (mut fac, mut uniq) = ([0; 5], [0; 5]);
        #[doc = "assert_eq![factors_prime_unique_plus_" $t
            "_buf(24, &mut fac, &mut uniq), Some((4, 2))];"]
        /// assert_eq![fac, [2, 2, 2, 3, 0]];
        /// assert_eq![uniq, [2, 3, 0, 0, 0]];
        /// ```
        #[inline]
        #[must_use]
        pub fn [<factors_prime_unique_plus_ $t _buf >](
            n: $t,
            pfbuf: &mut [$t],
            upfbuf: &mut [$t]
        ) -> Option<(usize, usize)> {
            let prime_factors_count = [<factors_prime_ $t _buf>](n, pfbuf)?;
            let mut unique_count = 0;
            for i in 0..prime_factors_count {
                let mut unique = true;
                for j in 0..unique_count {
                    if pfbuf[i] == upfbuf[j] {
                        unique = false;
                        break;
                    }
                }
                if unique {
                    if unique_count < upfbuf.len() {
                        upfbuf[unique_count] = pfbuf[i];
                        unique_count += 1;
                    } else {
                        return None;
                    }
                }
            }
            Some((prime_factors_count, unique_count))
        }
    }};
}
impl_ops![signed i8, i16, i32, i64, i128, isize];
impl_ops![unsigned u8, u16, u32, u64, u128, usize];
