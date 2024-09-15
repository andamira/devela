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
//     - factors_prime_unique_exp
//   - non_allocating:
//     - factors_prime_count
//     - factors_prime_unique_count
//     - factors_buf
//     - factors_proper_buf
//     - factors_prime_buf
//     - factors_prime_unique_buf
//     - factors_prime_unique_exp_buf
//     - factors_prime_unique_plus_buf

#[cfg(feature = "alloc")]
use crate::{
    _dep::_alloc::{collections::BTreeSet, vec, vec::Vec},
    error::Also,
};
use crate::{
    code::{iif, paste},
    num::{Int, NumError::MismatchedSizes, NumResult as Result},
};

// $t:   the input/output type
// $cap: the capability feature that enables the given implementation. E.g "_int_i8".
// $d:  the doclink suffix for the method name
macro_rules! impl_int {
    () => {
        impl_int![signed
            i8:"_int_i8":"", i16:"_int_i16":"-1", i32:"_int_i32":"-2",
            i64:"_int_i64":"-3", i128:"_int_i128":"-4", isize:"_int_isize":"-5"
        ];
        impl_int![unsigned
            u8:"_int_u8":"-6", u16:"_int_u16":"-7", u32:"_int_u32":"-8",
            u64:"_int_u64":"-9", u128:"_int_u128":"-10", usize:"_int_usize":"-11"
        ];
    };

    (signed $( $t:ty : $cap:literal : $d:literal ),+) => {
        $( impl_int![@signed $t:$cap:$d]; )+
    };
    (unsigned $( $t:ty : $cap:literal : $d:literal ),+) => {
        $( impl_int![@unsigned $t:$cap:$d]; )+
    };

    // implements signed ops
    (@signed $t:ty : $cap:literal : $d:literal) => { paste! {
        #[doc = crate::code::doc_availability!(feature = $cap)]
        ///
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
        #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Int<$t> {
            /* signed factors alloc */

            /// Returns the factors (including 1 and self).
            ///
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
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors(self) -> Vec<$t> {
                let n = self.0.abs();
                iif![n == 0; return vec![];
                iif![n == 1; return vec![1]]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
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
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_proper(self) -> Vec<$t> {
                let n = self.0.abs();
                iif![n == 0; return vec![]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
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

            /// Returns the prime factors.
            ///
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
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_prime(self) -> Vec<$t> {
                let mut factors = Vec::new();
                let mut n = self.0.abs();
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

            /// Returns the unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_prime_unique(self) -> Vec<$t> {
                self.factors_prime().also_mut(|v| v.dedup())
            }

            /// Returns the unique prime factors with its exponent.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique_exp(), vec![(2, 3), (3, 1)]];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors_prime_unique_exp(), vec![(2, 3), (3, 1)]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_unique_exp(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_unique_exp(), vec![]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_unique_exp(), vec![(7, 1)]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_prime_unique_exp(self) -> Vec<($t, u32)> {
                let mut factors = Vec::new();
                let mut current = None;
                let mut count = 0;

                for prime in self.factors_prime() {
                    match current {
                        Some(f) if f == prime => {
                            count += 1;
                        },
                        _ => {
                            if let Some(f) = current {
                                factors.push((f, count));
                            }
                            current = Some(prime);
                            count = 1;
                        },
                    }
                }
                if let Some(f) = current {
                    factors.push((f, count));
                }
                factors
            }

            /* signed factors non_alloc */

            /// Returns the count of prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_count(), 4];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors_prime_count(), 4];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_count(), 0];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_count(), 0];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_count(), 1];"]
            /// ```
            #[inline] #[must_use]
            pub fn factors_prime_count(self) -> usize {
                let mut n = self.0.abs();
                iif![n == 0; return 0];
                let mut count = 0;
                // Divide by 2 until the number is odd
                while n % 2 == 0 {
                    count += 1;
                    n /= 2;
                }
                // Divide by odd numbers starting from 3
                let mut i = 3;
                while i * i <= n {
                    while n % i == 0 {
                        count += 1;
                        n /= i;
                    }
                    i += 2;
                }
                // If the remaining number is greater than 2, it's a prime factor
                iif![n > 2; count += 1];
                count
            }

            /// Returns the count of unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique_count(), 2];"]
            #[doc = "assert_eq![Int(-24_" $t ").factors_prime_unique_count(), 2];"]
            /// ```
            #[inline] #[must_use]
            pub fn factors_prime_unique_count(self) -> usize {
                let mut n = self.0.abs();
                iif![n == 0; return 0];
                let mut count = 0;
                let mut last = 0;

                // Divide by 2 until the number is odd
                while n % 2 == 0 {
                    iif![last != 2; { count += 1; last = 2 }];
                    n /= 2;
                }
                // Divide by odd numbers starting from 3
                let mut i = 3;
                while i * i <= n {
                    while n % i == 0 {
                        iif![last != i; { count += 1; last = i }];
                        n /= i;
                    }
                    i += 2;
                }
                // If the remaining number is greater than 2,
                // and not the same as the last factor, it's a prime factor
                if n > 2 && last != n {
                    count += 1;
                }
                count
            }

            /// Writes the factors in `fbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns a tuple with the number of factors and unique prime factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors is greater
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
            pub fn factors_buf(self, fbuf: &mut [$t], upfbuf: &mut [$t])
                -> Result<(usize, usize)> {
                let n = self.0.abs();
                iif![n == 0; return Ok((0, 0))];
                iif![n == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                fbuf[f_count] = 1;
                f_count += 1;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..=n {
                    if n % i == 0 {
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
            /// Returns a tuple with the number of factors and unique prime factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors is greater
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
            pub fn factors_proper_buf(self, fbuf: &mut [$t], upfbuf: &mut [$t])
                -> Result<(usize, usize)> {
                let n = self.0.abs();
                iif![n == 0; return Ok((0, 0))];
                iif![n == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..n {
                    if n % i == 0 {
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
            /// Returns the number of factors found
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors
            /// is greater than the length of the `buffer`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut buf = [0; 5];
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_buf(&mut buf), Ok(4)];"]
            ///
            /// assert_eq![buf[..4], [2, 2, 2, 3]];
            #[doc = "assert![Int(24_" $t " * 4).factors_prime_buf(&mut buf).is_err()];"]
            /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the factor of 3 didn't fit
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
            /// The `buffer` must be large enough to hold all the non-unique factors of `n`.
            /// In that case the function will return the number of unique factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] otherwise. In that case the buffer
            /// will only contain the non-unique factors that could fit, same as
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

            /// Writes the unique prime factors in the given `fbuffer`, and the
            /// associated exponent in the given `ebuffer` at the same index.
            ///
            /// The `fbuffer` must be large enough to hold all the non-unique factors of `n`.
            /// In that case the function will return the number of unique factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] otherwise. In that case the buffer
            /// will only contain the non-unique factors that could fit, same as
            #[doc = "[`factors_prime_buf`](#method.factors_prime_buf" $d ")."]
            ///
            /// Returns [`MismatchedSizes`] if `ebuffer` is not large enough as well.
            /// In that case the number of unique factors will equal `ebuffer.len()`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut fbuf = [0; 4];
            /// let mut ebuf = [0; 2];
            #[doc = "assert_eq![Int(40_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(2)];"]
            /// assert_eq![fbuf[..2], [2, 5]]; // 2^3, 5^1, …
            /// assert_eq![ebuf[..2], [3, 1]];
            ///
            #[doc = "assert_eq![Int(0_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(0)];"]
            #[doc = "assert_eq![Int(1_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(0)];"]
            #[doc = "assert_eq![Int(7_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(1)];"]
            /// assert_eq![fbuf[..1], [7]]; // 7^1
            /// assert_eq![ebuf[..1], [1]];
            ///
            /// // When `fbuffer` is not large enough:
            /// let mut fbuf = [0; 3];
            /// let mut ebuf = [0; 2];
            #[doc = "assert![Int(24_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf).is_err()];"]
            /// assert_eq![fbuf, [2, 2, 2]]; // the factor of 5 didn't fit
            /// assert_eq![ebuf, [0, 0]]; // the exponents didn't get written
            ///
            /// // When `ebuffer` is not large enough:
            /// let mut fbuf = [0; 4];
            /// let mut ebuf = [0; 1];
            #[doc = "assert![Int(24_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf).is_err()];"]
            /// assert_eq![fbuf[..ebuf.len()], [2]]; // 2^3, Err, …
            /// assert_eq![ebuf[..], [3]];
            /// ```
            // IMPROVE: differenciate between both errors more clearly.
            pub fn factors_prime_unique_exp_buf(self, fbuffer: &mut [$t], ebuffer: &mut [u32])
            -> Result<usize> {
                let prime_factors_count = self.factors_prime_buf(fbuffer)?;
                iif![prime_factors_count == 0; return Ok(0)];

                let mut current_factor = fbuffer[0]; // current factor
                let mut unique_idx = 0; // current unique factor index
                let mut exp_count = 1; //

                for i in 1..prime_factors_count {
                    // Same factor as before, increment the exponent count
                    if fbuffer[i] == current_factor {
                        exp_count += 1;
                    } else {
                        // New factor found, store the previous factor and its exp_count
                        fbuffer[unique_idx] = current_factor;
                        iif![unique_idx >= ebuffer.len(); return Err(MismatchedSizes)];
                        ebuffer[unique_idx] = exp_count;
                        unique_idx += 1; // Move to the next unique factor

                        // Reset for the new factor
                        current_factor = fbuffer[i];
                        exp_count = 1;
                    }
                }
                // Store the last factor and its exponent count
                if unique_idx < fbuffer.len() && unique_idx < ebuffer.len() {
                    fbuffer[unique_idx] = current_factor;
                    ebuffer[unique_idx] = exp_count;
                    unique_idx += 1; // increment the index to represent the unique count
                } else {
                    return Err(MismatchedSizes);
                }
                Ok(unique_idx)
            }

            /// Writes the prime factors in `pfbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns the number of factors found.
            ///
            /// # Errors
            /// Returns `MismatchedSizes` if the total number of factors
            /// is greater than the length of the `buffer`.
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
    (@unsigned $t:ty : $cap:literal : $d:literal) => { paste! {
        #[doc = crate::code::doc_availability!(feature = $cap)]
        ///
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
        #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
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
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors(self) -> Vec<$t> {
                let n = self.0;
                iif![n == 0; return vec![]; iif![n == 1; return vec![1]]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
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
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_proper(self) -> Vec<$t> {
                let n = self.0;
                iif![n == 0; return vec![]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
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
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_prime(self) -> Vec<$t> {
                let mut factors = Vec::new();
                let mut n = self.0;
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

            /// Returns the unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_prime_unique(self) -> Vec<$t> {
                self.factors_prime().also_mut(|v| v.dedup())
            }

            /// Returns the unique prime factors with its exponent.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique_exp(), vec![(2, 3), (3, 1)]];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_unique_exp(), vec![]];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_unique_exp(), vec![]];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_unique_exp(), vec![(7, 1)]];"]
            /// ```
            #[inline] #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
            pub fn factors_prime_unique_exp(self) -> Vec<($t, u32)> {
                let mut factors = Vec::new();
                let mut current = None;
                let mut count = 0;

                for prime in self.factors_prime() {
                    match current {
                        Some(f) if f == prime => {
                            count += 1;
                        },
                        _ => {
                            if let Some(f) = current {
                                factors.push((f, count));
                            }
                            current = Some(prime);
                            count = 1;
                        },
                    }
                }
                if let Some(f) = current {
                    factors.push((f, count));
                }
                factors
            }

            /* unsigned factors non_alloc */

            /// Returns the count of prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_count(), 4];"]
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_count(), 0];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_count(), 0];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_count(), 1];"]
            /// ```
            #[inline] #[must_use]
            pub fn factors_prime_count(self) -> usize {
                let mut n = self.0;
                iif![n == 0; return 0];
                let mut count = 0;
                // Divide by 2 until the number is odd
                while n % 2 == 0 {
                    count += 1;
                    n /= 2;
                }
                // Divide by odd numbers starting from 3
                let mut i = 3;
                while i * i <= n {
                    while n % i == 0 {
                        count += 1;
                        n /= i;
                    }
                    i += 2;
                }
                // If the remaining number is greater than 2, it's a prime factor
                iif![n > 2; count += 1];
                count
            }

            /// Returns the count of unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_unique_count(), 2];"]
            /// ```
            #[inline] #[must_use]
            pub fn factors_prime_unique_count(self) -> usize {
                let mut n = self.0;
                iif![n == 0; return 0];
                let mut count = 0;
                let mut last = 0;

                // Divide by 2 until the number is odd
                while n % 2 == 0 {
                    iif![last != 2; { count += 1; last = 2 }];
                    n /= 2;
                }
                // Divide by odd numbers starting from 3
                let mut i = 3;
                while i * i <= n {
                    while n % i == 0 {
                        iif![last != i; { count += 1; last = i }];
                        n /= i;
                    }
                    i += 2;
                }
                // If the remaining number is greater than 2,
                // and not the same as the last factor, it's a prime factor
                if n > 2 && last != n {
                    count += 1;
                }
                count
            }

            /// Writes the factors in `fbuf`, and the unique prime factors in `upfbuf`.
            ///
            /// Returns a tuple with the number of factors and unique prime factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors
            /// is greater than the length of any buffer.
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
                let n = self.0;
                iif![n == 0; return Ok((0, 0))];
                iif![n == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                fbuf[f_count] = 1;
                f_count += 1;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..=n {
                    if n % i == 0 {
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
            /// Returns a tuple with the number of factors and unique prime factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors
            /// is greater than the length of any buffer.
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
                let n = self.0;
                iif![n == 0; return Ok((0, 0))];
                iif![n == 1; { fbuf[0] = 1; return Ok((1, 0)); }];
                let mut f_count = 0;
                let prime_factors_count = self.factors_prime_unique_buf(upfbuf)?;
                for i in 2..n {
                    if n % i == 0 {
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
            /// Returns the number of factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors
            /// is greater than the length of the `buffer`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut buf = [0; 5];
            #[doc = "assert_eq![Int(24_" $t ").factors_prime_buf(&mut buf), Ok(4)];"]
            ///
            /// assert_eq![buf[..4], [2, 2, 2, 3]];
            #[doc = "assert![Int(24_" $t " * 4).factors_prime_buf(&mut buf).is_err()];"]
            /// assert_eq![buf, [2, 2, 2, 2, 2]]; // the factor of 3 didn't fit
            ///
            #[doc = "assert_eq![Int(0_" $t ").factors_prime_buf(&mut buf), Ok(0)];"]
            #[doc = "assert_eq![Int(1_" $t ").factors_prime_buf(&mut buf), Ok(0)];"]
            #[doc = "assert_eq![Int(7_" $t ").factors_prime_buf(&mut buf), Ok(1)];"]
            /// assert_eq![buf[..1], [7]];
            /// ```
            #[inline]
            pub fn factors_prime_buf(self, buffer: &mut [$t]) -> Result<usize> {
                let n = self.0;
                iif![n == 0; return Ok(0)];
                let (mut n, mut idx) = (n, 0);
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
            /// The `buffer` must be large enough to hold all the non-unique factors of `n`.
            /// In that case the function will return the number of unique factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] otherwise. In that case the buffer
            /// will only contain the non-unique factors that could fit, same as
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

            /// Writes the unique prime factors in the given `fbuffer`, and the
            /// associated exponent in the given `ebuffer` at the same index.
            ///
            /// The `fbuffer` must be large enough to hold all the non-unique factors of `n`.
            /// In that case the function will return the number of unique factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] otherwise. In that case the buffer
            /// will only contain the non-unique factors that could fit, same as
            #[doc = "[`factors_prime_buf`](#method.factors_prime_buf" $d ")."]
            ///
            /// Returns [`MismatchedSizes`] if `ebuffer` is not large enough as well.
            /// In that case the number of unique factors will equal `ebuffer.len()`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let mut fbuf = [0; 4];
            /// let mut ebuf = [0; 2];
            #[doc = "assert_eq![Int(40_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(2)];"]
            /// assert_eq![fbuf[..2], [2, 5]]; // 2^3, 5^1, …
            /// assert_eq![ebuf[..2], [3, 1]];
            ///
            #[doc = "assert_eq![Int(0_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(0)];"]
            #[doc = "assert_eq![Int(1_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(0)];"]
            #[doc = "assert_eq![Int(7_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf), Ok(1)];"]
            /// assert_eq![fbuf[..1], [7]]; // 7^1
            /// assert_eq![ebuf[..1], [1]];
            ///
            /// // When `fbuffer` is not large enough:
            /// let mut fbuf = [0; 3];
            /// let mut ebuf = [0; 2];
            #[doc = "assert![Int(24_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf).is_err()];"]
            /// assert_eq![fbuf, [2, 2, 2]]; // the factor of 5 didn't fit
            /// assert_eq![ebuf, [0, 0]]; // the exponents didn't get written
            ///
            /// // When `ebuffer` is not large enough:
            /// let mut fbuf = [0; 4];
            /// let mut ebuf = [0; 1];
            #[doc = "assert![Int(24_" $t
                ").factors_prime_unique_exp_buf(&mut fbuf, &mut ebuf).is_err()];"]
            /// assert_eq![fbuf[..ebuf.len()], [2]]; // 2^3, Err, …
            /// assert_eq![ebuf[..], [3]];
            /// ```
            // IMPROVE: differenciate between both errors more clearly.
            pub fn factors_prime_unique_exp_buf(self, fbuffer: &mut [$t], ebuffer: &mut [u32])
            -> Result<usize> {
                let prime_factors_count = self.factors_prime_buf(fbuffer)?;
                iif![prime_factors_count == 0; return Ok(0)];

                let mut current_factor = fbuffer[0]; // current factor
                let mut unique_idx = 0; // current unique factor index
                let mut exp_count = 1; //

                for i in 1..prime_factors_count {
                    // Same factor as before, increment the exponent count
                    if fbuffer[i] == current_factor {
                        exp_count += 1;
                    } else {
                        // New factor found, store the previous factor and its exp_count
                        fbuffer[unique_idx] = current_factor;
                        iif![unique_idx >= ebuffer.len(); return Err(MismatchedSizes)];
                        ebuffer[unique_idx] = exp_count;
                        unique_idx += 1; // Move to the next unique factor

                        // Reset for the new factor
                        current_factor = fbuffer[i];
                        exp_count = 1;
                    }
                }
                // Store the last factor and its exponent count
                if unique_idx < fbuffer.len() && unique_idx < ebuffer.len() {
                    fbuffer[unique_idx] = current_factor;
                    ebuffer[unique_idx] = exp_count;
                    unique_idx += 1; // increment the index to represent the unique count
                } else {
                    return Err(MismatchedSizes);
                }
                Ok(unique_idx)
            }

            /// Writes the prime factors in `pfbuf`, and the unique factors in `upfbuf`.
            ///
            /// Returns the number of factors found.
            ///
            /// # Errors
            /// Returns [`MismatchedSizes`] if the total number of factors
            /// is greater than the length of the `buffer`.
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
impl_int!();
