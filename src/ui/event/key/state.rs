// devela/src/ui/event/key/state.rs
//
//! Defines [`KeyState`].
//

use crate::ConstInit;

#[doc = crate::_tags!(interaction)]
/// Represents the state of a [`Key`][crate::Key].
#[doc = crate::_doc_meta!{location("ui/event")}]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyState {
    /// The key was pressed.
    #[default]
    Press,
    /// The key was released.
    Release,
    /// The key was repeated.
    Repeat,
}
impl ConstInit for KeyState {
    const INIT: Self = Self::Press;
}
