// devela::sugar
//
//! Syntax sugar abstractions.
//
// TOC
// - fns
//   - bx
//
// - macros
//   - cdbg!
//   - iif!
//   - rfs!

/* fns */

/// Brief [`Box`][alloc::boxed::Box] constructor.
///
/// # Examples
/// ```
/// use devela::all::bx;
///
/// assert_eq![bx(45), Box::new(45)];
/// ```
#[inline(always)]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn bx<T>(v: T) -> alloc::boxed::Box<T> {
    alloc::boxed::Box::new(v)
}

/* macros */

/// *`c`compact [`dbg!`]*. Uses `{:?}` instead of `{:#?}` for formatting.
///
/// # Examples
/// ```
/// use devela::all::cdbg;
///
/// let a = vec![1, 2, 3];
/// let _b = cdbg![a];
/// //       ^-- prints: [src/main.rs:5] a = [1, 2, 3]
/// ```
// Source code based on the original `dbg!` implementation.
#[macro_export]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! cdbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}", // <- KEY CHANGE
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(cdbg!($val)),+,)
    };
}
#[cfg(feature = "std")]
pub use cdbg;

/// *`i`nline `if`*.
///
/// A inline alternative for `if` and `if let`.
///
/// # Examples
///
/// Replacing `if`:
/// ```
/// use devela::all::iif;
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
/// use devela::all::iif;
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
/// use devela::all::iif;
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

/// *`r`ust `f`ormat `s`kip* macro.
///
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// It can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// use devela::all::rfs;
///
/// // rustfmt has no powers here
/// rfs! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
macro_rules! rfs { ( $($line:tt)+ ) => { $($line)+ }; }
pub use rfs;

#[cfg(test)]
#[allow(non_snake_case)]
mod test_S {
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
