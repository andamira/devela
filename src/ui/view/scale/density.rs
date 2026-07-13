// devela/ui/view/scale/density.rs
//
//! Defines [`UiDensity`].
//

use crate::UiRound::{self, Ceil, Floor, Inward, Nearest, Outward};
use crate::{Cmp, Lunit, RatioU32, RegionS2, UiNum, UiRect, unwrap};

#[doc = crate::_tags!(ui quant)]
/// Physical output pixels per logical UI pixel.
#[doc = crate::_doc_meta! {
    location("ui/view/scale"),
    test_size_of(UiDensity = 8; niche Option),
}]
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UiDensity {
    scale: RatioU32,
}
impl Default for UiDensity {
    fn default() -> Self {
        Self::ONE
    }
}

#[rustfmt::skip]
impl UiDensity {
    /// Identity density: one physical pixel per logical pixel.
    pub const ONE: Self = Self::new_unchecked(RatioU32::ONE);

    /// Android-like low density: `0.5x`.
    pub const LDPI: Self = Self::new_unchecked(unwrap![some RatioU32::new(1, 2)]);

    /// Android-like medium density: `1x`.
    pub const MDPI: Self = Self::ONE;

    /// Android-like high density: `1.5x`.
    pub const HDPI: Self = Self::new_unchecked(unwrap![some RatioU32::new(3, 2)]);

    /// Android-like extra-high density: `2x`.
    pub const XHDPI: Self = Self::new_unchecked(unwrap![some RatioU32::new(2, 1)]);

    /// Android-like extra-extra-high density: `3x`.
    pub const XXHDPI: Self = Self::new_unchecked(unwrap![some RatioU32::new(3, 1)]);

    /// Android-like extra-extra-extra-high density: `4x`.
    pub const XXXHDPI: Self = Self::new_unchecked(unwrap![some RatioU32::new(4, 1)]);

    const fn new_unchecked(scale: RatioU32) -> Self { Self { scale } }
}
#[rustfmt::skip]
impl UiDensity {
    /* constructors */

    /// Constructs a density from a nonzero scale ratio.
    ///
    /// Returns `None` if `scale == 0`.
    pub const fn new(scale: RatioU32) -> Option<Self> {
        if scale.is_zero() { None } else { Some(Self { scale }) }
    }

    /// Constructs from physical output pixels per logical UI pixel.
    pub const fn from_ratio(num: u32, den: u32) -> Option<Self> {
        let Some(scale) = RatioU32::new(num, den) else { return None };
        Self::new(scale)
    }
    /// Constructs from Android-like DPI using `160 dpi` as `1x`.
    pub const fn from_dpi_160(dpi: u32) -> Option<Self> { Self::from_ratio(dpi, 160) }

    /* queries */

    /// Returns the scale ratio.
    pub const fn scale(self) -> RatioU32 { self.scale }

    /// Returns `(physical_px, logical_px)`.
    pub const fn num_den(self) -> (u32, u32) { self.scale.num_den() }

    /* projections */

    /// Projects a logical layout scalar to integer physical output pixels.
    ///
    /// `Inward` and `Outward` are rectangle policies; for scalar projection
    /// they behave like `Nearest`.
    pub const fn lunit_to_px(self, value: Lunit, round: UiRound) -> i32 {
        let quanta = value.quanta() as i64;
        let (num, den) = self.scale.num_den();
        let n = quanta.saturating_mul(num as i64);
        let d = (Lunit::QUANTA_PER_LOGICAL_PX as i64).saturating_mul(den as i64);
        UiNum::round_div_scalar_i64_to_i32(n, d, round)
    }

    /// Projects integer physical output pixels back into logical layout space.
    ///
    /// `Inward` and `Outward` are rectangle policies; for scalar projection
    /// they behave like `Nearest`.
    pub const fn px_to_lunit(self, px: i32, round: UiRound) -> Lunit {
        let (num, den) = self.scale.num_den();
        let n = (px as i64)
            .saturating_mul(Lunit::QUANTA_PER_LOGICAL_PX as i64)
            .saturating_mul(den as i64);
        let q = UiNum::div_round_scalar_i64(n, num as i64, round);
        Lunit::new_saturated_up(q)
    }

    /// Projects a logical UI rectangle to a physical output pixel rectangle.
    pub const fn rect_to_px(self, rect: UiRect, round: UiRound) -> RegionS2<i32> {
        match round {
            Outward => {
                let x0 = self.lunit_to_px(rect.left(), Floor);
                let y0 = self.lunit_to_px(rect.top(), Floor);
                let x1 = self.lunit_to_px(rect.right(), Ceil);
                let y1 = self.lunit_to_px(rect.bottom(), Ceil);
                RegionS2::from_xy_wh(x0, y0,
                    Cmp(x1.saturating_sub(x0)).max(0),
                    Cmp(y1.saturating_sub(y0)).max(0),
                )
            }
            Inward => {
                let x0 = self.lunit_to_px(rect.left(), Ceil);
                let y0 = self.lunit_to_px(rect.top(), Ceil);
                let x1 = self.lunit_to_px(rect.right(), Floor);
                let y1 = self.lunit_to_px(rect.bottom(), Floor);
                RegionS2::from_xy_wh(x0, y0,
                    Cmp(x1.saturating_sub(x0)).max(0),
                    Cmp(y1.saturating_sub(y0)).max(0),
                )
            }
            Floor | Ceil | Nearest => {
                RegionS2::from_xy_wh(
                    self.lunit_to_px(rect.x(), round),
                    self.lunit_to_px(rect.y(), round),
                    Cmp(self.lunit_to_px(rect.w(), round)).max(0),
                    Cmp(self.lunit_to_px(rect.h(), round)).max(0),
                )
            }
        }
    }

    /// Projects a physical output pixel rectangle back into logical UI space.
    pub const fn px_to_rect(self, rect: RegionS2<i32>, round: UiRound) -> UiRect {
        let (rx, ry) = (rect.x(), rect.y());
        let (rr, rb) = (rect.x().saturating_add(rect.w()), rect.y().saturating_add(rect.h()));
        match round {
            Outward => {
                let (x0, y0) = (self.px_to_lunit(rx, Floor), self.px_to_lunit(ry, Floor));
                let (x1, y1) = (self.px_to_lunit(rr, Ceil), self.px_to_lunit(rb, Ceil));
                UiRect::from_xy_wh(x0, y0, x1.sub_floor_zero(x0), y1.sub_floor_zero(y0))
            }
            Inward => {
                let x0 = self.px_to_lunit(rx, Ceil);
                let y0 = self.px_to_lunit(ry, Ceil);
                let x1 = self.px_to_lunit(rr, Floor);
                let y1 = self.px_to_lunit(rb, Floor);
                UiRect::from_xy_wh(x0, y0,
                    x1.sub_floor_zero(x0),
                    y1.sub_floor_zero(y0),
                )
            }
            Floor | Ceil | Nearest => {
                UiRect::from_xy_wh(
                    self.px_to_lunit(rect.x(), round),
                    self.px_to_lunit(rect.y(), round),
                    self.px_to_lunit(rect.w(), round).max_zero(),
                    self.px_to_lunit(rect.h(), round).max_zero(),
                )
            }
        }
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn projects_lunit_to_px() {
        let d = UiDensity::ONE;
        assert_eq!(d.lunit_to_px(Lunit::px(10), UiRound::Nearest), 10);
        assert_eq!(d.lunit_to_px(Lunit::px_frac(1, 2), UiRound::Floor), 0);
        assert_eq!(d.lunit_to_px(Lunit::px_frac(1, 2), UiRound::Ceil), 1);
    }
    #[test]
    fn projects_ratio() {
        let d = UiDensity::from_ratio(3, 2).unwrap();
        assert_eq!(d.lunit_to_px(Lunit::px(10), UiRound::Nearest), 15);
        assert_eq!(d.px_to_lunit(15, UiRound::Nearest), Lunit::px(10));
    }
    #[test]
    fn rect_outward_covers_fractional_edges() {
        let d = UiDensity::from_ratio(3, 2).unwrap();
        let r = UiRect::from_xy_wh(
            Lunit::px_frac(1, 2),
            Lunit::px_frac(1, 2),
            Lunit::px(10),
            Lunit::px(10),
        );
        let px = d.rect_to_px(r, UiRound::Outward);
        assert_eq!(px.x(), 0);
        assert_eq!(px.y(), 0);
        assert!(px.w() >= 15);
        assert!(px.h() >= 15);
    }
}
