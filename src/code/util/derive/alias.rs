// devela::code::util::derive
//
//! Attribute adapters for declarative macros.
//

#[doc = crate::_tags!(code)]
/// Defines derive aliases usable from [`macro_derive`][crate::macro_derive].
#[doc = crate::_doc_location!("code/util")]
///
/// # Example
/// ```
/// use devela::{derive_alias, macro_derive};
///
/// derive_alias! {
///     #[derive(CopyEq!)] = #[derive(Clone, Copy, PartialEq, Eq)];
///     #[derive(Value!)] = #[derive(CopyEq!, Debug)];
/// }
///
/// #[macro_derive(Value!)]
/// struct Id(u32);
///
/// let a = Id(7);
/// let b = a; // `Copy`.
///
/// assert_eq!(a, b);
/// assert_eq!(format!("{a:?}"), "Id(7)");
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! derive_alias· {
    () => {};
    (
        #[derive($name:ident !)] = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::derive_alias!(%def pub(in crate) $name ($($derives)*));
        $crate::derive_alias! { $($rest)* }
    };
    (
        pub #[derive($name:ident !)] = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::derive_alias!(%def pub $name ($($derives)*));
        $crate::derive_alias! { $($rest)* }
    };
    (%def $vis:vis $name:ident ($($derives:tt)*)) => {
        $crate::macro_dollar! { ($d:tt) => {
            #[allow(nonstandard_style)]
            macro_rules! $name {
                ($d($item:tt)*) => {
                    $crate::__macro_nested_derive! { #[derive($($derives)*)] $d($item)* }
                };
            }
            #[allow(unused_imports)]
            $vis use $name;
        }}
    };
}
#[doc(inline)]
pub use derive_alias· as derive_alias;

#[doc = crate::_tags!(code)]
/// Defines attribute aliases usable from [`macro_apply`][crate::macro_apply].
#[doc = crate::_doc_location!("code/util")]
///
/// # Examples
/// ```
/// use devela::{attr_alias, macro_apply};
///
/// attr_alias! {
///     #[macro_apply(inline_must)] =
///         #[inline]
///         #[must_use]
///     ;
/// }
///
/// #[macro_apply(inline_must)]
/// fn answer() -> u8 {
///     42
/// }
///
/// assert_eq!(answer(), 42);
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! attr_alias· {
    () => {};
    (
        #[macro_apply($name:ident $(!)?)] = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::attr_alias!(%def pub(in crate) $name ($(#[$($attrs)*])+));
        $crate::attr_alias! { $($rest)* }
    };
    (
        pub #[macro_apply($name:ident $(!)?)] = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::attr_alias!(%def pub $name ($(#[$($attrs)*])+));
        $crate::attr_alias! { $($rest)* }
    };
    (%def $vis:vis $name:ident ($(#[$($attrs:tt)*])+)) => {
        $crate::macro_dollar! { ($d:tt) => {
            #[allow(nonstandard_style)]
            macro_rules! $name {
                ($d($item:tt)*) => {
                    $(#[$($attrs)*])+
                    $d($item)*
                };
            }

            #[allow(unused_imports)]
            $vis use $name;
        }}
    };
}
#[doc(inline)]
pub use attr_alias· as attr_alias;

/// Recursively applies #[derive(...)] then drops duplicate item.
#[doc(hidden)]
#[macro_export]
macro_rules! __macro_nested_derive {
    ( #[derive($($derives:tt)*)] $($item:tt)* ) => {
        #[$crate::macro_derive($($derives)*)]
        #[$crate::macro_apply($crate::__macro_drop_item!)]
        $($item)*
    };
}
/// Internal item eraser for nested derive expansion.
#[doc(hidden)]
#[macro_export]
macro_rules! __macro_drop_item {
    ($it:item) => {};
}
