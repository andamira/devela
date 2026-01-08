// devela_base_core::code::ops::punroll
//
//! Defines the [`punroll!`] macro.
//

#[doc = crate::_tags!(code)]
/// Compile-time pattern unroll over a width of `0..=8 || pow(2) <= 64`.
#[doc = crate::_doc_location!("core/ops")]
///
/// Three forms are supported:
/// - `punroll![N |i| stmt]`    Emits `stmt` once per index, with `i` bound each time.
/// - `punroll![N [] |i| expr]` Produces an array `[expr(i0), expr(i1), …]`.
/// - `punroll![N () |i| expr]` Produces a tuple `(expr(i0), expr(i1), …)`.
///
/// Here `i` is always a `usize`, and `N` is one of the built-in widths:
/// `0, 1, 2, 3, 4, 5, 6, 7, 8, 16, 32, 64`.
///
/// This macro is a low-level building block for small, fixed-width compile-time
/// expansions: vector-like patterns, lane-style evaluation, and other unrolled
/// constant-time constructions without loop codegen.
//
// # NOTES
// For macro hygiene reasons it's not possible to generate partial contents of an array or tuple,
// not even from a separate arm of the same macro, so each one has to be constructed in one step.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! punroll {
    (0 |$i:ident| $stmt:stmt) => {};
    (0 [] |$i:ident| $expr:expr) => { [] };
    (0 () |$i:ident| $expr:expr) => { () };
    (1 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0] } };
    (1 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0] } };
    (1 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0] } };
    (2 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0, 1] } };
    (2 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0, 1] } };
    (2 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0, 1] } };
    (3 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0, 1, 2] } };
    (3 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0, 1, 2] } };
    (3 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0, 1, 2] } };
    (4 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0, 1, 2, 3] } };
    (4 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0, 1, 2, 3] } };
    (4 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0, 1, 2, 3] } };
    (5 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0, 1, 2, 3, 4] } };
    (5 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0, 1, 2, 3, 4] } };
    (5 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0, 1, 2, 3, 4] } };
    (6 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0, 1, 2, 3, 4, 5] } };
    (6 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0, 1, 2, 3, 4, 5] } };
    (6 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0, 1, 2, 3, 4, 5] } };
    (7 |$i:ident| $stmt:stmt) => { $crate::punroll! { # |$i| $stmt; [0, 1, 2, 3, 4, 5, 6] } };
    (7 [] |$i:ident| $expr:expr) => { $crate::punroll! { # [] |$i| $expr; [0, 1, 2, 3, 4, 5, 6] } };
    (7 () |$i:ident| $expr:expr) => { $crate::punroll! { # () |$i| $expr; [0, 1, 2, 3, 4, 5, 6] } };
    (8 |$i:ident| $stmt:stmt) => {
        $crate::punroll! { # |$i| $stmt; [0, 1, 2, 3, 4, 5, 6, 7] }
    };
    (8 [] |$i:ident| $expr:expr) => {
        $crate::punroll! { # [] |$i| $expr; [0, 1, 2, 3, 4, 5, 6, 7] }
    };
    (8 () |$i:ident| $expr:expr) => {
        $crate::punroll! { # () |$i| $expr; [0, 1, 2, 3, 4, 5, 6, 7] }
    };
    (16 |$i:ident| $stmt:stmt) => {
        $crate::punroll! { # |$i| $stmt;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        }
    };
    (16 [] |$i:ident| $expr:expr) => {
        $crate::punroll! { # [] |$i| $expr;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        }
    };
    (16 () |$i:ident| $expr:expr) => {
        $crate::punroll! { # () |$i| $expr;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        }
    };
    (32 |$i:ident| $stmt:stmt) => {
        $crate::punroll! { # |$i| $stmt;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
        }
    };
    (32 [] |$i:ident| $expr:expr) => {
        $crate::punroll! { # [] |$i| $expr;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
        }
    };
    (32 () |$i:ident| $expr:expr) => {
        $crate::punroll! { # () |$i| $expr;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]
        }
    };
    (64 |$i:ident| $stmt:stmt) => {
        $crate::punroll! { # |$i| $stmt;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63]
        }
    };
    (64 [] |$i:ident| $expr:expr) => {
        $crate::punroll! { # [] |$i| $expr;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63]
        }
    };
    (64 () |$i:ident| $expr:expr) => {
        $crate::punroll! { # () |$i| $expr;
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63]
        }
    };

    /* internal expansion routines */

    // statements over an index list
    (# |$i:ident| $stmt:stmt; [$($idx:literal),+]) => {{
        $( { let $i: usize = $idx; $stmt } )+
    }};
    // array of expressions over an index list
    (# [] |$i:ident| $expr:expr; [$($idx:literal),+]) => {
        [ $( { let $i: usize = $idx; $expr } ),+ ]
    };
    // tuple of expressions over an index list
    (# () |$i:ident| $expr:expr; [$($idx:literal),+]) => {
        ( $( { let $i: usize = $idx; $expr } ),+ )
    };
}
#[doc(inline)]
pub use punroll;

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    #[test] fn statements() {
        let a=0; punroll![0 |i|a+=i]; assert_eq!(a, 0); //
        let mut a=0; punroll![1 |i|a+=i]; assert_eq!(a, 0+0);
        let mut a=0; punroll![2 |i|a+=i]; assert_eq!(a, 0+1);
        let mut a=0; punroll![3 |i|a+=i]; assert_eq!(a, 0+1+2);
        let mut a=0; punroll![4 |i|a+=i]; assert_eq!(a, 0+1+2+3);
        let mut a=0; punroll![5 |i|a+=i]; assert_eq!(a, 0+1+2+3+4);
    }
    #[test] fn array() {
        let a: [i32; 0] = punroll![0[] |i|i*2]; assert_eq!(a, []); //
        let a = punroll![1[] |i|i*2]; assert_eq!(a, [0]);
        let a = punroll![2[] |i|i*2]; assert_eq!(a, [0,2]);
        let a = punroll![3[] |i|i*2]; assert_eq!(a, [0,2,4]);
        let a = punroll![4[] |i|i*2]; assert_eq!(a, [0,2,4,6]);
        let a = punroll![5[] |i|i*2]; assert_eq!(a, [0,2,4,6,8]);
    }
    #[test] fn tuple() {
        let a = punroll![0() |i|i*i]; assert_eq!(a, ()); //
        let a = punroll![1() |i|i*i]; assert_eq!(a, (0));
        let a = punroll![2() |i|i*i]; assert_eq!(a, (0,1));
        let a = punroll![3() |i|i*i]; assert_eq!(a, (0,1,4));
        let a = punroll![4() |i|i*i]; assert_eq!(a, (0,1,4,9));
        let a = punroll![5() |i|i*i]; assert_eq!(a, (0,1,4,9,16));
    }
}
