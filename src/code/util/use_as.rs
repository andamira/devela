// devela/src/code/util/use_as.rs
//
//! Defines [`use_as!`].
//

#[doc = crate::_tags!(code)]
/// Imports families of prefixed items under shorter local names.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// The shared prefix is prepended to each listed suffix to form its source name:
/// - `Name` imports `PrefixName as Name`.
/// - `Name as Alias` imports `PrefixName as Alias`.
/// - `_` imports `Prefix`.
/// - `_ as Alias` imports `Prefix as Alias`.
///
/// An optional visibility may precede the source path.
///
/// # Examples
///
/// This imports `Debug` as `Dbg`, `DebugList` as `ListFmt`,
/// `AtomicBool` as `Abool` and `AtomicUsize` as `Usize`.
/// ```
/// devela::use_as! {
///     +Debug: devela::{_ as Dbg, List as ListFmt},
///     +Atomic: pub(crate) devela::{Bool as Abool, Usize},
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
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! use_as {
    ($( +$prefix:ident: $vis:vis $($path:ident)::+::{ $($items:tt)* } ),+ $(,)?) => {
        $( $crate::use_as![%[$prefix] [$vis] [$($path)::+] [] $($items)*]; )+
    };
    /* finished */
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*]) => {
        $crate::paste! { $v use $($p)::+::{ $($i)* }; }
    };
    /* bare prefix: `_` */
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] _, $($t:tt)*) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* $pre,] $($t)*];
    };
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] _) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* $pre,]];
    };
    /* bare prefix with custom alias: `_ as Name` */
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] _ as $a:ident, $($t:tt)*) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* $pre as $a,] $($t)*];
    };
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] _ as $a:ident) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* $pre as $a,]];
    };
    /* suffixed item with custom alias */
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] $n:ident as $a:ident, $($t:tt)*) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* [<$pre $n>] as $a,] $($t)*];
    };
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] $n:ident as $a:ident) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* [<$pre $n>] as $a,]];
    };
    /* suffixed item */
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] $n:ident, $($t:tt)*) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* [<$pre $n>] as $n,] $($t)*];
    };
    (%[$pre:ident] [$v:vis] [$($p:ident)::+] [$($i:tt)*] $n:ident) => {
        $crate::use_as![%[$pre] [$v] [$($p)::+] [$($i)* [<$pre $n>] as $n,]];
    };
}
#[doc(inline)]
pub use use_as;
