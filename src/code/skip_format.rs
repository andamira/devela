// devela::code::skip_format
//
//! skip formatting macros
//

/// *`s`kip `f`ormatting* wrapper macro.
///
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// *Rust will format macros only if they use parenthesis `()` and the input is
/// separated by commas, then it formats is the same way as function call.*
///
/// This macro can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially in places where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// # use devela::code::sf;
/// // rustfmt has no powers here
/// sf! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
macro_rules! sf { ( $($line:tt)+ ) => { $($line)+ }; }
pub use sf;
