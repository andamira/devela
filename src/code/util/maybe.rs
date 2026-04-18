// devela::code::util::maybe
//
//! Defines [`maybe!`], [`maybe_slot!`].
//

// MAYBE add: Debug, Display, PartialEq, PartialOrd, Drop
#[doc = crate::_tags!(code maybe)]
/// Helper for using optionally implemented traits, like `Default` or `Clone`.
#[doc = crate::_doc_location!("code/util")]
///
/// The first boolean argument says whether `$T` implements the given trait.
///
/// # Examples
/// ```
/// # use devela::{assert_eq_all, maybe, NonZeroU8};
/// assert_eq![maybe![default:true, u8], Some(0)];
/// assert_eq![maybe![default:true, &str], Some("")];
/// assert_eq![maybe![default:false, u8], None];
/// assert_eq![maybe![default:false, NonZeroU8], None];
///
/// let s1 = String::from("string1");
/// let s2 = maybe![clone:true, String, &s1].expect("cloned");
/// let s3 = maybe![clone:true, String, &s1].expect("cloned");
/// assert_eq_all![&s1, &s2, &s3];
/// ```
/// ```compile_fail
/// # use devela::{maybe, NonZeroU8};
/// let _ = maybe![default:true, NonZeroU8];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! maybe {
    ( // Returns either Some(<$T>::default()) or `None`.
      default: $implements_default:stmt, $T:ty ) => {{
        /* didactic notes */

        // // For this to compile $T always has to implement Default:
        //
        // $implements_default.then(|| Self::Value::$C_name(<$T>::default())),

        // // WAIT attributes on expressions are experimental
        // // https://github.com/rust-lang/rust/issues/15701
        //
        // #[crate::compile($implements_default)]
        // { Some(Self::Value::$C_name(<$T>::default())) }
        // #[crate::compile(not($implements_default))]
        // { None }

        // // WAIT: custom attributes can't be applied to statements
        // // https://github.com/rust-lang/rust/issues/54727
        //
        // #[crate::compile($implements_default)]
        // let res = Some(Self::Value::$C_name(<$T>::default()));
        // #[crate::compile(not($implements_default))]
        // let res = None;
        // res

        // The only solution for now:
        #[$crate::compile($implements_default)]
        fn maybe_default<T: Default>() -> Option<T> {
            Some(T::default())
        }
        #[$crate::compile(not($implements_default))]
        fn maybe_default<T>() -> Option<T> {
            None
        }
        maybe_default::<$T>()
    }};
    (
      // Returns either Some(<$value: $T>.clone()) or `None`.
      clone: $implements_clone:stmt, $T:ty, $value:expr ) => {{
        #[$crate::compile($implements_clone)]
        fn maybe_clone<T: Clone>(value: &T) -> Option<T> {
            Some(value.clone())
        }
        #[$crate::compile(not($implements_clone))]
        fn maybe_clone<T>(_value: &T) -> Option<T> {
            None
        }
        maybe_clone::<$T>($value)
    }};
}
#[doc(inline)]
pub use maybe;

#[doc = crate::_tags!(code maybe primitive)]
/// Expands to a primitive slot from either a direct carrier or a wrapped outer type.
#[doc = crate::_doc_location!("code/util")]
///
/// The first argument is the outside type used for dispatch.
///
/// This macro hardcodes a whitelist of type spellings treated as direct primitive
/// carriers, such as `u32`, `bool`, `char`, and crate aliases like `usize_up`.
/// For those spellings, the accessor suffix is accepted but ignored,
/// and the macro expands directly to `value`.
///
/// Any other type is treated as an outer wrapper type. In that case, the
/// accessor suffix must reach one of the direct primitive carriers,
/// or a reference to one.
///
/// This helper is mainly intended for use inside other macros, so they can
/// operate uniformly over direct primitive carriers and wrapper types
/// without needing separate expansion paths at the call site.
///
/// Primitive outside types expand directly to `value`.
/// Wrapper outside types use the accessor tokens to reach the inner primitive.
///
/// # Forms
/// - `maybe_slot!(Primitive, value, accessor)`
/// - `maybe_slot!(Wrapper, value, .field_path)`
/// - `maybe_slot!(Wrapper, value, * .ref_accessor())`
///
/// # Notes
/// - The accessor is always required, even for direct primitive carriers.
/// - Direct primitive carriers accept the accessor contract but ignore it.
/// - Prefix the accessor with `*` when it returns `&Prim` or `&mut Prim`.
/// - Setter-style methods are outside the scope of this macro.
/// - The direct-carrier whitelist is hardcoded and may grow over time.
///
/// # Examples
/// ```
/// # use devela::maybe_slot;
/// let mut a: u32 = 5;
/// let b = maybe_slot!(u32, a, .0);
/// assert_eq![b, 5];
///
/// maybe_slot!(u32, a, .0) = 9;
/// assert_eq![a, 9];
///
/// #[derive(Debug, PartialEq)]
/// struct Wrap(u32);
/// let mut w = Wrap(3);
/// assert_eq!(maybe_slot!(Wrap, w, .0), 3);
///
/// maybe_slot!(Wrap, w, .0) = 8;
/// assert_eq!(w, Wrap(8));
///
/// #[derive(Debug, PartialEq)]
/// struct Outer { inner: Wrap }
/// let mut o = Outer { inner: Wrap(11) };
/// maybe_slot!(Outer, o, .inner.0) = 22;
/// assert_eq!(o, Outer { inner: Wrap(22) });
///
/// // Reference-returning accessors need explicit dereferencing.
/// impl Wrap {
///     fn get_ref(&self) -> &u32 { &self.0 }
///     fn get_mut(&mut self) -> &mut u32 { &mut self.0 }
/// }
/// let q = maybe_slot!(Wrap, w, * .get_ref());
/// assert_eq!(q, 8);
///
/// maybe_slot!(Wrap, w, * .get_mut()) = 4;
/// assert_eq!(w, Wrap(4));
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! maybe_slot {
    (
    // Direct primitive-carrier spellings:
    // accept the accessor contract, but expand directly to the value itself.
     u8, $value:expr, $($_access:tt)+) => { $value };
    (u16, $value:expr, $($_access:tt)+) => { $value };
    (u32, $value:expr, $($_access:tt)+) => { $value };
    (u64, $value:expr, $($_access:tt)+) => { $value };
    (u128, $value:expr, $($_access:tt)+) => { $value };
    (usize, $value:expr, $($_access:tt)+) => { $value };
    (usize_up, $value:expr, $($_access:tt)+) => { $value };
    (usize_down, $value:expr, $($_access:tt)+) => { $value };
    (i8, $value:expr, $($_access:tt)+) => { $value };
    (i16, $value:expr, $($_access:tt)+) => { $value };
    (i32, $value:expr, $($_access:tt)+) => { $value };
    (i64, $value:expr, $($_access:tt)+) => { $value };
    (i128, $value:expr, $($_access:tt)+) => { $value };
    (isize, $value:expr, $($_access:tt)+) => { $value };
    (isize_up, $value:expr, $($_access:tt)+) => { $value };
    (isize_down, $value:expr, $($_access:tt)+) => { $value };
    (f32, $value:expr, $($_access:tt)+) => { $value };
    (f64, $value:expr, $($_access:tt)+) => { $value };
    (fsize, $value:expr, $($_access:tt)+) => { $value };
    (bool, $value:expr, $($_access:tt)+) => { $value };
    (char, $value:expr, $($_access:tt)+) => { $value };
    (

    // Wrapped access through a reference-returning accessor:
    // `* .get_ref()`, `* .get_mut()`, ...
     $outer_ty:ty, $value:expr, * $($access:tt)+) => { *(($value) $($access)+) };
    (
    // Wrapped access through a direct field or tuple-field path:
    // `.0`, `.inner.0`, ...
     $outer_ty:ty, $value:expr, $($access:tt)+) => { (($value) $($access)+) };
}
#[doc(inline)]
pub use maybe_slot;
