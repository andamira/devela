// devela::sugar
//
//! Syntax sugar abstractions.
//
// TOC
// - bx
// - iif!
// - rfs!

/// Brief [`Box`] constructor.
///
/// # Examples
/// ```
/// use devela::bx;
///
/// assert_eq![bx(45), Box::new(45)];
/// ```
#[inline(always)]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn bx<T>(v: T) -> Box<T> {
    Box::new(v)
}

/// *`i`nline `if`* macro.
///
/// # Examples
/// ```
/// use devela::iif;
///
/// let s = iif![1 > 0; true; false];
/// iif![1 > 0; println!("true")];
/// ```
/// instead of
/// ```
/// let s = if 1 > 0 {
///     true
/// } else {
///     false
/// };
/// if 1 > 0 {
///     println!("true");
/// }
/// ```
#[macro_export]
macro_rules! iif {
    ($if: expr; $true: expr) => {
        if $if {
            $true
        }
    };
    ($if: expr ; $true: expr ; $false: expr) => {
        if $if {
            $true
        } else {
            $false
        }
    };
}

#[cfg(test)]
mod test_iif {
    use crate::iif;

    #[test]
    fn iif() {
        assert_eq!('a', iif!(true ; 'a' ; 'b'));
        assert_eq!('b', iif!(false ; 'a' ; 'b'));
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
/// use devela::rfs;
///
/// // rustfmt has no powers here
/// rfs! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
macro_rules! rfs { ( $($line:tt)+ ) => { $($line)+ }; }
