// devela/ui/layout/metric.rs
//
//! Metric aliases: [`UiExt`], [`UiPos`], [`UiRect`], [`UiStride`].
//
// Boundary: these aliases specialize generic geometry for logical UI space;
// conversion into backend output units happens after layout.
//

use crate::{Extent2, Lunit, Position2, RegionS2, Stride2};

#[doc = crate::_tags!(ui layout)]
/// A 2-dimensional layout extent.
#[doc = crate::_doc_meta!{ location("ui/layout") }]
pub type UiExt = Extent2<Lunit>;

#[doc = crate::_tags!(ui layout)]
/// A 2-dimensional layout position.
#[doc = crate::_doc_meta!{ location("ui/layout") }]
pub type UiPos = Position2<Lunit>;

#[doc = crate::_tags!(ui layout)]
/// A 2-dimensional layout stride.
#[doc = crate::_doc_meta!{ location("ui/layout") }]
pub type UiStride = Stride2<Lunit>;

#[doc = crate::_tags!(ui layout)]
/// A 2-dimensional layout region.
#[doc = crate::_doc_meta!{ location("ui/layout") }]
pub type UiRect = RegionS2<Lunit>;

#[rustfmt::skip]
impl UiRect {
    /// Returns the left edge position.
    pub const fn left(self) -> Lunit { self.x() }
    /// Returns the top edge position.
    pub const fn top(self) -> Lunit { self.y() }

    /// Returns the right edge position, saturating on overflow.
    pub const fn right(self) -> Lunit { self.x().sat_add(self.w()) }
    /// Returns the bottom edge position, saturating on overflow.
    pub const fn bottom(self) -> Lunit { self.y().sat_add(self.h()) }

    /// Returns whether the rectangle has no positive area.
    pub const fn is_empty(self) -> bool {
        self.w().positive_part().eq(Lunit::ZERO)
            || self.h().positive_part().eq(Lunit::ZERO)
    }

    /// Returns this rectangle inset equally on all sides.
    ///
    /// Negative insets are ignored. If the inset exceeds the rectangle extent,
    /// the resulting extent is clamped to zero.
    pub const fn inset(self, all: Lunit) -> Self {
        self.inset_ltrb(all, all, all, all)
    }
    /// Returns this rectangle inset horizontally and vertically.
    ///
    /// Negative insets are ignored. If an inset exceeds the rectangle extent,
    /// the resulting extent is clamped to zero.
    pub const fn inset_xy(self, x: Lunit, y: Lunit) -> Self {
        self.inset_ltrb(x, y, x, y)
    }

    /// Returns this rectangle inset by left, top, right, and bottom margins.
    ///
    /// Negative insets are ignored. The resulting rectangle is kept inside the
    /// original rectangle and its extent is clamped to zero.
    pub const fn inset_ltrb(self, left: Lunit, top: Lunit, right: Lunit, bottom: Lunit) -> Self {
        let w = self.w().positive_part();
        let h = self.h().positive_part();
        let l = left.positive_part().min(w);
        let t = top.positive_part().min(h);
        let w_after_l = w.sub_floor_zero(l);
        let h_after_t = h.sub_floor_zero(t);
        let r = right.positive_part().min(w_after_l);
        let b = bottom.positive_part().min(h_after_t);
        Self::from_xy_wh(
            self.x().sat_add(l),
            self.y().sat_add(t),
            w_after_l.sub_floor_zero(r),
            h_after_t.sub_floor_zero(b),
        )
    }

    /// Cuts a rectangle from the left side, returning `(taken, rest)`.
    ///
    /// Negative widths take nothing. Widths larger than the rectangle are
    /// clamped to the rectangle width.
    pub const fn cut_left(self, width: Lunit) -> (Self, Self) {
        let w = self.w().positive_part();
        let h = self.h().positive_part();
        let used = width.positive_part().min(w);
        let rest_w = w.sub_floor_zero(used);
        let taken = Self::from_xy_wh(self.x(), self.y(), used, h);
        let rest = Self::from_xy_wh(self.x().sat_add(used), self.y(), rest_w, h);
        (taken, rest)
    }
    /// Cuts a rectangle from the right side, returning `(taken, rest)`.
    ///
    /// Negative widths take nothing. Widths larger than the rectangle are
    /// clamped to the rectangle width.
    pub const fn cut_right(self, width: Lunit) -> (Self, Self) {
        let w = self.w().positive_part();
        let h = self.h().positive_part();
        let used = width.positive_part().min(w);
        let rest_w = w.sub_floor_zero(used);
        let rest = Self::from_xy_wh(self.x(), self.y(), rest_w, h);
        let taken = Self::from_xy_wh(self.x().sat_add(rest_w), self.y(), used, h);
        (taken, rest)
    }
    /// Cuts a rectangle from the top side, returning `(taken, rest)`.
    ///
    /// Negative heights take nothing. Heights larger than the rectangle are
    /// clamped to the rectangle height.
    pub const fn cut_top(self, height: Lunit) -> (Self, Self) {
        let w = self.w().positive_part();
        let h = self.h().positive_part();
        let used = height.positive_part().min(h);
        let rest_h = h.sub_floor_zero(used);
        let taken = Self::from_xy_wh(self.x(), self.y(), w, used);
        let rest = Self::from_xy_wh(self.x(), self.y().sat_add(used), w, rest_h);
        (taken, rest)
    }
    /// Cuts a rectangle from the bottom side, returning `(taken, rest)`.
    ///
    /// Negative heights take nothing. Heights larger than the rectangle are
    /// clamped to the rectangle height.
    pub const fn cut_bottom(self, height: Lunit) -> (Self, Self) {
        let w = self.w().positive_part();
        let h = self.h().positive_part();
        let used = height.positive_part().min(h);
        let rest_h = h.sub_floor_zero(used);
        let rest = Self::from_xy_wh(self.x(), self.y(), w, rest_h);
        let taken = Self::from_xy_wh(self.x(), self.y().sat_add(rest_h), w, used);
        (taken, rest)
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn edges() {
        let r = UiRect::from_xy_wh(Lunit::px(10), Lunit::px(20), Lunit::px(30), Lunit::px(40));
        assert_eq!(r.left(), Lunit::px(10));
        assert_eq!(r.top(), Lunit::px(20));
        assert_eq!(r.right(), Lunit::px(40));
        assert_eq!(r.bottom(), Lunit::px(60));
    }
    #[test]
    fn empty() {
        assert!(UiRect::from_xy_wh(Lunit::ZERO, Lunit::ZERO, Lunit::ZERO, Lunit::px(1)).is_empty());
        assert!(UiRect::from_xy_wh(Lunit::ZERO, Lunit::ZERO, Lunit::px(1), Lunit::ZERO).is_empty());
        assert!(
            !UiRect::from_xy_wh(Lunit::ZERO, Lunit::ZERO, Lunit::px(1), Lunit::px(1)).is_empty()
        );
    }
    #[test]
    fn inset() {
        let r = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(50));
        let inner = r.inset(Lunit::px(10));
        assert_eq!(inner.x(), Lunit::px(10));
        assert_eq!(inner.y(), Lunit::px(10));
        assert_eq!(inner.w(), Lunit::px(80));
        assert_eq!(inner.h(), Lunit::px(30));
    }
    #[test]
    fn over_inset_clamps_to_empty() {
        let r = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(10), Lunit::px(10));
        let inner = r.inset(Lunit::px(100));
        assert!(inner.is_empty());
        assert_eq!(inner.w(), Lunit::ZERO);
        assert_eq!(inner.h(), Lunit::ZERO);
    }
    #[test]
    fn cut_left() {
        let r = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(20));
        let (left, rest) = r.cut_left(Lunit::px(30));
        assert_eq!(left.x(), Lunit::px(0));
        assert_eq!(left.w(), Lunit::px(30));
        assert_eq!(rest.x(), Lunit::px(30));
        assert_eq!(rest.w(), Lunit::px(70));
    }
    #[test]
    fn cut_right() {
        let r = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(20));
        let (right, rest) = r.cut_right(Lunit::px(30));
        assert_eq!(right.x(), Lunit::px(70));
        assert_eq!(right.w(), Lunit::px(30));
        assert_eq!(rest.x(), Lunit::px(0));
        assert_eq!(rest.w(), Lunit::px(70));
    }
    #[test]
    fn over_cut_clamps() {
        let r = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(10), Lunit::px(10));
        let (taken, rest) = r.cut_left(Lunit::px(100));
        assert_eq!(taken.w(), Lunit::px(10));
        assert_eq!(rest.w(), Lunit::ZERO);
    }
    #[test]
    fn cut_top() {
        let r = UiRect::from_xy_wh(Lunit::px(0), Lunit::px(0), Lunit::px(100), Lunit::px(50));
        let (top, rest) = r.cut_top(Lunit::px(20));
        assert_eq!(top.y(), Lunit::px(0));
        assert_eq!(top.h(), Lunit::px(20));
        assert_eq!(rest.y(), Lunit::px(20));
        assert_eq!(rest.h(), Lunit::px(30));
    }
}
