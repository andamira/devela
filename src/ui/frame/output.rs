// devela/src/ui/frame/output.rs
//
//! Defines [`UiOutput`], [`UiOutputView`].
//

use crate::{LayoutReceipt, UiDraw, UiDrawList, UiDrawListView, UiView};

#[doc = crate::_tags!(ui)]
/// A completed backend-neutral UI frame over caller-chosen storage.
#[doc = crate::_doc_meta! { location("ui/frame") }]
/// Aggregates the spatial, visual, and drawing records produced during one UI frame.
///
/// The three streams have distinct roles:
/// - [`LayoutReceipt`] records resolved layout assignment.
/// - [`UiView`] records visual identity, geometry, ordering, and flags.
/// - [`UiDrawList`] contains the final painter-ordered presentation sequence.
///
/// `UiOutput` is a frame artifact, not the mutable [`UiFrame`]
/// authorship context and not a concrete presentation surface.
///
/// `L`, `V`, and `D` determine ownership and storage. They may contain borrowed
/// slices, fixed storage, allocated vectors, or other suitable representations.
///
/// [`UiFrame`]: crate::UiFrame
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiOutput<L, V, D> {
    layouts: L,
    views: V,
    draws: D,
}

#[rustfmt::skip]
impl<L, V, D> UiOutput<L, V, D> {
    /// Constructs a UI output from its frame record streams.
    pub const fn from_parts(layouts: L, views: V, draws: D) -> Self {
        Self { layouts, views, draws }
    }

    /// Returns the underlying layout-record storage.
    pub const fn layout_storage(&self) -> &L { &self.layouts }
    /// Returns the underlying view-record storage.
    pub const fn view_storage(&self) -> &V { &self.views }
    /// Returns the underlying draw-list storage.
    pub const fn draw_list(&self) -> &D { &self.draws }

    /// Consumes this output and returns its three frame record streams.
    pub fn into_parts(self) -> (L, V, D) { (self.layouts, self.views, self.draws) }

    /// Consumes this output and returns its three frame record streams.
    pub const fn into_parts_const(self) -> (L, V, D) where Self: Copy {
        (self.layouts, self.views, self.draws)
    }
}

#[rustfmt::skip]
impl<L, V, S, T, B> UiOutput<L, V, UiDrawList<S, T, B>>
where
    L: AsRef<[LayoutReceipt]>,
    V: AsRef<[UiView]>,
    B: AsRef<[UiDraw<S, T>]>,
{
    /// Returns this UI output as a borrowed read-only frame view.
    pub fn as_view(&self) -> UiOutputView<'_, S, T> {
        UiOutputView::from_parts(self.layouts.as_ref(), self.views.as_ref(), self.draws.as_view())
    }
    /// Returns the layout records.
    pub fn layouts(&self) -> &[LayoutReceipt] { self.layouts.as_ref() }
    /// Returns the visual records.
    pub fn views(&self) -> &[UiView] { self.views.as_ref() }
}

#[doc = crate::_tags!(ui lifetime)]
/// A borrowed read-only view over a completed backend-neutral UI frame.
#[doc = crate::_doc_meta! { location("ui/frame") }]
///
/// Borrows the frame record streams without taking ownership of their storage.
///
/// This is the canonical form intended for backend-neutral presenters.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiOutputView<'a, S, T = &'a str> {
    layouts: &'a [LayoutReceipt],
    views: &'a [UiView],
    draws: UiDrawListView<'a, S, T>,
}

#[rustfmt::skip]
impl<'a, S, T> UiOutputView<'a, S, T> {
    /// Constructs a borrowed output view from its three frame record streams.
    pub const fn from_parts(
        layouts: &'a [LayoutReceipt],
        views: &'a [UiView],
        draws: UiDrawListView<'a, S, T>,
    ) -> Self {
        Self { layouts, views, draws }
    }
    /// Constructs a borrowed output view directly from record slices.
    pub const fn from_slices(
        layouts: &'a [LayoutReceipt],
        views: &'a [UiView],
        draws: &'a [UiDraw<S, T>],
    ) -> Self {
        Self::from_parts(layouts, views, UiDrawList::from_slice(draws))
    }
    /// Returns the layout records.
    pub const fn layouts(&self) -> &'a [LayoutReceipt] { self.layouts }
    /// Returns the visual records.
    pub const fn views(&self) -> &'a [UiView] { self.views }
    /// Returns the painter-ordered draw list.
    pub const fn draw_list(&self) -> &UiDrawListView<'a, S, T> { &self.draws }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{Ptr, UiRect};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    enum Style {
        Background,
        Foreground,
    }
    fn draw(style: Style) -> UiDraw<Style> {
        UiDraw::rect_fill(UiRect::default(), style)
    }

    #[test]
    fn constructs_from_parts() {
        let layouts = [LayoutReceipt::default()];
        let views = [UiView::default()];
        let draws = UiDrawList::from_array([draw(Style::Background), draw(Style::Foreground)]);
        let output = UiOutput::from_parts(layouts, views, draws);
        assert_eq!(output.layout_storage().len(), 1);
        assert_eq!(output.view_storage().len(), 1);
        assert_eq!(output.draw_list().len(), 2);
    }
    #[test]
    fn borrowed_view_preserves_all_streams() {
        let output = UiOutput::from_parts(
            [LayoutReceipt::default()],
            [UiView::default()],
            UiDrawList::from_array([draw(Style::Background), draw(Style::Foreground)]),
        );
        let view = output.as_view();
        assert_eq!(view.layouts(), output.layout_storage().as_slice());
        assert_eq!(view.views(), output.view_storage().as_slice());
        assert_eq!(view.draw_list().as_slice(), output.draw_list().as_slice(),);
        assert!(Ptr::eq(view.layouts().as_ptr(), output.layout_storage().as_ptr(),));
        assert!(Ptr::eq(view.views().as_ptr(), output.view_storage().as_ptr(),));
        assert!(Ptr::eq(
            view.draw_list().as_slice().as_ptr(),
            output.draw_list().as_slice().as_ptr(),
        ));
    }
    #[test]
    fn constructs_view_directly_from_slices() {
        let layouts = [LayoutReceipt::default()];
        let views = [UiView::default()];
        let draws = [draw(Style::Background), draw(Style::Foreground)];
        let output = UiOutputView::from_slices(&layouts, &views, &draws);
        assert_eq!(output.layouts(), &layouts);
        assert_eq!(output.views(), &views);
        assert_eq!(output.draw_list().as_slice(), &draws);
    }
    #[test]
    fn into_parts_returns_original_storage() {
        let layouts = [LayoutReceipt::default()];
        let views = [UiView::default()];
        let draws = UiDrawList::from_array([draw(Style::Background)]);
        let output = UiOutput::from_parts(layouts, views, draws);
        let (actual_layouts, actual_views, actual_draws) = output.into_parts();
        assert_eq!(actual_layouts, layouts);
        assert_eq!(actual_views, views);
        assert_eq!(actual_draws.as_slice(), draws.as_slice());
    }
}
