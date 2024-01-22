// devela::num::int::wrapper::impl_factors
//
//! implements base related functions
//
// TOC
// - signed|unsigned:
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

use crate::{
    code::{iif, paste},
    num::{Int, NumErrors as E, NumResult as Result},
};
use E::MismatchedSizes;
#[cfg(feature = "alloc")]
use {
    crate::code::Also,
    ::_alloc::{collections::BTreeSet, vec, vec::Vec},
};

// $t:   the input/output type
// $d:  the doclink suffix for the method name
macro_rules! impl_factors {
    (signed $( $t:ty : $d:literal ),+) => { $( impl_factors![@signed $t:$d]; )+ };
    (unsigned $( $t:ty : $d:literal ),+) => { $( impl_factors![@unsigned $t:$d]; )+ };

    // implements signed ops
    (@signed $t:ty : $d:literal) => { paste! {
        #[doc = "# Integer factors related methods for `" $t "`\n\n"]
        /// - Allocating:
        #[doc = "   - [factors](#method.factors" $d ")"]
        #[doc = "   - [factors_proper](#method.factors_proper" $d ")"]
        #[doc = "   - [factors_prime](#method.factors_prime" $d ")"]
        #[doc = "   - [factors_prime_unique](#method.factors_prime_unique" $d ")"]
        /// - Not allocating:
        #[doc = "   - [factors_buf](#method.factors_buf" $d ")"]
        #[doc = "   - [factors_proper_buf](#method.factors_proper_buf" $d ")"]
        #[doc = "   - [factors_prime_buf](#method.factors_prime_buf" $d ")"]
        #[doc = "   - [factors_prime_unique_buf](#method.factors_prime_unique_buf" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* signed factors alloc */

            /// Returns the factors (including 1 and self).
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors(), vec![1]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors(), vec![1, 7]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors(mut self) -> Vec<$t> {
                self.0 = self.0.abs();
                iif![self.0 == 0; return vec![];
                iif![self.0 == 1; return vec![1]]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
                    let temp = set.clone();
                    let mut x = p;
                    while x <= self.0 {
                        for &num in &temp {
                            let new_num = num * x;
                            iif!{self.0 % new_num == 0; {set.insert(new_num);} }
                        }
                        x *= p;
                    }
                }
                set.into_iter().collect()
            }

            /// Returns the proper factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_proper(), vec![2, 3, 4, 6, 8, 12]];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors_proper(), vec![2, 3, 4, 6, 8, 12]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_proper(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_proper(), vec![]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_proper(), vec![]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors_proper(mut self) -> Vec<$t> {
                self.0 = self.0.abs();
                iif![self.0 == 0; return vec![]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
                    let temp = set.clone();
                    let mut x = p;
                    while x <= self.0 {
                        for &num in &temp {
                            let new_num = num * x;
                            if self.0 % new_num == 0 {
                                set.insert(new_num);
                            }
                        }
                        x *= p;
                    }
                }
                set.remove(&1);
                set.remove(&self.0);
                set.into_iter().collect()
            }

            /// Returns the prime factors.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime(), vec![2, 2, 2, 3]];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors_prime(), vec![2, 2, 2, 3]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_prime(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime(), vec![]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime(), vec![7]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors_prime(self) -> Vec<$t> {
                let mut factors = Vec::new();
                iif![self.0 == 0; return factors];
                let mut n = self.0.abs();
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

            /// Returns the unique prime factors.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors_prime_unique(self) -> Vec<$t> {
                self.factors_prime().also_mut(|v| v.dedup())
            }

            /* signed factors non_alloc */

            /// Writes the factors in `fbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns a tuple with the number of factors and unique prime factors found,
            /// or [`MismatchedSizes`] if the total number of factors is greater
            /// than the length of any buffer.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
            #[doc = "assert_eq![Int(24_" $t ").factors_buf(&mut fbuf, &mut upbuf), Ok((8, 2))];"]
            ///
            /// assert_eq![fbuf[..8], [1, 2, 3, 4, 6, 8, 12, 24]];
            /// assert_eq![upbuf[..2], [2, 3]];
            /// ```
            #[inline]
            pub fn factors_buf(mut self, fbuf: &mut [$t], upfbuf: &mut [$t])
                -> Result<(usize, usize)> {
                self.0 = self.0.abs();
                iif![self.0 == 0; return Ok((0, 0))];
                iif![self.0 == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                fbuf[f_count] = 1;
                f_count += 1;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..=self.0 {
                    if self.0 % i == 0 {
                        if f_count < fbuf.len() {
                            fbuf[f_count] = i;
                            f_count += 1;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok((f_count, prime_factors_count))
            }

            /// Writes the proper factors in `fbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns a tuple with the number of factors and unique prime factors found,
            /// or [`MismatchedSizes`] if the total number of factors is greater
            /// than the length of any buffer.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
            #[doc = "assert_eq![Int(24_" $t
                ").factors_proper_buf(&mut fbuf, &mut upbuf), Ok((6, 2))];"]
            ///
            /// assert_eq![fbuf[..6], [2, 3, 4, 6, 8, 12,]];
            /// assert_eq![upbuf[..2], [2, 3]];
            /// ```
            #[inline]
            pub fn factors_proper_buf(mut self, fbuf: &mut [$t], upfbuf: &mut [$t])
                -> Result<(usize, usize)> {
                self.0 = self.0.abs();
                iif![self.0 == 0; return Ok((0, 0))];
                iif![self.0 == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..self.0 {
                    if self.0 % i == 0 {
                        if f_count < fbuf.len() {
                            fbuf[f_count] = i;
                            f_count += 1;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok((f_count, prime_factors_count))
            }

            /// Writes the prime factors in the given `buffer`.
            ///
            /// Returns the number of factors found, or [`MismatchedSizes`] if the total number
            /// of factors is greater than the length of the `buffer`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut buf = [0; 5];
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_buf(&mut buf), Ok(4)];"]
            ///
            /// assert_eq![buf[..4], [2, 2, 2, 3]];
            #[doc = "assert![Int(24_" $t " * 4).factors_prime_buf(&mut buf).is_err()];"]
            /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the 3 didn't fit
            ///
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_buf(&mut buf), Ok(0)];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_buf(&mut buf), Ok(0)];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_buf(&mut buf), Ok(1)];"]
            /// assert_eq![buf[..1], [7]];
            /// ```
            #[inline]
            pub fn factors_prime_buf(self, buffer: &mut [$t]) -> Result<usize> {
                iif![self.0 == 0; return Ok(0)];
                let (mut n, mut idx) = (self.0.abs(), 0);
                while n % 2 == 0 {
                    if idx < buffer.len() {
                        buffer[idx] = 2; idx += 1; n /= 2;
                    } else {
                        return Err(MismatchedSizes);
                    }
                }
                let mut i = 3;
                while i * i <= n {
                    while n % i == 0 {
                        if idx < buffer.len() {
                            buffer[idx] = i; idx += 1; n /= i;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                    i += 2;
                }
                if n > 2 {
                    if idx < buffer.len() {
                        buffer[idx] = n; idx += 1;
                    } else {
                        return Err(MismatchedSizes);
                    }
                }
                Ok(idx)
            }

            /// Writes the prime factors in the given `buffer`.
            ///
            /// The buffer must be large enough to hold all the non-unique factors of `n`.
            /// In that case the function will return the number of unique factors found.
            ///
            /// Otherwise it will return `MismatchedSizes`, and the buffer will only contain
            /// the non-unique factors that could fit, like
            #[doc = "[`factors_prime_buf`](#method.factors_prime_buf" $d ")."]
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut uniq = [0; 5];
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique_buf(&mut uniq), Ok(2)];"]
            /// assert_eq![uniq, [2, 3, 2, 3, 0]];
            /// ```
            #[inline]
            pub fn factors_prime_unique_buf(self, buffer: &mut [$t]) -> Result<usize> {
                let prime_factors_count = self.factors_prime_buf(buffer)?;
                let mut unique_count = 1;
                let mut last_unique = buffer[0];
                for i in 1..prime_factors_count {
                    if buffer[i] != last_unique {
                        if unique_count < buffer.len() {
                            buffer[unique_count] = buffer[i];
                            last_unique = buffer[i];
                            unique_count += 1;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok(unique_count)
            }

            /// Writes the prime factors in `pfbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns the number of factors found, or `MismatchedSizes` if the total number
            /// of factors is greater than the length of the `buffer`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let (mut fac, mut uniq) = ([0; 5], [0; 5]);
            #[doc = "assert_eq![Int(24_" $t
                ").factors_prime_unique_plus_buf(&mut fac, &mut uniq), Ok((4, 2))];"]
            /// assert_eq![fac, [2, 2, 2, 3, 0]];
            /// assert_eq![uniq, [2, 3, 0, 0, 0]];
            /// ```
            #[inline]
            pub fn factors_prime_unique_plus_buf(self, pfbuf: &mut [$t], upfbuf: &mut [$t]
            ) -> Result<(usize, usize)> {
                let prime_factors_count = self.factors_prime_buf(pfbuf)?;
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
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok((prime_factors_count, unique_count))
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $d:literal) => { paste! {
        #[doc = "# Integer factors related methods for `" $t "`\n\n"]
        /// - Allocating:
        #[doc = "   - [factors](#method.factors" $d ")"]
        #[doc = "   - [factors_proper](#method.factors_proper" $d ")"]
        #[doc = "   - [factors_prime](#method.factors_prime" $d ")"]
        #[doc = "   - [factors_prime_unique](#method.factors_prime_unique" $d ")"]
        /// - Not allocating:
        #[doc = "   - [factors_buf](#method.factors_buf" $d ")"]
        #[doc = "   - [factors_proper_buf](#method.factors_proper_buf" $d ")"]
        #[doc = "   - [factors_prime_buf](#method.factors_prime_buf" $d ")"]
        #[doc = "   - [factors_prime_unique_buf](#method.factors_prime_unique_buf" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* unsigned factors alloc */

            /// Returns the factors (including 1 and self).
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors(), vec![1]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors(), vec![1, 7]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors(self) -> Vec<$t> {
                iif![self.0 == 0; return vec![]; iif![self.0 == 1; return vec![1]]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
                    let temp = set.clone();
                    let mut x = p;
                    while x <= self.0 {
                        for &num in &temp {
                            let new_num = num * x;
                            if self.0 % new_num == 0 {
                                set.insert(new_num);
                            }
                        }
                        x *= p;
                    }
                }
                set.into_iter().collect()
            }

            /// Returns the proper factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_proper(), vec![2, 3, 4, 6, 8, 12]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_proper(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_proper(), vec![]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_proper(), vec![]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors_proper(self) -> Vec<$t> {
                iif![self.0 == 0; return vec![]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
                    let temp = set.clone();
                    let mut x = p;
                    while x <= self.0 {
                        for &num in &temp {
                            let new_num = num * x;
                            if self.0 % new_num == 0 {
                                set.insert(new_num);
                            }
                        }
                        x *= p;
                    }
                }
                set.remove(&1);
                set.remove(&self.0);
                set.into_iter().collect()
            }

            /// Returns the prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime(), vec![2, 2, 2, 3]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_prime(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime(), vec![]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime(), vec![7]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors_prime(mut self) -> Vec<$t> {
                let mut factors = Vec::new();
                iif![self.0 == 0; return factors];
                // Divide by 2 until the number is odd
                while self.0 % 2 == 0 {
                    factors.push(2);
                    self.0 /= 2;
                }
                // Divide by odd numbers starting from 3
                let mut i = 3;
                while i * i <= self.0 {
                    while self.0 % i == 0 {
                        factors.push(i);
                        self.0 /= i;
                    }
                    i += 2;
                }
                // If the remaining number is greater than 2, it's a prime factor
                iif![self.0 > 2; factors.push(self.0)];
                factors
            }

            /// Returns the unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
            pub fn factors_prime_unique(self) -> Vec<$t> {
                self.factors_prime().also_mut(|v| v.dedup())
            }

            /* unsigned factors non_alloc */

            /// Writes the factors in `fbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns a tuple with the number of factors and unique prime factors found,
            /// or `MismatchedSizes` if the total number of factors is greater
            /// than the length of any buffer.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
            #[doc = "assert_eq![Int(24_" $t ").factors_buf(&mut fbuf, &mut upbuf), Ok((8, 2))];"]
            ///
            /// assert_eq![fbuf[..8], [1, 2, 3, 4, 6, 8, 12, 24]];
            /// assert_eq![upbuf[..2], [2, 3]];
            /// ```
            #[inline]
            pub fn factors_buf(self, fbuf: &mut [$t], upfbuf: &mut [$t]) -> Result<(usize, usize)> {
                iif![self.0 == 0; return Ok((0, 0))];
                iif![self.0 == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                fbuf[f_count] = 1;
                f_count += 1;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..=self.0 {
                    if self.0 % i == 0 {
                        if f_count < fbuf.len() {
                            fbuf[f_count] = i; f_count += 1;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok((f_count, prime_factors_count))
            }

            /// Writes the proper factors in `fbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns a tuple with the number of factors and unique prime factors found,
            /// or `MismatchedSizes` if the total number of factors is greater
            /// than the length of any buffer.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let (mut fbuf, mut upbuf) = ([0; 20], [0; 20]);
            #[doc = "assert_eq![Int(24_" $t
                ").factors_proper_buf(&mut fbuf, &mut upbuf), Ok((6, 2))];"]
            ///
            /// assert_eq![fbuf[..6], [2, 3, 4, 6, 8, 12,]];
            /// assert_eq![upbuf[..2], [2, 3]];
            /// ```
            #[inline]
            pub fn factors_proper_buf(self, fbuf: &mut [$t], upfbuf: &mut [$t]
            ) -> Result<(usize, usize)> {
                iif![self.0 == 0; return Ok((0, 0))];
                iif![self.0 == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..self.0 {
                    if self.0 % i == 0 {
                        if f_count < fbuf.len() {
                            fbuf[f_count] = i;
                            f_count += 1;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok((f_count, prime_factors_count))
            }

            /// Writes the prime factors in the given `buffer`.
            ///
            /// Returns the number of factors found, or [`MismatchedSizes`] if the total number
            /// of factors is greater than the length of the `buffer`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut buf = [0; 5];
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_buf(&mut buf), Ok(4)];"]
            ///
            /// assert_eq![buf[..4], [2, 2, 2, 3]];
            #[doc = "assert![Int(24_" $t " * 4).factors_prime_buf(&mut buf).is_err()];"]
            /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the 3 didn't fit
            ///
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_buf(&mut buf), Ok(0)];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_buf(&mut buf), Ok(0)];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_buf(&mut buf), Ok(1)];"]
            /// assert_eq![buf[..1], [7]];
            /// ```
            #[inline]
            pub fn factors_prime_buf(self, buffer: &mut [$t]) -> Result<usize> {
                iif![self.0 == 0; return Ok(0)];
                let (mut n, mut idx) = (self.0, 0);
                while n % 2 == 0 {
                    if idx < buffer.len() {
                        buffer[idx] = 2; idx += 1; n /= 2;
                    } else {
                        return Err(MismatchedSizes);
                    }
                }
                let mut i = 3;
                while i * i <= n {
                    while n % i == 0 {
                        if idx < buffer.len() {
                            buffer[idx] = i; idx += 1; n /= i;
                        } else {
                        return Err(MismatchedSizes);
                        }
                    }
                    i += 2;
                }
                if n > 2 {
                    if idx < buffer.len() {
                        buffer[idx] = n; idx += 1;
                    } else {
                        return Err(MismatchedSizes);
                    }
                }
                Ok(idx)
            }

            /// Writes the prime factors in the given `buffer`.
            ///
            /// The buffer must be large enough to hold all the non-unique factors of `n`.
            /// In that case the function will return the number of unique factors found.
            ///
            /// Otherwise it will return [`MismatchedSizes`], and the buffer will only contain
            /// the non-unique factors that could fit. Same as calling the method
            #[doc = "[`factors_prime_buf`](#method.factors_prime_buf" $d ")."]
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut uniq = [0; 5];
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique_buf(&mut uniq), Ok(2)];"]
            /// assert_eq![uniq, [2, 3, 2, 3, 0]];
            /// ```
            #[inline]
            pub fn factors_prime_unique_buf(self, buffer: &mut [$t]) -> Result<usize> {
                let prime_factors_count = self.factors_prime_buf(buffer)?;
                let mut unique_count = 1;
                let mut last_unique = buffer[0];
                for i in 1..prime_factors_count {
                    if buffer[i] != last_unique {
                        if unique_count < buffer.len() {
                            buffer[unique_count] = buffer[i];
                            last_unique = buffer[i];
                            unique_count += 1;
                        } else {
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok(unique_count)
            }

            /// Writes the prime factors in `pfbuf`, and the unique factors in `upfbuf`.
            ///
            /// Returns the number of factors found, or `MismatchedSizes` if the total number
            /// of factors is greater than the length of the `buffer`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let (mut fac, mut uniq) = ([0; 5], [0; 5]);
            #[doc = "assert_eq![Int(24_" $t
                ").factors_prime_unique_plus_buf(&mut fac, &mut uniq), Ok((4, 2))];"]
            /// assert_eq![fac, [2, 2, 2, 3, 0]];
            /// assert_eq![uniq, [2, 3, 0, 0, 0]];
            /// ```
            #[inline]
            pub fn factors_prime_unique_plus_buf(self, pfbuf: &mut [$t], upfbuf: &mut [$t]
                ) -> Result<(usize, usize)> {
                let prime_factors_count = self.factors_prime_buf(pfbuf)?;
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
                            return Err(MismatchedSizes);
                        }
                    }
                }
                Ok((prime_factors_count, unique_count))
            }
        }
    }};
}
impl_factors![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_factors![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
