// devela/src/num/dom/int/macro/define.rs
//
//! Defines [`int!`] and its primitive-backed integer value methods.

#[doc = crate::_tags!(construction num)]
/// Defines a transparent integer value type over a primitive carrier.
#[doc = crate::_doc_meta!{
    location("num/dom/int", macro int),
}]
/// Operation families are selected explicitly with `ops(...)`.
/// The initial implementation supports the `base` family.
/// See [`IntExample`][crate::IntExample] for signed and unsigned method examples.
///
/// # Syntax
/// ```
/// use devela::int;
/// // A concrete integer backed by one primitive:
/// int! { pub struct MyInt: (i32) impl; ops(base); }
/// assert_eq!(MyInt::new(-128).digits().get(), 3);
///
/// // A generic shell with selected primitive implementations:
/// int! { pub struct MyInts: <> impl (i16, i32, u32); ops(base); }
/// assert_eq!(MyInts::<u32>::new(127).digits().get(), 3);
///
/// // Deferral, like divisor!:
/// int! { pub struct LateInts: <>; }
/// int! { impl LateInts: <> (i8, u8); ops(base); }
/// assert_eq!(LateInts::<i8>::new(-128).digital_root().get(), 2);
/// ```
/// The original compact syntax (`struct MyInt: i32; ops(base);`) also works.
///
/// The generated type is a transparent value carrier with const `new` and `get`
/// methods. Semantic operation families are emitted only when requested.
/// Rust operator traits are not implied by these families.
/// A generic shell exposes methods only for primitive carriers listed in its `impl`.
///
/// # Selectivity
/// ```compile_fail
/// use devela::int;
///
/// int! {
///     struct BaseOnly: i32;
///     ops(base);
/// }
///
/// let _ = BaseOnly::new(7).div_ceil(BaseOnly::new(2));
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! int· {
    /* divisor!-style definitions: concrete and generic */

    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : ($Carrier:ident) impl;
        ops($($op:ident),* $(,)?);
    ) => {
        $crate::int! {
            $(#[$attr])*
            $vis struct $Name : $Carrier;
            ops($($op),*);
        }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : <> impl ($($Carrier:ident),+ $(,)?);
        ops($($op:ident),* $(,)?);
    ) => {
        $crate::int! {
            $(#[$attr])*
            $vis struct $Name : <>;
        }
        $crate::int! { impl $Name : <> ($($Carrier),+); ops($($op),*); }
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : <>;
    ) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $Name<T>(T);
    };
    (
        impl $Name:ident : <> ($($Carrier:ident),+ $(,)?);
        ops($($op:ident),* $(,)?);
    ) => {
        $crate::__int!(%impl_generic
            name[$Name] carriers[$($Carrier),+] ops[$($op),*]
        );
    };
    (
        $(#[$attr:meta])*
        $vis:vis struct $Name:ident : $Carrier:tt;
        ops($($op:ident),* $(,)?);
    ) => {
        $(#[$attr])*
        #[must_use]
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $Name($Carrier);

        impl $Name {
            /// Creates a value from its primitive carrier.
            $vis const fn new(value: $Carrier) -> Self { Self(value) }
            /// Returns the primitive carrier value.
            $vis const fn get(self) -> $Carrier { self.0 }
        }

        $crate::__int!(%dispatch_carrier
            vis[$vis] name[$Name] target[$Name] carrier[$Carrier] ops[$($op),*]
        );
    };
}
#[doc(inline)]
pub use int· as int;
