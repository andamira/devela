// devela_base_core::code::util::maybe
//
//! Defines the [`maybe!`] macro helper.
//
// MAYBE add: Debug, Display, PartialEq, PartialOrd, Drop

#[doc = crate::_TAG_CODE!()]
#[doc = crate::_TAG_MAYBE!()]
/// Helper for using optionally implemented traits, like `Default` or `Clone`.
#[doc = crate::_doc!(location: "code/util")]
///
/// The first boolean argument says whether `$T` implements the given trait.
///
/// # Examples
/// ```
/// # use devela_base_core::{assert_eq_all, maybe, NonZeroU8};
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
/// # use devela_base_core::{maybe, NonZeroU8};
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
