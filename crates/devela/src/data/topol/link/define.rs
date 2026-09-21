//
//! Defines [`link!`] macro.
//

#[doc = crate::_tags!(construction data_structure topol)]
/// Defines a compact record of named optional links.
#[doc = crate::_doc_meta!{
    location("data/topol/link", macro link),
}]
/// Each declared field represents an independent optional link to one target.
///
/// `link!` only represents direct links. It does not own linked values,
/// validate target existence or lifetime, maintain reciprocal links, or impose
/// higher-level topology such as chains, trees, or graphs.
///
/// Each field declares its primitive carrier and may optionally declare
/// a distinct storage representation:
///
/// - `name: Prim;` uses `Prim` as both carrier and representation.
/// - `name: Prim + Repr;` uses `Prim` as the carrier and `Repr` as the representation.
///
/// A niche-aware representation can keep the optional link compact.
///
/// # Example
/// ```
/// use devela::{NonMaxU8, NonMaxU16, link};
///
/// link! {
///     [
///         next: u8 + NonMaxU8;
///         prev: u8 + NonMaxU8;
///         parent: u16 + NonMaxU16;
///     ]
///     /// A small set of independent links.
///     pub MyLinks;
/// }
///
/// let mut links = MyLinks::from_prim(Some(7), None, Some(2)).unwrap();
///
/// assert_eq!(links.get_next_prim(), Some(7));
/// assert!(!links.has_prev());
/// assert_eq!(links.get_parent_prim(), Some(2));
///
/// links.set_prev_prim(3).unwrap();
/// assert_eq!(links.get_prev_prim(), Some(3));
///
/// links.clear_parent();
/// assert!(!links.has_parent());
/// ```
///
/// A field always contains at most one target. Variable-size relations such as
/// arbitrary graph adjacency require additional storage or a higher-level
/// topology built from links.
///
/// See also [`LinkExample`].
///
/// [`MaybeNiche`]: crate::MaybeNiche
/// [`LinkExample`]: crate::LinkExample
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
macro_rules! link· {
    (
        [ $($fields:tt)* ]
        $(#[$attr:meta])*
        $vis:vis $Links:ident $(;)?
    ) => {
        $crate::__link! { %parse_fields
            [] [$($fields)*] [$(#[$attr])* $vis $Links]
        }
    };
}
#[doc(inline)]
pub use link· as link;
