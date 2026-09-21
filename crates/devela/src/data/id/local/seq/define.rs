//
//! Defines [`id_seq!`], for sequential unique IDs. An identity allocator.
//

#[doc = crate::_tags!(construction id)]
/// A macro for constructing a unique sequential identifier generator.
#[doc = crate::_doc_meta!{
    location("data/id/local", macro id_seq),
}]
/// The underlying representation must implement [`PrimUint`].
/// The corresponding atomic integer type must also be available,
/// either natively on the target or through `dep_portable_atomic`.
///
/// IDs are generated sequentially from `0` up to one less than the
/// primitive maximum. The maximum value is reserved as the permanent
/// exhausted state.
///
/// Allocation uses [`Relaxed`] atomic ordering by default. Custom ordering
/// can be selected with the `*_with_ordering` methods.
///
/// # Examples
/// ```
/// use devela::id_seq;
///
/// id_seq![AppId, u8];
///
/// assert_eq![AppId::generated_ids(), 0];
/// assert_eq![AppId::remaining_ids(), u8::MAX];
///
/// assert_eq![AppId::new().unwrap().value(), 0];
/// assert_eq![AppId::new_unchecked().value(), 1];
///
/// // Generate all remaining IDs, ending at 254.
/// for _ in 2..u8::MAX {
///     let _ = AppId::new_unchecked();
/// }
///
/// assert_eq![AppId::generated_ids(), u8::MAX];
/// assert_eq![AppId::remaining_ids(), 0];
///
/// // Exhaustion is permanent.
/// assert_eq![AppId::new(), None];
/// assert_eq![AppId::new(), None];
/// ```
///
/// Only unsigned primitive integers are accepted:
/// ```compile_fail
/// use devela::id_seq;
///
/// id_seq![SignedId, i8];
/// ```
///
/// See also [`IdSeqU64Example`][crate::IdSeqU64Example].
///
/// [`PrimUint`]: crate::PrimUint
/// [`Relaxed`]: crate::AtomicOrdering::Relaxed
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! id_seq· {
    (
        $(#[$attr:meta])*
        $vis:vis $name:ident,
        $prim:ident
    ) => {
        $crate::paste! {
            $crate::__id_seq![%define
                $(#[$attr])*,
                $vis,
                $name,
                stringify!($name),
                [<$name:upper>],
                $prim,
                stringify!($prim),
                [<Atomic $prim:camel>]
            ];
        }
    };
}
#[doc(inline)]
pub use id_seq· as id_seq;
