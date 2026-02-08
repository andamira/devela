// devela_base_core::data::sort::impl_primitives
//
//! Implements compile-time sorting algorithms for arrays of primitives.
//

use crate::{Cmp, Sort, cswap, is, paste, whilst};

/* impl Sort on primitives */

macro_rules! impl_sort {
    () => {
        impl_sort![signed i8, i16, i32, i64, i128, isize];
        impl_sort![unsigned u8, u16, u32, u64, u128, usize];
        impl_sort![float f32, f64];
    };

    // $t: the input/output primitive type
    (signed   $( $t:ty ),+) => { $( impl_sort![@signed $t]; )+ };
    (unsigned $( $t:ty ),+) => { $( impl_sort![@unsigned $t]; )+ };
    (float    $( $t:ty ),+) => { $( impl_sort![@float $t]; )+ };

    (@signed $t:ty) => { paste! {
        /// Implement const sorting methods for arrays of primitives.
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[must_use]
            pub const fn bubble_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 0..N; {
                    whilst![j in 0..N-i-1; {
                        is![arr[j] > arr[j+1]; cswap!(xor arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn insertion_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 1..N; {
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
            pub const fn selection_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 0..N-1; {
                    let mut min_index = i;
                    whilst![j in (i+1),..N; {
                        is![arr[j] < arr[min_index]; min_index = j];
                    }];
                    cswap!(xor arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};

    (@unsigned $t:ty) => { paste! {
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[must_use]
            pub const fn bubble_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 0..N; {
                    whilst![j in 0..N-i-1; {
                        is![arr[j] > arr[j+1]; cswap!(xor: arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn insertion_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 1..N; {
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
            pub const fn selection_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 0..N-1; {
                    let mut min_index = i;
                    whilst![j in (i+1),..N; {
                        is![arr[j] < arr[min_index]; min_index = j];
                    }];
                    cswap!(xor: arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};

    (@float $t:ty) => { paste! {
        impl<const N: usize> Sort<[$t; N]> {
            /// Returns a copied sorted array using bubble sort.
            #[must_use]
            pub const fn bubble_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 0..N; {
                    whilst![j in 0..N-i-1; {
                        is![Cmp(arr[j]).gt(arr[j+1]); cswap!(tmp: arr[j], arr[j+1])];
                    }];
                }];
                arr
            }

            /// Returns a copied sorted array using insertion sort.
            #[must_use]
            pub const fn insertion_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 1..N; {
                    let mut j = i;
                    while j > 0 && Cmp(arr[j-1]).gt(arr[j]) {
                        cswap!(tmp: arr[j], arr[j-1]);
                        j -= 1;
                    }
                }];
                arr
            }

            /// Returns a copied sorted array using selection sort.
            #[must_use]
            pub const fn selection_array(&mut self) -> [$t; N] {
                let mut arr = self.0;
                whilst![i in 0..N-1; {
                    let mut min_index = i;
                    whilst![j in (i+1),..N; {
                        is![Cmp(arr[j]).lt(arr[min_index]); min_index = j];
                    }];
                    cswap!(tmp: arr[min_index], arr[i]);
                }];
                arr
            }
        }
    }};
}
impl_sort![];
