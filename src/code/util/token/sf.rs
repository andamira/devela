// devela/src/code/util/token/sf.rs
//
//! Defines skip formatting macro [`sf!`].
//

#[doc = crate::_tags!(code fmt)]
/// *`s`kip `f`ormatting* macro.
#[doc = crate::_doc_meta!{
    location("code/util/token", macro sf),
}]
/// Preserves the formatting of the code provided as arguments, by relying on
/// the fact that `rustfmt` does not usually apply formatting inside macros.
///
/// *Rust will format macros only if they use parenthesis `()`
/// and the input is separated by commas, as if it were a function call.*
///
/// This macro can be used as an alternative to the `#[rustfmt::skip]` attribute,
/// specially in places where it can't be applied yet on stable rust.
///
/// # Examples
/// ```
/// use devela::sf;
///
/// // rustfmt has no powers here
/// sf! { println!(); for i in 0..3 { print!{"{i} "} } println!(); }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! sf· { ( $($line:tt)+ ) => { $($line)+ }; }
#[doc(inline)]
pub use sf· as sf;
