// devela::code::util::is
//
//! inline if macro.
//

/// Conditional evaluation.
///
/// Combines:
/// 1. `if`/`else` conditions
/// 2. `if let` pattern matching
/// 3. Temporary value binding
///
/// # Examples
///
/// 1. Replacing `if`:
/// ```
/// # use devela::is;
/// is![true; print!("true")];
///
/// // This
/// let s = is![1 > 0; true; false];
///
/// // Would be equivalent to
/// let s = if 1 > 0 {
///     true
/// } else {
///     false
/// };
/// ```
///
/// 2. Replacing `if let`:
/// ```
/// # use devela::is;
/// let num = Some(123);
///
/// // This
/// is![let Some(n) = num ; println!("num:{n}") ; { dbg![num]; }];
///
/// // Would be equivalent to
/// if let Some(n) = num {
///     println!("num:{n}")
/// } else {
///     dbg![num];
/// }
/// ```
///
/// Nested:
/// ```
/// # use devela::is;
/// let mut s = String::new();
/// let is_premium = Some(true);
///
/// // This
/// is![let Some(b) = is_premium; is![b; s += " [premium]"]];
///
/// // Would be equivalent to
/// if let Some(b) = is_premium {
///     if b {
///         s += " [premium]";
///     }
/// }
/// ```
///
/// 3. Temporary value binding:
/// ```
/// # use devela::{format_args, is, FmtWrite};
/// let mut s = String::new();
/// let (a, b) = (1, 2);
///
/// // This
/// is![
///     tmp A = format_args!("A{a}");
///     tmp B = format_args!("B{b}");
///     write!(s, "{A}+{B},");
/// ];
/// assert_eq![&s, "A1+B2,"];
///
/// // Would be equivalent to
/// match format_args!("A{a}") {
///     A => match format_args!("B{b}") {
///         B => { write!(s, "{A}+{B},"); }
///     }
/// }
/// ```
///
/// Otherwise it fails with `E0716: temporary value dropped while borrowed` (in stable):
// WAIT:1.89 [Allow storing format_args! in variable](https://github.com/rust-lang/rust/pull/140748)
// ```compile_fail,E0716
/// ```ignore
/// # use devela::{format_args, FmtWrite};
/// let mut s = String::new();
/// let (a, b) = (1, 2);
///
/// let A = format_args!("A{a}"); // ← freed here
/// let B = format_args!("B{b}"); // ← freed here
/// write!(s, "{A}+{B},");
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! is {
    ($cond:expr ; $then:expr) => {
        if $cond {
            $then
        }
    };
    ($cond:expr ; $then:expr ; $($else:expr)?) => {
        // WAIT: [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701)
        // #[allow(clippy::redundant_else)]
        if $cond {
            $then
        } else {
            $( $else )?
        }
    };

    (let $pat:pat = $cond:expr ; $then:expr) => {
        #[allow(clippy::question_mark)]
        if let $pat = $cond {
            $then
        }
    };
    (let $pat:pat = $cond:expr ; $then:expr ; $($else:expr)? ) => {
        if let $pat = $cond {
            $then
        } else {
            $( $else )?
        }
    };

    // Temporary value binding helper that:
    // 1. Binds short-lived expressions to names
    // 3. Enables expression chaining while maintaining temporary lifetimes
    //
    // source: https://github.com/rust-lang/rust/issues/92698#issuecomment-1680155957
    (tmp $name:pat = $val:expr; $($rest:tt)+) => {
        match $val { $name => $crate::is!($($rest)*) }
    };
    ($($rest:tt)+) => {{ $($rest)+ }}
}
#[doc(inline)]
pub use is;

/// Renamed to [`is`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[deprecated(since = "0.23.0", note = "Use the 'is!' macro instead.")]
macro_rules! iif { ($($tt:tt)*) => { $crate::is![$($tt)*] }; }
#[allow(deprecated, reason = "re-exported")]
#[doc(inline)]
pub use iif;

#[cfg(test)]
mod test_is {
    use crate::is;

    #[test]
    fn is_if() {
        assert_eq!('a', is!(true ; 'a' ; 'b'));
        assert_eq!('b', is!(false ; 'a' ; 'b'));
    }

    #[test]
    fn is_let() {
        let somea = Some('a');
        let noa: Option<char> = None;

        assert_eq!('a', is!(let Some(a) = somea; a ; 'b'));
        assert_eq!('b', is!(let Some(a) = noa; a ; 'b'));
    }
}
