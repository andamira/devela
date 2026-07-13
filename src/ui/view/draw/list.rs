// devela/src/ui/view/draw/list.rs
//
//! Defines [`UiDrawList`], [`UiDrawListView`].
//

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{PhantomData, UiDraw};

#[doc = crate::_tags!(ui data_structure lifetime)]
/// A borrowed view over an ordered UI draw sequence.
#[doc = crate::_doc_meta! { location("ui/view") }]
///
/// It preserves the painter ordering of the underlying
/// [`UiDrawList`] without taking ownership of its storage.
pub type UiDrawListView<'a, S, T = &'a str> = UiDrawList<S, T, &'a [UiDraw<S, T>]>;

#[doc = crate::_tags!(ui data_structure)]
/// An ordered UI draw sequence over caller-chosen storage.
#[doc = crate::_doc_meta! { location("ui/view") }]
///
/// The list is a backend-neutral display list in resolved logical UI space.
///
/// Records are presented from first to last. When records overlap, a later
/// record appears in front of an earlier record. This painter ordering is
/// authoritative: presenters must preserve it and must not independently
/// sort records by operation, style, or backend representation.
///
/// `B` determines ownership and storage. It may contain a borrowed slice,
/// fixed storage, an allocated vector, or another contiguous draw sequence.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiDrawList<S, T, B> {
    draws: B,
    _marker: PhantomData<fn() -> UiDraw<S, T>>,
}

impl<S, T, const LEN: usize> UiDrawList<S, T, [UiDraw<S, T>; LEN]> {
    /// Constructs a draw list from an array.
    pub const fn from_array(draws: [UiDraw<S, T>; LEN]) -> Self {
        Self::from_storage(draws)
    }
}
impl<'a, S, T> UiDrawList<S, T, &'a [UiDraw<S, T>]> {
    /// Constructs a borrowed draw list from a slice.
    pub const fn from_slice(draws: &'a [UiDraw<S, T>]) -> Self {
        Self::from_storage(draws)
    }
}

#[rustfmt::skip]
impl<S, T, B> UiDrawList<S, T, B> {
    /// Constructs a draw list over the given storage.
    pub const fn from_storage(draws: B) -> Self { Self { draws, _marker: PhantomData } }

    /// Returns the underlying storage.
    pub const fn storage(&self) -> &B { &self.draws }
    /// Returns the underlying storage as an exclusive reference.
    pub const fn storage_mut(&mut self) -> &mut B { &mut self.draws }

    /// Consumes the list and returns its storage.
    pub fn into_storage(self) -> B { self.draws }
}

#[rustfmt::skip]
impl<S, T, B: AsRef<[UiDraw<S, T>]>> UiDrawList<S, T, B> {
    /// Returns the number of draw records.
    pub fn len(&self) -> usize { self.as_slice().len() }

    /// Returns whether the draw list is empty.
    pub fn is_empty(&self) -> bool { self.as_slice().is_empty() }

    /// Returns the ordered draw records.
    pub fn as_slice(&self) -> &[UiDraw<S, T>] { self.draws.as_ref() }

    /// Returns a borrowed view preserving this list's draw order.
    pub fn as_view(&self) -> UiDrawListView<'_, S, T> { UiDrawList::from_storage(self.as_slice()) }

    /// Iterates over draw records in painter order, from back to front.
    pub fn iter(&self) -> ::core::slice::Iter<'_, UiDraw<S, T>> {
        self.as_slice().iter()
    }
}

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<S, T> UiDrawList<S, T, Vec<UiDraw<S, T>>> {
    /// Constructs an empty allocating draw list.
    pub const fn new() -> Self { Self::from_storage(Vec::new()) }

    /// Constructs an empty allocating draw list with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_storage(Vec::with_capacity(capacity))
    }

    /// Appends a draw record.
    pub fn push(&mut self, draw: UiDraw<S, T>) { self.draws.push(draw); }

    /// Clears all draw records while retaining allocated capacity.
    pub fn clear(&mut self) { self.draws.clear(); }

    /// Returns the draw capacity.
    pub fn capacity(&self) -> usize { self.draws.capacity() }
}

#[cfg(test)]
mod list_tests {
    use super::*;
    use crate::UiRect;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum Style {
        Back,
        Middle,
        Front,
    }
    fn draw(style: Style) -> UiDraw<Style, &'static str> {
        UiDraw::rect_fill(UiRect::default(), style)
    }

    #[test]
    fn preserves_painter_order() {
        let draws = [draw(Style::Back), draw(Style::Middle), draw(Style::Front)];
        let list = UiDrawList::from_storage(draws.as_slice());
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().kind().style(), &Style::Back);
        assert_eq!(iter.next().unwrap().kind().style(), &Style::Middle);
        assert_eq!(iter.next().unwrap().kind().style(), &Style::Front);
        assert!(iter.next().is_none());
    }
    #[test]
    fn borrowed_view_preserves_storage_and_order() {
        let draws = [draw(Style::Back), draw(Style::Middle), draw(Style::Front)];
        let list = UiDrawList::from_storage(draws);
        let view = list.as_view();
        assert_eq!(view.len(), 3);
        assert_eq!(view.as_slice(), list.as_slice());
        // Confirms that the view points at the same underlying sequence.
        assert!(core::ptr::eq(view.as_slice().as_ptr(), list.as_slice().as_ptr(),));
    }
    #[test]
    fn empty_and_length_queries() {
        let empty: &[UiDraw<Style, &str>] = &[];
        let empty = UiDrawList::from_storage(empty);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        let draws = [draw(Style::Back), draw(Style::Front)];
        let list = UiDrawList::from_storage(draws.as_slice());
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
    }
    #[test]
    fn into_storage_returns_original_storage() {
        let draws = [draw(Style::Back), draw(Style::Front)];
        let list = UiDrawList::from_array(draws);
        assert_eq!(list.into_storage(), draws);
    }
}
