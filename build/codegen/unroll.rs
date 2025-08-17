// devela::build::codegen::unroll
//
//! Code generator for the [`unroll!`] macro.
//
// TOC
// - `unroll!` macro definition.
// - tests.

#![allow(clippy::uninlined_format_args, reason = "consistency")]

use super::super::utils::*;
use std::{
    fs::{File, create_dir_all},
    io::{BufWriter, Error, Write},
    writeln as w,
};

const LOWER_LIMIT: usize = 16;

#[rustfmt::skip] const MAX_RECURSION: usize = {
    if cfg!(not(feature = "_unroll_128")) { 64
    } else if cfg!(all(feature = "_unroll_128", not(feature = "_unroll_256"))) { 128
    } else if cfg!(all(feature = "_unroll_256", not(feature = "_unroll_512"))) { 256
    } else if cfg!(all(feature = "_unroll_512", not(feature = "_unroll_1024"))) { 512
    } else if cfg!(all(feature = "_unroll_1024", not(feature = "_unroll_2048"))) { 1024
    } else { 2048 }
};

#[rustfmt::skip]
pub(crate) fn generate() -> Result<(), Error> {
    let build_out_dir = out_dir().join("build/");
    create_dir_all(&build_out_dir)?;
    let path = build_out_dir.join("unroll.rs");

    // the generated file will be imported from /src/code/util/unroll/mod.rs
    #[cfg(feature = "__dbg")]
    println(&format!("generated: {}", path.display()));

    let file = File::create(path)?;
    let mut f = BufWriter::new(file);
    // let mut f = BufWriter::with_capacity(100 * 1024, file);

    let macro_code1 = r#"/// Unrolls the given for loop.
///
/// # Example
/// ```ignore
/// unroll! {
///   for i in 0..5 {
///     println!("Iteration {}", i);
///   }
/// }
/// ```
///
/// will expand into:
/// ```ignore
/// { println!("Iteration {}", 0); }
/// { println!("Iteration {}", 1); }
/// { println!("Iteration {}", 2); }
/// { println!("Iteration {}", 3); }
/// { println!("Iteration {}", 4); }
/// ```
///
/// # Features
/// By default it's implemented for a maximum recusion of 64 iterations.
/// It supports increased limits of 128, 256, 512, 1024 and 2048 by enabling the
/// corresponding capability feature: `_unroll_[128|256|512|1024|2048]`.
///
/// # Vendored
/// This is adapted work from [crunchy][crate::_info::vendored#crunchy]"#;
// In sync with code::utils::_doc::doc_!(vendor:)
    w!(f, "{macro_code1}")?;
    let macro_code2 = r#"#[doc(hidden)]
#[macro_export]
macro_rules! _unroll {
    (
    // Base case for ranges with no iterations.
    for $v:ident in 0..0 $c:block) => {};
    (
    // Handles ranges with a step value.
    for $v:ident < $max:tt in ($start:tt..$end:tt).step_by($val:expr) {$($c:tt)*}) => {
        // Expands the loop by calculating the stepped range and recursively unrolling.
        {
            let step = $val;
            let start = $start;
            let end = start + ($end - start) / step;
            $crate::unroll! {
                for val < $max in start..end {
                    let $v: usize = ((val.wrapping_sub(start)) * step) + start;
                    $($c)*
                }
            }
        }
    };
    (
    // Redirects stepped ranges.
    for $v:ident in ($start:tt..$end:tt).step_by($val:expr) {$($c:tt)*}) => {
        $crate::unroll! {
            for $v < $end in ($start..$end).step_by($val) {$($c)*}
        }
    };
    (
    // Simplifies parentheses in ranges.
    for $v:ident in ($start:tt..$end:tt) {$($c:tt)*}) => {
        $crate::unroll!{ for $v in $start..$end {$($c)*} }
    };
    (
    // Main handler for unrolling a range.
    for $v:ident in $start:tt..$end:tt {$($c:tt)*}) => {
        // Calls an internal recursive macro to expand the loop.
        #[allow(non_upper_case_globals)]
        #[allow(unused_comparisons)]
        { $crate::unroll![@$v, 0, $end, { if $v >= $start {$($c)*} }]; }
    };
    (
    // Validates the range and redirects to internal recursive unrolling with bounds checking.
    for $v:ident < $max:tt in $start:tt..$end:tt $c:block) => {
        #[allow(non_upper_case_globals)]
        {
            let range = $start..$end;
            assert!($max >= range.end, "`{0}` out of range `{1:?}`", stringify!($max), range,);
            $crate::unroll![@$v, 0, $max, { if $v >= range.start && $v < range.end { $c } }];
        }
    };
    (
    // Special case for ranges starting at zero.
    for $v:ident in 0..$end:tt {$($statement:tt)*}) => {
        // Calls the internal recursive unrolling macro.
        #[allow(non_upper_case_globals)]
        { $crate::unroll![@$v, 0, $end, {$($statement)*}]; }
    };
    (
    /* private, recursive unrolling cases */
     @$v:ident, $a:expr, 0, $c:block) => {
        { const $v: usize = $a; $c }
    };
"#;
    w!(f, "{macro_code2}")?;

    for i in 1..MAX_RECURSION + 1 {
        w!(f, "    (@$v:ident, $a:expr, {0}, $c:block) => {{", i)?;
        if i <= LOWER_LIMIT {
            w!(f, "        {{ const $v: usize = $a; $c }}")?;
            for a in 1..i {
                w!(f, "        {{ const $v: usize = $a + {0}; $c }}", a)?;
            }
        } else {
            let half = i / 2;

            if i.is_multiple_of(2) {
                w!(f, "        $crate::unroll![@$v, $a, {0}, $c];", half)?;
                w!(f, "        $crate::unroll![@$v, $a + {0}, {0}, $c];", half)?;
            } else {
                if half > 1 {
                    w!(f, "        $crate::unroll![@$v, $a, {0}, $c];", i - 1)?;
                }
                w!(f, "        {{ const $v: usize = $a + {0}; $c }}", i - 1)?;
            }
        }
        w!(f, "    }};")?;
    }
    w!(f, "}}\n#[doc(inline)]\npub use _unroll as unroll;")?;

    /* tests */

    let tests_code1 = r#"
#[cfg(all(test, feature = "alloc"))]
mod tests {
    use crate::{unroll, vec_ as vec, Vec};

    #[test]
    fn invalid_range() {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in (5..4) {
                a.push(i);
            }
        }
        assert!(a.is_empty());
    }

    #[test]
    fn start_at_one_with_step() {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in (2..4).step_by(1) {
                a.push(i);
            }
        }
        assert_eq!(a, vec![2, 3]);
    }

    #[test]
    fn start_at_one() {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in 1..4 {
                a.push(i);
            }
        }
        assert_eq!(a, vec![1, 2, 3]);
    }

    #[test]
    fn test_all() {
        {
            let a: Vec<usize> = vec![];
            unroll! {
                for i in 0..0 {
                    a.push(i);
                }
            }
            assert_eq!(a, (0..0).collect::<Vec<usize>>());
        }
        {
            let mut a: Vec<usize> = vec![];
            unroll! {
                for i in 0..1 {
                    a.push(i);
                }
            }
            assert_eq!(a, (0..1).collect::<Vec<usize>>());
        }"#;
    w!(f, "{0}", tests_code1)?;

    w!(f, r#"
        {{
            let mut a: Vec<usize> = vec![];
            unroll! {{
                for i in 0..{0} {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (0..{0}).collect::<Vec<usize>>());
        }}
        {{
            let mut a: Vec<usize> = vec![];
            let start = {0} / 4;
            let end = start * 3;
            unroll! {{
                for i < {0} in start..end {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (start..end).collect::<Vec<usize>>());
        }}
        {{
            let mut a: Vec<usize> = vec![];
            unroll! {{
                for i in (0..{0}).step_by(2) {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (0..{0} / 2).map(|x| x * 2).collect::<Vec<usize>>());
        }}
        {{
            let mut a: Vec<usize> = vec![];
            let start = {0} / 4;
            let end = start * 3;
            unroll! {{
                for i < {0} in (start..end).step_by(2) {{
                    a.push(i);
                }}
            }}
            assert_eq!(a, (start..end).filter(|x| x % 2 == 0).collect::<Vec<usize>>());
        }}
    }}
}}"#, MAX_RECURSION)?;

    // --------------------------------------------------------------------------

    if let Err(e) = f.flush() {
        eprintln!("Failed to write to file: {0}", e);
        std::process::exit(1);
    }

    // #[cfg(doc)] // format the source if we're building the docs
    // super::super::rustfmt_file(path);
    Ok(())
}
