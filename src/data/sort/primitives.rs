// devela::data::sort::impl_primitives
//
//! Implements compile-time sorting algorithms for arrays of primitives.
//

#[cfg(_sort_float··)]
use crate::Compare;
use crate::{Sort, cfor, cswap, is, paste};

/* impl Sort on primitives */

#[cfg(_sort··)]
macro_rules! impl_sort {
    () => {
        impl_sort![signed
            i8:"_sort_i8",
            i16:"_sort_i16",
            i32:"_sort_i32",
            i64:"_sort_i64",
            i128:"_sort_i128",
            isize:"_sort_isize"
        ];
        impl_sort![unsigned
            u8:"_sort_u8",
            u16:"_sort_u16",
            u32:"_sort_u32",
            u64:"_sort_u64",
            u128:"_sort_u128",
            usize:"_sort_usize"
        ];
        impl_sort![float f32:"_sort_f32", f64:"_sort_f64"];
    };

    // $t: the input/output primitive type
    (signed   $( $t:ty : $cap:literal ),+) => { $( impl_sort![@signed $t:$cap]; )+ };
    (unsigned $( $t:ty : $cap:literal ),+) => { $( impl_sort![@unsigned $t:$cap]; )+ };
    (float    $( $t:ty : $cap:literal ),+) => { $( impl_sort![@float $t:$cap]; )+ };

    (@signed $t:ty : $cap:literal) => { paste! {
        /// Implement const sorting methods for arrays of primitives.
        #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap)]
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[must_use]
            pub const fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        is![arr[j] > arr[j+1]; cswap!(xor arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && arr[j-1] > arr[j] {
                        cswap!(xor arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        is![arr[j] < arr[min_index]; min_index = j];
                    }];
                    cswap!(xor arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};

    (@unsigned $t:ty : $cap:literal) => { paste! {
        #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap)]
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[must_use]
            pub const fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        is![arr[j] > arr[j+1]; cswap!(xor: arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && arr[j-1] > arr[j] {
                        cswap!(xor: arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using selection sort.
            #[must_use]
            pub const fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        is![arr[j] < arr[min_index]; min_index = j];
                    }];
                    cswap!(xor: arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};

    (@float $t:ty : $cap:literal) => { paste! {
        #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        #[cfg(feature = $cap)]
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[must_use]
            pub const fn bubble_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N => {
                    cfor![j in 0..N-i-1 => {
                        is![Compare(arr[j]).gt(arr[j+1]); cswap!(tmp: arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn insertion_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 1..N => {
                    let mut j = i;
                    while j > 0 && Compare(arr[j-1]).gt(arr[j]) {
                        cswap!(tmp: arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using selection sort.
            #[must_use]
            pub const fn selection_array(self) -> [$t; N] {
                let mut arr = self.0;
                cfor![i in 0..N-1 => {
                    let mut min_index = i;
                    cfor![j in (i+1)..N => {
                        is![Compare(arr[j]).lt(arr[min_index]); min_index = j];
                    }];
                    cswap!(tmp: arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};
}
#[cfg(_sort··)]
impl_sort![];
