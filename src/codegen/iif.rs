// devela::codegen::iif
//
//! inline if macro.
//

/// A more compact *`i`nline `if`*.
///
/// A inline alternative for `if` and `if let`.
///
/// # Examples
///
/// Replacing `if`:
/// ```
/// use devela::codegen::iif;
///
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
/// use devela::codegen::iif;
///
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
/// use devela::codegen::iif;
///
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
macro_rules! iif {
    ($if:expr; $true:expr) => {
        if $if {
            $true
        }
    };
    ($if:expr ; $true:expr ; $($false:expr)?) => {
        if $if {
            $true
        } else {
            $( $false )?
        }
    };

    (let $pat:pat = $if:expr ; $true:expr) => {
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
pub use iif;

#[cfg(test)]
mod test_iif {
    use crate::codegen::iif;

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
