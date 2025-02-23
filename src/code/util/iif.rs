// devela::code::util::iif
//
//! inline if macro.
//

/// *`i`nline `if`* macro.
///
/// A inline alternative for `if` and `if let`.
///
/// # Examples
///
/// Replacing `if`:
/// ```
/// # use devela::iif;
/// // This
/// let s = iif![1 > 0; true; false];
///
/// // Would be equivalent to
/// let s = if 1 > 0 {
///     true
/// } else {
///     false
/// };
/// ```
///
/// Replacing `if let`:
/// ```
/// # use devela::iif;
/// let num = Some(123);
///
/// // This
/// iif![let Some(n) = num ; println!("num:{n}") ; { dbg![num]; }];
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
/// # use devela::iif;
/// let mut s = String::new();
/// let is_premium = Some(true);
///
/// // This
/// iif![let Some(b) = is_premium; iif![b; s += " [premium]"]];
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
macro_rules! iif {
    ($if:expr; $true:expr) => {
        if $if {
            $true
        }
    };
    ($if:expr ; $true:expr ; $($false:expr)?) => {
        // WAIT: [stmt_expr_attributes](https://github.com/rust-lang/rust/issues/15701)
        // #[allow(clippy::redundant_else)]
        if $if {
            $true
        } else {
            $( $false )?
        }
    };

    (let $pat:pat = $if:expr ; $true:expr) => {
        #[allow(clippy::question_mark)]
        if let $pat = $if {
            $true
        }
    };
    (let $pat:pat = $if:expr ; $true:expr ; $($false:expr)? ) => {
        if let $pat = $if {
            $true
        } else {
            $( $false )?
        }
    };
}
#[doc(inline)]
pub use iif;

#[cfg(test)]
mod test_iif {
    use crate::iif;

    #[test]
    fn iif() {
        assert_eq!('a', iif!(true ; 'a' ; 'b'));
        assert_eq!('b', iif!(false ; 'a' ; 'b'));
    }

    #[test]
    fn iif_let() {
        let somea = Some('a');
        let noa: Option<char> = None;

        assert_eq!('a', iif!(let Some(a) = somea; a ; 'b'));
        assert_eq!('b', iif!(let Some(a) = noa; a ; 'b'));
    }
}
