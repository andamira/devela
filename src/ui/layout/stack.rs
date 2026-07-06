// devela/ui/layout/stack.rs
//
//! Defines [`UiStack`].
//

use crate::{Boundary2d, Lunit, UiRect};

#[doc = crate::_tags!(layout)]
/// Cursor-like helper for sequentially taking rectangles from a region.
#[doc = crate::_doc_meta!{ location("ui/layout") }]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiStack {
    rest: UiRect,
    side: Boundary2d,
    gap: Lunit,
}
#[rustfmt::skip]
impl UiStack {
    /* constructors */

    /// Creates a stack that takes rectangles from `side`.
    ///
    /// Negative gaps are ignored.
    pub const fn new(rest: UiRect, side: Boundary2d, gap: Lunit) -> Self {
        Self { rest, side, gap: gap.positive_part() }
    }
    /// Creates a stack that takes from the top side and advances downward.
    pub const fn down(rest: UiRect) -> Self {
        Self::new(rest, Boundary2d::Top, Lunit::ZERO)
    }
    /// Creates a stack that takes from the bottom side and advances upward.
    pub const fn up(rest: UiRect) -> Self {
        Self::new(rest, Boundary2d::Bottom, Lunit::ZERO)
    }
    /// Creates a stack that takes from the left side and advances rightward.
    pub const fn right(rest: UiRect) -> Self {
        Self::new(rest, Boundary2d::Left, Lunit::ZERO)
    }
    /// Creates a stack that takes from the right side and advances leftward.
    pub const fn left(rest: UiRect) -> Self {
        Self::new(rest, Boundary2d::Right, Lunit::ZERO)
    }

    /* get */

    /// Returns the remaining rectangle.
    pub const fn rest(self) -> UiRect { self.rest }

    /// Returns the side this stack takes from.
    pub const fn side(self) -> Boundary2d { self.side }

    /// Returns the gap consumed after successful takes.
    pub const fn gap(self) -> Lunit { self.gap }

    /// Returns this stack with a different gap.
    ///
    /// Negative gaps are ignored.
    pub const fn with_gap(self, gap: Lunit) -> Self {
        Self { rest: self.rest, side: self.side, gap: gap.positive_part() }
    }

    /* take */

    /// Takes the next rectangle from the configured side.
    pub const fn take(&mut self, extent: Lunit) -> UiRect {
        match self.side {
            Boundary2d::Left => self.take_left(extent),
            Boundary2d::Right => self.take_right(extent),
            Boundary2d::Top => self.take_top(extent),
            Boundary2d::Bottom => self.take_bottom(extent),
        }
    }
    /// Takes a rectangle from the left side.
    pub const fn take_left(&mut self, width: Lunit) -> UiRect {
        let (taken, rest) = self.rest.cut_left(width);
        self.rest = rest;
        if !taken.is_empty() { self.skip_left(self.gap); }
        taken
    }
    /// Takes a rectangle from the right side.
    pub const fn take_right(&mut self, width: Lunit) -> UiRect {
        let (taken, rest) = self.rest.cut_right(width);
        self.rest = rest;
        if !taken.is_empty() { self.skip_right(self.gap); }
        taken
    }
    /// Takes a rectangle from the top side.
    pub const fn take_top(&mut self, height: Lunit) -> UiRect {
        let (taken, rest) = self.rest.cut_top(height);
        self.rest = rest;
        if !taken.is_empty() { self.skip_top(self.gap); }
        taken
    }
    /// Takes a rectangle from the bottom side.
    pub const fn take_bottom(&mut self, height: Lunit) -> UiRect {
        let (taken, rest) = self.rest.cut_bottom(height);
        self.rest = rest;
        if !taken.is_empty() { self.skip_bottom(self.gap); }
        taken
    }
    /// Takes all remaining space.
    pub const fn take_rest(&mut self) -> UiRect {
        let rest = self.rest;
        self.rest = UiRect::from_xy_wh(rest.x(), rest.y(), Lunit::ZERO, Lunit::ZERO);
        rest
    }

    /* discard */

    /// Discards space from the left side.
    pub const fn skip_left(&mut self, width: Lunit) {
        let (_, rest) = self.rest.cut_left(width);
        self.rest = rest;
    }
    /// Discards space from the right side.
    pub const fn skip_right(&mut self, width: Lunit) {
        let (_, rest) = self.rest.cut_right(width);
        self.rest = rest;
    }
    /// Discards space from the top side.
    pub const fn skip_top(&mut self, height: Lunit) {
        let (_, rest) = self.rest.cut_top(height);
        self.rest = rest;
    }
    /// Discards space from the bottom side.
    pub const fn skip_bottom(&mut self, height: Lunit) {
        let (_, rest) = self.rest.cut_bottom(height);
        self.rest = rest;
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    #[test]

    fn take_top_with_gap() {
        let root = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(100));
        let mut stack = UiStack::down(root).with_gap(Lunit::px(5));
        let title = stack.take_top(Lunit::px(20));
        let rest = stack.rest();
        assert_eq!(title.y(), Lunit::px(0));
        assert_eq!(title.h(), Lunit::px(20));
        assert_eq!(rest.y(), Lunit::px(25));
        assert_eq!(rest.h(), Lunit::px(75));
    }
    #[test]
    fn take_uses_configured_side() {
        let root = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(100));
        let mut stack = UiStack::new(root, Boundary2d::Left, Lunit::px(2));
        let sidebar = stack.take(Lunit::px(30));
        let rest = stack.rest();
        assert_eq!(sidebar.x(), Lunit::px(0));
        assert_eq!(sidebar.w(), Lunit::px(30));
        assert_eq!(rest.x(), Lunit::px(32));
        assert_eq!(rest.w(), Lunit::px(68));
    }
    #[test]
    fn empty_take_does_not_consume_gap() {
        let root = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(100));
        let mut stack = UiStack::new(root, Boundary2d::Top, Lunit::px(5));
        let taken = stack.take(Lunit::ZERO);
        let rest = stack.rest();
        assert!(taken.is_empty());
        assert_eq!(rest.y(), Lunit::px(0));
        assert_eq!(rest.h(), Lunit::px(100));
    }
}
