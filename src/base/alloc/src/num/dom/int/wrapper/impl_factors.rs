// devela_base_alloc::num::dom::int::wrapper::impl_factors
//
//! Implements factors-related allocating methods for [`IntAlloc`].
//
// TOC
// - signed|unsigned:
//   - allocating:
//     - factors
//     - factors_proper
//     - factors_prime
//     - factors_prime_unique
//     - factors_prime_unique_exp
//
// TODO: rename methods with alloc affix.

use crate::{BTreeSet, Hook, IntAlloc, Vec, is, paste, vec_ as vec};

/// Implements factors-related methods for [`IntAlloc`].
///
/// # Args.
/// $t:   the input/output type
///
/// $d:   the doclink suffix for the method name
macro_rules! impl_factors {
    () => {
        impl_factors![signed
            i8    |"",
            i16   |"-1",
            i32   |"-2",
            i64   |"-3",
            i128  |"-4",
            isize |"-5"
        ];
        impl_factors![unsigned
            u8    |"-6",
            u16   |"-7",
            u32   |"-8",
            u64   |"-9",
            u128  |"-10",
            usize |"-11"
        ];
    };
    (signed $( $t:ty | $d:literal ),+) => {
        $( impl_factors![@signed $t|$d]; )+
    };
    (unsigned $( $t:ty | $d:literal ),+) => {
        $( impl_factors![@unsigned $t|$d]; )+
    };
    (
    // implements signed ops
    @signed $t:ty | $d:literal) => { paste! {
        ///
        #[doc = "# Integer factors related methods for `" $t "`\n\n"]
        #[doc = "   - [factors](#method.factors" $d ")"]
        #[doc = "   - [factors_proper](#method.factors_proper" $d ")"]
        #[doc = "   - [factors_prime](#method.factors_prime" $d ")"]
        #[doc = "   - [factors_prime_unique](#method.factors_prime_unique" $d ")"]
        ///
        impl IntAlloc<$t> {
            /* signed factors alloc */

            /// Returns the factors (including 1 and self).
            ///
            /// # Examples
            /// ```
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t
                ").factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
            #[doc = "assert_eq![IntAlloc::new(-24_" $t
                ").factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors().is_empty()];"]
            #[doc = "assert_eq![IntAlloc::new(1_" $t ").factors(), vec![1]];"]
            #[doc = "assert_eq![IntAlloc::new(7_" $t ").factors(), vec![1, 7]];"]
            /// ```
            #[must_use]
            pub fn factors(self) -> Vec<$t> {
                let n = self.0.0.abs();
                is![n == 0; return vec![];
                is![n == 1; return vec![1]]];
                let mut set = BTreeSet::new();
                set.insert(1);
                for p in self.factors_prime_unique() {
                    let temp = set.clone();
                    let mut x = p;
                    while x <= n {
                        for &num in &temp {
                            let new_num = num * x;
                            is!{n % new_num == 0; {set.insert(new_num);} }
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
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t
                ").factors_proper(), vec![2, 3, 4, 6, 8, 12]];"]
            #[doc = "assert_eq![IntAlloc::new(-24_" $t
                ").factors_proper(), vec![2, 3, 4, 6, 8, 12]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors_proper().is_empty()];"]
            #[doc = "assert![IntAlloc::new(1_" $t ").factors_proper().is_empty()];"]

            #[doc = "assert![IntAlloc::new(7_" $t ").factors_proper().is_empty()];"]
            /// ```
            #[must_use]
            pub fn factors_proper(self) -> Vec<$t> {
                let n = self.0.0.abs();
                is![n == 0; return vec![]];
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
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t ").factors_prime(), vec![2, 2, 2, 3]];"]
            #[doc = "assert_eq![IntAlloc::new(-24_" $t ").factors_prime(), vec![2, 2, 2, 3]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors_prime().is_empty()];"]
            #[doc = "assert![IntAlloc::new(1_" $t ").factors_prime().is_empty()];"]
            #[doc = "assert_eq![IntAlloc::new(7_" $t ").factors_prime(), vec![7]];"]
            /// ```
            #[must_use]
            pub fn factors_prime(self) -> Vec<$t> {
                let mut factors = Vec::new();
                let mut n = self.0.0.abs();
                is![n == 0; return factors];

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
                is![n > 2; factors.push(n)];
                factors
            }

            /// Returns the unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            #[doc = "assert_eq![IntAlloc::new(-24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            /// ```
            #[must_use]
            pub fn factors_prime_unique(self) -> Vec<$t> {
                self.factors_prime().hook(|v| v.dedup())
            }

            /// Returns the unique prime factors with its exponent.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_alloc::IntAlloc;
                #[doc = "assert_eq![IntAlloc::new(24_" $t
            ").factors_prime_unique_exp(), vec![(2, 3), (3, 1)]];"]
            #[doc = "assert_eq![IntAlloc::new(-24_" $t
                ").factors_prime_unique_exp(), vec![(2, 3), (3, 1)]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors_prime_unique_exp().is_empty()];"]
            #[doc = "assert![IntAlloc::new(1_" $t ").factors_prime_unique_exp().is_empty()];"]
            #[doc = "assert_eq![IntAlloc::new(7_" $t ").factors_prime_unique_exp(), vec![(7, 1)]];"]
            /// ```
            #[must_use]
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
        }
    }};
    (
    // implements unsigned ops
    @unsigned $t:ty | $d:literal) => { paste! {
        ///
        #[doc = "# Integer factors related methods for `" $t "`\n\n"]
        #[doc = "   - [factors](#method.factors" $d ")"]
        #[doc = "   - [factors_proper](#method.factors_proper" $d ")"]
        #[doc = "   - [factors_prime](#method.factors_prime" $d ")"]
        #[doc = "   - [factors_prime_unique](#method.factors_prime_unique" $d ")"]
        ///
        impl IntAlloc<$t> {
            /* unsigned factors alloc */

            /// Returns the factors (including 1 and self).
            ///
            /// # Examples
            /// ```
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t
                ").factors(), vec![1, 2, 3, 4, 6, 8, 12, 24]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors().is_empty()];"]
            #[doc = "assert_eq![IntAlloc::new(1_" $t ").factors(), vec![1]];"]
            #[doc = "assert_eq![IntAlloc::new(7_" $t ").factors(), vec![1, 7]];"]
            /// ```
            #[must_use]
            pub fn factors(self) -> Vec<$t> {
                let n = self.0.0;
                is![n == 0; return vec![]; is![n == 1; return vec![1]]];
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
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t
                ").factors_proper(), vec![2, 3, 4, 6, 8, 12]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors_proper().is_empty()];"]
            #[doc = "assert![IntAlloc::new(1_" $t ").factors_proper().is_empty()];"]
            #[doc = "assert![IntAlloc::new(7_" $t ").factors_proper().is_empty()];"]
            /// ```
            #[must_use]
            pub fn factors_proper(self) -> Vec<$t> {
                let n = self.0.0;
                is![n == 0; return vec![]];
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
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t ").factors_prime(), vec![2, 2, 2, 3]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors_prime().is_empty()];"]
            #[doc = "assert![IntAlloc::new(1_" $t ").factors_prime().is_empty()];"]
            #[doc = "assert_eq![IntAlloc::new(7_" $t ").factors_prime(), vec![7]];"]
            /// ```
            #[must_use]
            pub fn factors_prime(self) -> Vec<$t> {
                let mut factors = Vec::new();
                let mut n = self.0.0;
                is![n == 0; return factors];

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
                is![n > 2; factors.push(n)];
                factors
            }

            /// Returns the unique prime factors.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t ").factors_prime_unique(), vec![2, 3]];"]
            /// ```
            #[must_use]
            pub fn factors_prime_unique(self) -> Vec<$t> {
                self.factors_prime().hook(|v| v.dedup())
            }

            /// Returns the unique prime factors with its exponent.
            ///
            /// # Examples
            /// ```
            /// # use devela_base_alloc::IntAlloc;
            #[doc = "assert_eq![IntAlloc::new(24_" $t
                ").factors_prime_unique_exp(), vec![(2, 3), (3, 1)]];"]
            #[doc = "assert![IntAlloc::new(0_" $t ").factors_prime_unique_exp().is_empty()];"]
            #[doc = "assert![IntAlloc::new(1_" $t ").factors_prime_unique_exp().is_empty()];"]
            #[doc = "assert_eq![IntAlloc::new(7_" $t ").factors_prime_unique_exp(), vec![(7, 1)]];"]
            /// ```
            #[must_use]
            #[cfg(feature = "alloc")]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
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
        }
    }};
}
impl_factors!();
