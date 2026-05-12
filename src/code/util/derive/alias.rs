// devela::code::util::derive
//
//! Attribute adapters for declarative macros.
//

#[doc = crate::_tags!(code)]
/// Defines attribute aliases usable from [`macro_apply`][crate::macro_apply].
#[doc = crate::_doc_location!("code/util")]
///
/// # Examples
/// ```
/// # use devela::{macro_apply_alias, macro_apply};
/// macro_apply_alias! {
///     inline_must =
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
///
/// Passing arguments:
/// ```
/// # use devela::{macro_apply_alias, macro_apply};
/// macro_apply_alias! {
///     repr_doc($repr:ident, $doc:literal) =
///         #[repr($repr)]
///         #[doc = $doc]
///     ;
/// }
///
/// #[macro_apply(repr_doc(u8, "A compact enum."))]
/// enum Small {
///     A = 1,
/// }
///
/// assert_eq!(size_of::<Small>(), 1);
/// assert_eq!(Small::A as u8, 1);
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! macro_apply_alias· {
    () => {};
    (
        $name:ident = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias! { #[macro_apply($name)] = $(#[$($attrs)*])+; }
        $crate::macro_apply_alias! { $($rest)* }
    };
    (
        $name:ident ($($params:tt)*) = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias! { #[macro_apply($name($($params)*))] = $(#[$($attrs)*])+; }
        $crate::macro_apply_alias! { $($rest)* }
    };
    (
        pub $name:ident = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias! { pub #[macro_apply($name)] = $(#[$($attrs)*])+; }
        $crate::macro_apply_alias! { $($rest)* }
    };
    (
        pub $name:ident ($($params:tt)*) = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias! { pub #[macro_apply($name($($params)*))] = $(#[$($attrs)*])+; }
        $crate::macro_apply_alias! { $($rest)* }
    };
}
#[doc(inline)]
pub use macro_apply_alias· as macro_apply_alias;

// Hidden implementation grammar for macro_apply_alias.
//
// This form mirror the eventual use-site attributes,
// while the public builders use a simpler `name(...) = ...;` syntax
#[doc(hidden)]
#[macro_export]
macro_rules! __macro_apply_alias {
    () => {};
    (
        #[macro_apply($name:ident ($($params:tt)*))] = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias!(%def_args pub(in crate) $name ($($params)*) ($(#[$($attrs)*])+));
        $crate::__macro_apply_alias! { $($rest)* }
    };
    (
        pub #[macro_apply($name:ident ($($params:tt)*))] = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias!(%def_args pub $name ($($params)*) ($(#[$($attrs)*])+));
        $crate::__macro_apply_alias! { $($rest)* }
    };
    (
        #[macro_apply($name:ident $(!)?)] = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias!(%def pub(in crate) $name ($(#[$($attrs)*])+));
        $crate::__macro_apply_alias! { $($rest)* }
    };
    (
        pub #[macro_apply($name:ident $(!)?)] = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::__macro_apply_alias!(%def pub $name ($(#[$($attrs)*])+));
        $crate::__macro_apply_alias! { $($rest)* }
    };
    (%def_args $vis:vis $name:ident ($($params:tt)*) ($(#[$($attrs:tt)*])+)) => {
        $crate::macro_dollar! { ($d:tt) => {
            #[allow(nonstandard_style)]
            macro_rules! $name {
                (($($params)*) $d($item:tt)*) => {
                    $(#[$($attrs)*])+
                    $d($item)*
                };
            }
            #[allow(unused_imports)]
            $vis use $name;
        }}
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

#[doc = crate::_tags!(code)]
/// Defines derive aliases usable from [`macro_derive`][crate::macro_derive].
#[doc = crate::_doc_location!("code/util")]
///
/// # Example
/// ```
/// # use devela::{macro_derive_alias, macro_derive};
/// macro_derive_alias! {
///     CopyEq = #[derive(Clone, Copy, PartialEq, Eq)];
///     Value = #[derive(CopyEq!, Debug)];
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
///
/// Passing arguments:
/// ```
/// # use devela::{macro_derive_alias, macro_derive};
/// macro_rules! derive_label {
///     (
///         ($label:literal)
///         $(#[$meta:meta])*
///         $vis:vis struct $Name:ident ($($fields:tt)*);
///     ) => {
///         impl $Name {
///             pub const LABEL: &'static str = $label;
///         }
///     };
/// }
///
/// macro_derive_alias! {
///     Value($label:literal) =
///         #[derive(Clone, Copy, Debug, PartialEq, Eq, derive_label!($label))];
/// }
///
/// #[macro_derive(Value!("id"))]
/// struct Id(u32);
///
/// let a = Id(7);
/// let b = a;
///
/// assert_eq!(a, b);
/// assert_eq!(Id::LABEL, "id");
/// assert_eq!(format!("{a:?}"), "Id(7)");
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! macro_derive_alias· {
    () => {};
    (
        $name:ident = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias! { #[derive($name!)] = #[derive($($derives)*)]; }
        $crate::macro_derive_alias! { $($rest)* }
    };
    (
        $name:ident ($($params:tt)*) = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias! { #[derive($name!($($params)*))] = #[derive($($derives)*)]; }
        $crate::macro_derive_alias! { $($rest)* }
    };
    (
        pub $name:ident = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias! { pub #[derive($name!)] = #[derive($($derives)*)]; }
        $crate::macro_derive_alias! { $($rest)* }
    };
    (
        pub $name:ident ($($params:tt)*) = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias! {
            pub #[derive($name!($($params)*))] = #[derive($($derives)*)]; }
        $crate::macro_derive_alias! { $($rest)* }
    };
}
#[doc(inline)]
pub use macro_derive_alias· as macro_derive_alias;

// Hidden implementation grammar for macro_derive_alias.
//
// This form mirror the eventual use-site attributes,
// while the public builders use a simpler `name(...) = ...;` syntax
#[doc(hidden)]
#[macro_export]
macro_rules! __macro_derive_alias {
    () => {};
    (
        #[derive($name:ident ! ($($params:tt)*))] = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias!(%def_args pub(in crate) $name ($($params)*) ($($derives)*));
        $crate::__macro_derive_alias! { $($rest)* }
    };
    (
        pub #[derive($name:ident ! ($($params:tt)*))] = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias!(%def_args pub $name ($($params)*) ($($derives)*));
        $crate::__macro_derive_alias! { $($rest)* }
    };
    (
        #[derive($name:ident !)] = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias!(%def pub(in crate) $name ($($derives)*));
        $crate::__macro_derive_alias! { $($rest)* }
    };
    (
        pub #[derive($name:ident !)] = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::__macro_derive_alias!(%def pub $name ($($derives)*));
        $crate::__macro_derive_alias! { $($rest)* }
    };
    (%def_args $vis:vis $name:ident ($($params:tt)*) ($($derives:tt)*)) => {
        $crate::macro_dollar! { ($d:tt) => {
            #[allow(nonstandard_style)]
            macro_rules! $name {
                (($($params)*) $d($item:tt)*) => {
                    $crate::__macro_nested_derive! {
                        #[derive($($derives)*)]
                        $d($item)*
                    }
                };
            }
            #[allow(unused_imports)]
            $vis use $name;
        }}
    };
    (%def $vis:vis $name:ident ($($derives:tt)*)) => {
        $crate::macro_dollar! { ($d:tt) => {
            #[allow(nonstandard_style)]
            macro_rules! $name {
                ($d($item:tt)*) => {
                    $crate::__macro_nested_derive! {
                        #[derive($($derives)*)]
                        $d($item)*
                    }
                };
            }
            #[allow(unused_imports)]
            $vis use $name;
        }}
    };
}

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
