// devela::code::marker::repr
//
//! Defines [`Repr`].
//

#[doc = crate::_tags!(code data)]
/// Selects how a value exposes one of its representations.
#[doc = crate::_doc_location!("code/marker")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReprMode {
    /// Shows the direct underlying representation.
    #[default]
    Raw,
    /// Shows named semantic parts when available.
    Named,
    /// Shows both the direct representation and named semantic parts.
    RawNamed,
}
crate::_impl_init![Self::Raw => ReprMode];
