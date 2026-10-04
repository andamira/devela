//
//! Defines [`use_as!`].
//

#[doc = crate::_tags!(code)]
/// Imports related items while systematically adapting their local names.
#[doc = crate::_doc_meta!{
    location("code/util/synth", macro use_as),
}]
/// The `+` separates a common source prefix from a common local suffix:
/// - `Prefix+` imports `PrefixName as Name`.
/// - `+Suffix` imports `Name as NameSuffix`.
/// - `Prefix+Suffix` imports `PrefixName as NameSuffix`.
///
/// `Name as Alias` uses the explicit alias instead of the automatic local name.
/// `_` omits the item stem: `Prefix+` imports `Prefix`, while `Prefix+Suffix`
/// imports `Prefix as PrefixSuffix`. `_ as Alias` uses the explicit alias.
/// The suffix-only form does not accept `_` because it names no source item.
///
/// An optional visibility may precede the source path.
/// Relative and absolute paths are supported.
///
/// # Examples
///
/// This imports `Debug` as `Dbg`, `DebugList` as `ListFmt`,
/// `AtomicBool` as `Abool`, and `AtomicUsize` as `Usize`.
/// ```
/// devela::use_as! {
///     Debug+: devela::{_ as Dbg, List as ListFmt},
///     Atomic+: pub(crate) devela::{Bool as Abool, Usize},
/// }
///
/// struct Values([Abool; 3]);
///
/// impl Dbg for Values {
///     fn fmt(&self, f: &mut devela::Formatter<'_>) -> devela::FmtResult<()> {
///         let mut list: ListFmt<'_, '_> = f.debug_list();
///         list.entries(self.0.iter());
///         list.finish()
///     }
/// }
/// ```
///
/// A common suffix is added to the local names instead:
/// ```
/// devela::use_as! {
///     +Legacy: ::core::range::legacy::{
///         Range, RangeFrom, RangeInclusive, RangeToInclusive,
///     },
/// }
///
/// let _: Option<RangeLegacy<usize>> = None;
/// let _: Option<RangeFromLegacy<usize>> = None;
/// ```
///
/// Both sides can be combined:
/// ```
/// mod family { pub struct AtomicBool; }
///
/// devela::use_as! { Atomic+Legacy: self::family::{Bool} }
///
/// # fn main() {
/// let _: Option<BoolLegacy> = None;
/// # }
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! use_as· {
    ($pre:ident + $($rest:tt)*) => {
        $crate::__use_as![%groups $pre + $($rest)* , @end];
    };
    (+ $suf:ident $($rest:tt)*) => {
        $crate::__use_as![%groups + $suf $($rest)* , @end];
    };
}
#[doc(inline)]
pub use use_as· as use_as;

#[doc(hidden)]
#[macro_export]
macro_rules! __use_as· {
    /* group parsing */

    (%groups @end) => {};
    (%groups , $($rest:tt)*) => {
        $crate::__use_as![%groups $($rest)*];
    };

    /* absolute source path */
    (%groups $pre:ident + $suf:ident: $v:vis :: $($p:ident)::+::{ $($items:tt)* }, $($rest:tt)*) => {
        $crate::__use_as![%items [$pre] [$suf] [$v] [::] [$($p)::+] $($items)* , @end];
        $crate::__use_as![%groups $($rest)*];
    };
    (%groups $pre:ident + : $v:vis :: $($p:ident)::+::{ $($items:tt)* }, $($rest:tt)*) => {
        $crate::__use_as![%items [$pre] [] [$v] [::] [$($p)::+] $($items)* , @end];
        $crate::__use_as![%groups $($rest)*];
    };
    (%groups + $suf:ident: $v:vis :: $($p:ident)::+::{ $($items:tt)* }, $($rest:tt)*) => {
        $crate::__use_as![%items [] [$suf] [$v] [::] [$($p)::+] $($items)* , @end];
        $crate::__use_as![%groups $($rest)*];
    };

    /* relative source path */
    (%groups $pre:ident + $suf:ident: $v:vis $($p:ident)::+::{ $($items:tt)* }, $($rest:tt)*) => {
        $crate::__use_as![%items [$pre] [$suf] [$v] [] [$($p)::+] $($items)* , @end];
        $crate::__use_as![%groups $($rest)*];
    };
    (%groups $pre:ident + : $v:vis $($p:ident)::+::{ $($items:tt)* }, $($rest:tt)*) => {
        $crate::__use_as![%items [$pre] [] [$v] [] [$($p)::+] $($items)* , @end];
        $crate::__use_as![%groups $($rest)*];
    };
    (%groups + $suf:ident: $v:vis $($p:ident)::+::{ $($items:tt)* }, $($rest:tt)*) => {
        $crate::__use_as![%items [] [$suf] [$v] [] [$($p)::+] $($items)* , @end];
        $crate::__use_as![%groups $($rest)*];
    };

    /* item parsing */

    (%items $pre:tt $suf:tt $v:tt $root:tt $path:tt @end) => {};
    (%items $pre:tt $suf:tt $v:tt $root:tt $path:tt , $($rest:tt)*) => {
        $crate::__use_as![%items $pre $suf $v $root $path $($rest)*];
    };

    /* explicit alias: it overrides the automatic local suffix */
    (%items [$pre:ident] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        $n:ident as $a:ident, $($rest:tt)*) => {
        $crate::paste! { $v use $($r)* $($p)::+::[<$pre $n>] as $a; }
        $crate::__use_as![%items [$pre] [$suf] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [$pre:ident] [] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        $n:ident as $a:ident, $($rest:tt)*) => {
        $crate::paste! { $v use $($r)* $($p)::+::[<$pre $n>] as $a; }
        $crate::__use_as![%items [$pre] [] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        $n:ident as $a:ident, $($rest:tt)*) => {
        $v use $($r)* $($p)::+::$n as $a;
        $crate::__use_as![%items [] [$suf] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };

    /* automatic local name */
    (%items [$pre:ident] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        $n:ident, $($rest:tt)*) => {
        $crate::paste! { $v use $($r)* $($p)::+::[<$pre $n>] as [<$n $suf>]; }
        $crate::__use_as![%items [$pre] [$suf] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [$pre:ident] [] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        $n:ident, $($rest:tt)*) => {
        $crate::paste! { $v use $($r)* $($p)::+::[<$pre $n>] as $n; }
        $crate::__use_as![%items [$pre] [] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        $n:ident, $($rest:tt)*) => {
        $crate::paste! { $v use $($r)* $($p)::+::$n as [<$n $suf>]; }
        $crate::__use_as![%items [] [$suf] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };

    /* bare source prefix: `_` */
    (%items [$pre:ident] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        _ as $a:ident, $($rest:tt)*) => {
        $v use $($r)* $($p)::+::$pre as $a;
        $crate::__use_as![%items [$pre] [$suf] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [$pre:ident] [] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        _ as $a:ident, $($rest:tt)*) => {
        $v use $($r)* $($p)::+::$pre as $a;
        $crate::__use_as![%items [$pre] [] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [$pre:ident] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        _, $($rest:tt)*) => {
        $crate::paste! { $v use $($r)* $($p)::+::$pre as [<$pre $suf>]; }
        $crate::__use_as![%items [$pre] [$suf] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [$pre:ident] [] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        _, $($rest:tt)*) => {
        $v use $($r)* $($p)::+::$pre;
        $crate::__use_as![%items [$pre] [] [$v] [$($r)*] [$($p)::+] $($rest)*];
    };
    (%items [] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        _ as $a:ident, $($rest:tt)*) => {
        compile_error!("`_` in `use_as!` requires a source prefix");
    };
    (%items [] [$suf:ident] [$v:vis] [$($r:tt)*] [$($p:ident)::+]
        _, $($rest:tt)*) => {
        compile_error!("`_` in `use_as!` requires a source prefix");
    };
}
#[doc(hidden)]
pub use __use_as· as __use_as;
