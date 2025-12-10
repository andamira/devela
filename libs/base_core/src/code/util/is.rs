// devela_base_core::code::util::is
//
//! Defines inline if macro [`is!`].
//

/// Conditional evaluation.
///
/// Combines:
/// 1. `if`/`else` conditions
/// 2. `if let` pattern matching
///
/// # Examples
///
/// 1. Replacing `if`:
/// ```
/// # use devela_base_core::is;
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
/// # use devela_base_core::is;
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
/// # use devela_base_core::is;
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
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! is {
    ($cond:expr ; $then:expr) => {
        #[allow(clippy::question_mark, reason = "to remain const-friendly")]
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
}
#[doc(inline)]
pub use is;

#[cfg(test)]
mod test_is {
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
