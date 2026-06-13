// devela/src/code/util/derive/alias.rs
//
//! Attribute adapters for declarative macros.
//
// WAIT: [attributes on expressions](https://github.com/rust-lang/rust/issues/15701)
// NOTE: paste! is used to avoid the following error:
// macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths

#[doc = crate::_tags!(code)]
/// Defines attribute aliases usable from [`macro_apply`][crate::macro_apply].
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// These aliases expand to attributes,
/// so they only apply where attributes are accepted.
///
/// Generated aliases are emitted through a hidden macro and a re-export.
/// Add `#[doc(inline)]` to the alias definition to show it in public docs.
///
/// For out-of-line modules (`mod name;`), call the alias macro directly instead.
// WAIT: [proc_macro_hygiene](https://github.com/rust-lang/rust/issues/54727#issuecomment-485181171)
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
macro_rules! macro_apply_alias {
    () => {};
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::macro_apply_alias! {% [$(#[$meta])*] $vis $name () ($(#[$($attrs)*])+) }
        $crate::macro_apply_alias! { $($rest)* }
    };
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident ($($params:tt)*) = $(#[$($attrs:tt)*])+;
        $($rest:tt)*
    ) => {
        $crate::macro_apply_alias! {% [$(#[$meta])*] $vis $name ($($params)*) ($(#[$($attrs)*])+) }
        $crate::macro_apply_alias! { $($rest)* }
    };
    (% [$($meta:tt)*] $vis:vis $name:ident () ($(#[$($attrs:tt)*])+)) => {
        $crate::macro_dollar! { ($d:tt) => { $crate::paste! {
            #[allow(nonstandard_style)]
            mod [<__mod_ $name __>] {
                #[doc(hidden)]
                #[allow(unused_macros)]
                #[$crate::compile_attr(same($vis, pub), macro_export)]
                macro_rules! [<__ $name __>] {
                    ($d($item:tt)*) => { $(#[$($attrs)*])+ $d($item)* };
                }
                #[$crate::compile(none($vis))]
                pub(super) use [<__$name __>] as $name;
                #[$crate::compile(some($vis))]
                $vis use [<__$name __>] as $name;
            }
            $($meta)*
            #[allow(unused_imports)]
            #[doc = "\n\n---"]
            #[doc = "*Generated with [`macro_apply_alias`][$crate::macro_apply_alias]*."]
            $vis use [<__mod_ $name __>]::$name;
        }}}
    };
    (% [$($meta:tt)*] $vis:vis $name:ident ($($params:tt)*) ($(#[$($attrs:tt)*])+)) => {
        $crate::macro_dollar! { ($d:tt) => { $crate::paste! {
            #[allow(nonstandard_style)]
            mod [<__mod_ $name __>] {
                #[doc(hidden)]
                #[allow(unused_macros)]
                #[$crate::compile_attr(same($vis, pub), macro_export)]
                macro_rules! [<__$name __>] {
                    (($($params)*) $d($item:tt)*) => { $(#[$($attrs)*])+ $d($item)* };
                }
                #[$crate::compile(none($vis))]
                pub(super) use [<__$name __>] as $name;
                #[$crate::compile(some($vis))]
                $vis use [<__$name __>] as $name;
            }
            $($meta)*
            #[allow(unused_imports)]
            #[doc = "\n\n---"]
            #[doc = "*Generated with [`macro_apply_alias`][$crate::macro_apply_alias]*."]
            $vis use [<__mod_ $name __>]::$name;
        }}}
    };
}
#[doc(inline)]
pub use macro_apply_alias;

#[doc = crate::_tags!(code)]
/// Defines derive aliases usable from [`macro_derive`][crate::macro_derive].
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// These aliases expand through declarative derive macros,
/// so they apply to items inspected by `macro_derive`.
///
/// Generated aliases are emitted through a hidden macro and a re-export.
/// Add `#[doc(inline)]` to the alias definition to show it in public docs.
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
macro_rules! macro_derive_alias {
    () => {};
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::macro_derive_alias! {% [$(#[$meta])*] $vis $name () ($($derives)*) }
        $crate::macro_derive_alias! { $($rest)* }
    };
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident ($($params:tt)*) = #[derive($($derives:tt)*)];
        $($rest:tt)*
    ) => {
        $crate::macro_derive_alias! {% [$(#[$meta])*] $vis $name ($($params)*) ($($derives)*) }
        $crate::macro_derive_alias! { $($rest)* }
    };
    (% [$($meta:tt)*] $vis:vis $name:ident () ($($derives:tt)*)) => {
        $crate::macro_dollar! { ($d:tt) => { $crate::paste! {
            #[allow(nonstandard_style)]
            mod [<__mod_ $name __>] {
                #[doc(hidden)]
                #[allow(unused_macros)]
                #[$crate::compile_attr(same($vis, pub), macro_export)]
                macro_rules! [<__ $name __>] {
                    ($d($item:tt)*) => {
                        $crate::__macro_nested_derive! { #[derive($($derives)*)] $d($item)* }
                    };
                }
                #[$crate::compile(none($vis))]
                pub(super) use [<__$name __>] as $name;
                #[$crate::compile(some($vis))]
                $vis use [<__ $name __>] as $name;
            }
            $($meta)*
            #[allow(unused_imports)]
            #[doc = "\n\n---"]
            #[doc = "*Generated with [`macro_derive_alias`][$crate::macro_derive_alias]*."]
            $vis use [<__mod_ $name __>]::$name;
        }}}
    };
    (% [$($meta:tt)*] $vis:vis $name:ident ($($params:tt)*) ($($derives:tt)*)) => {
        $crate::macro_dollar! { ($d:tt) => { $crate::paste! {
            #[allow(nonstandard_style)]
            mod [<__mod_ $name __>] {
                #[doc(hidden)]
                #[allow(unused_macros)]
                #[$crate::compile_attr(same($vis, pub), macro_export)]
                macro_rules! [<__ $name __>] {
                    (($($params)*) $d($item:tt)*) => {
                        $crate::__macro_nested_derive! { #[derive($($derives)*)] $d($item)* }
                    };
                }
                #[$crate::compile(none($vis))]
                pub(super) use [<__$name __>] as $name;
                #[$crate::compile(some($vis))]
                $vis use [<__ $name __>] as $name;
            }
            $($meta)*
            #[allow(unused_imports)]
            #[doc = "\n\n---"]
            #[doc = "*Generated with [`macro_derive_alias`][$crate::macro_derive_alias]*."]
            $vis use [<__mod_ $name __>]::$name;
        }}}
    };
}
#[doc(inline)]
pub use macro_derive_alias;

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
