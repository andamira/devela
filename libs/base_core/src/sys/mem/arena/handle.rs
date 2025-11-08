// devela_base_core::sys::mem::arena::handle
//
//! Defines [`ArenaHandle`].
//

///
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArenaHandle {
    pub(super) offset: usize,
    pub(super) len: usize,
}
#[rustfmt::skip]
impl ArenaHandle {
    /// Creates a new handle.
    pub(crate) const fn new(offset: usize, len: usize) -> Self { ArenaHandle { offset, len } }

    /// Returns the length of the stored data.
    pub const fn len(self) -> usize { self.len }

    /// Returns the offset of the stored data.
    pub const fn offset(self) -> usize { self.offset }
}
