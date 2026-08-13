// devela/src/data/codec/radix/define.rs
//
//! Defines [`Radix`].
//

#[doc = crate::_tags!(codec)]
/// A configurable radix-based binary-to-text codec.
#[doc = crate::_doc_meta!{
    location("data/codec"),
    test_size_of(__: Radix<16> = 1|8),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Radix<const BASE: u8> {
    pub(super) cfg: u8,
}

impl<const BASE: u8> Radix<BASE> {
    /// The numeric base.
    pub const BASE: u8 = BASE;

    pub(super) const fn configured(cfg: u8) -> Self {
        Self { cfg }
    }
}
