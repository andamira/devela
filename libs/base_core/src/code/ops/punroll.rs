// devela_base_core::code::ops::punroll
//
//! Defines the [`punroll!`] macro.
//

/// Compile-time pattern unroll over a fixed power-of-two width.
///
/// Three forms are supported:
/// - `punroll![N |i| stmt]`
///   Emits `stmt` once per index, with `i` bound each time.
/// - `punroll![N [] |i| expr]`
///   Produces an array `[expr(i0), expr(i1), …]`.
/// - `punroll![N () |i| expr]`
///   Produces a tuple `(expr(i0), expr(i1), …)`.
///
/// Here `i` is always a `usize`, and `N` is one of the built-in widths:
/// `2, 4, 8, 16, 32, 64`.
///
/// This macro is a low-level building block for small, fixed-width compile-time
/// expansions: vector-like patterns, lane-style evaluation, and other unrolled
/// constant-time constructions without loop codegen.
#[macro_export]
macro_rules! punroll {
    (2 |$i:ident| $stmt:stmt) => {
        $crate::punroll! { # |$i| $stmt; [0, 1] }
    };
    (2 [] |$i:ident| $expr:expr) => {
        $crate::punroll! { # [] |$i| $expr; [0, 1] }
    };
    (2 () |$i:ident| $expr:expr) => {
        $crate::punroll! { # () |$i| $expr; [0, 1] }
    };

    (4 |$i:ident| $stmt:stmt) => {
        $crate::punroll! { # |$i| $stmt; [0, 1, 2, 3] }
    };
    (4 [] |$i:ident| $expr:expr) => {
        $crate::punroll! { # [] |$i| $expr; [0, 1, 2, 3] }
    };
    (4 () |$i:ident| $expr:expr) => {
        $crate::punroll! { # () |$i| $expr; [0, 1, 2, 3] }
    };
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
pub use punroll;

#[cfg(test)]
mod tests {
    #[test]
    fn punroll_statements() {
        let mut acc = 0;
        punroll![4 | i | acc += i];
        assert_eq!(acc, 0 + 1 + 2 + 3);
    }

    #[test]
    fn punroll_array() {
        let a = punroll![4 [] |i| i * 2];
        assert_eq!(a, [0, 2, 4, 6]);
    }

    #[test]
    fn punroll_tuple() {
        let t = punroll![2() | i | i + 10];
        assert_eq!(t, (10, 11));
    }
}
