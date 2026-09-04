// devela/src/code/marker/repr.rs
//
//! Defines [`Repr`].
//

#[doc = crate::_tags!(code data)]
/// Selects how a value exposes one of its representations.
#[doc = crate::_doc_meta!{
    location("code/marker", enum ReprMode),
    test_size_of(ReprMode = 1|8; niche Option),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReprMode {
    #[doc = crate::_tags!(init)]
    /// Shows the direct underlying representation.
    #[default]
    Raw,
    /// Shows named semantic parts when available.
    Named,
    /// Shows both the direct representation and named semantic parts.
    RawNamed,
}
crate::_impl_init![Self::Raw => ReprMode];
impl crate::BitSized<3> for ReprMode {}
