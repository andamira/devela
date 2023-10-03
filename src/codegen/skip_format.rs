// devela::codegen::skip_format
//
//! skip formatting macros
//

/// *`s`kip `f`ormatting* macro.
///
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// It can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// use devela::codegen::sf;
///
/// // rustfmt has no powers here
/// sf! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
macro_rules! sf { ( $($line:tt)+ ) => { $($line)+ }; }
pub use sf;

/// *`s`kip `f`ormatting `b`lock* macro.
///
/// Surrounds with brackets the provided code, and preserves its formatting.
///
/// It can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// use devela::codegen::sfb;
///
/// // rustfmt has no powers here
/// sfb! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
macro_rules! sfb { ( $($line:tt)+ ) => { { $($line)+ } }; }
pub use sfb;
