// devela::ui::event::pointer
//
//! Defines [`EventWheel`] [`EventWheelUnit`].
//

use crate::{_impl_init, EventButtons, Position2};

#[doc = crate::_tags!(event interaction)]
/// Represents a normalized wheel-scroll event.
#[doc = crate::_doc_location!("ui/event")]
///
/// Carries signed horizontal and vertical scroll deltas at a cursor position,
/// expressed in the semantic [`unit`][Self::unit].
///
/// Normalized wheel deltas follow UI axes:
/// - positive `delta_x` scrolls right
/// - negative `delta_x` scrolls left
/// - positive `delta_y` scrolls down
/// - negative `delta_y` scrolls up
///
/// The sign describes semantic scroll direction in UI space after backend
/// normalization. It does not describe the backend-native wheel encoding or the
/// physical wheel rotation directly.
///
/// A single event may carry both horizontal and vertical motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventWheel {
    /// The normalized horizontal scroll delta.
    pub delta_x: i32,
    /// The normalized vertical scroll delta.
    pub delta_y: i32,
    /// The semantic unit of the delta fields.
    pub unit: EventWheelUnit,
    /// The y-coordinate of the cursor during the event.
    pub x: i32,
    /// The x-coordinate of the cursor during the event.
    pub y: i32,
    /// A bitmask of currently pressed buttons.
    pub buttons: EventButtons,
}
_impl_init! { ConstInit:
    Self::new(0, 0, EventWheelUnit::INIT, 0, 0, EventButtons::INIT) => EventWheel
}
#[rustfmt::skip]
impl EventWheel {
    /// Returns a normalized wheel-scroll event.
    pub const fn new(delta_x: i32, delta_y: i32, unit: EventWheelUnit,
        x: i32, y: i32, buttons: EventButtons) -> Self {
        Self { delta_x, delta_y, unit, x, y, buttons }
    }

    /// Returns both normalized scroll deltas as `(x, y)`.
    #[must_use]
    pub const fn delta(&self) -> (i32, i32) { (self.delta_x, self.delta_y) }
    /// Returns the cursor position as `(x, y)`.
    pub const fn pos(&self) -> Position2<i32> { Position2::new([self.x, self.y]) }
    /// Returns the cursor position as `(x, y)`.
    pub const fn xy(&self) -> (i32, i32) { (self.x, self.y) }

    /* */

    /// Returns `true` if there is horizontal wheel motion.
    #[must_use]
    pub const fn has_x(&self) -> bool { self.delta_x != 0 }
    /// Returns `true` if there is vertical wheel motion.
    #[must_use]
    pub const fn has_y(&self) -> bool { self.delta_y != 0 }
    /// Returns `true` if both deltas are zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool { self.delta_x == 0 && self.delta_y == 0 }

    /// Returns `true` if the vertical delta is upward.
    #[must_use]
    pub const fn is_up(&self) -> bool { self.delta_y < 0 }
    /// Returns `true` if the vertical delta is downward.
    #[must_use]
    pub const fn is_down(&self) -> bool { self.delta_y > 0 }
    /// Returns `true` if the horizontal delta is rightward.
    #[must_use]
    pub const fn is_right(&self) -> bool { self.delta_x > 0 }
    /// Returns `true` if the horizontal delta is leftward.
    #[must_use]
    pub const fn is_left(&self) -> bool { self.delta_x < 0 }

    /// Returns `true` if the unit is [`EventWheelUnit::Step`].
    #[must_use]
    pub const fn is_step(&self) -> bool { matches!(self.unit, EventWheelUnit::Step) }
    /// Returns `true` if the unit is [`EventWheelUnit::Pixel`].
    #[must_use]
    pub const fn is_pixel(&self) -> bool { matches!(self.unit, EventWheelUnit::Pixel) }
    /// Returns `true` if the unit is [`EventWheelUnit::Line`].
    #[must_use]
    pub const fn is_line(&self) -> bool { matches!(self.unit, EventWheelUnit::Line) }
    /// Returns `true` if the unit is [`EventWheelUnit::Page`].
    #[must_use]
    pub const fn is_page(&self) -> bool { matches!(self.unit, EventWheelUnit::Page) }

}

#[doc = crate::_tags!(event interaction)]
/// The semantic unit carried by an [`EventWheel`].
#[doc = crate::_doc_location!("ui/event")]
///
/// This describes the meaning of `delta_x` and `delta_y` after backend normalization.
///
///
/// # Notes
/// - [`Step`][Self::Step] is the default and represents
///   discrete wheel notches or equivalent semantic steps.
/// - [`Pixel`][Self::Pixel], [`Line`][Self::Line], and [`Page`][Self::Page]
///   preserve richer backend units when available.
/// - Backends should translate their native wheel representation inward to one
///   of these units.
///
/// # Backend notes
/// - **X11** wheel pseudo-buttons map naturally to [`Step`][Self::Step].
/// - **Terminal** wheel reporting also maps naturally to [`Step`][Self::Step].
/// - **Web** may report wheel deltas in pixel, line, or page units.
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventWheelUnit {
    /// Discrete semantic wheel steps.
    #[default]
    Step,
    /// Pixel-based wheel deltas.
    Pixel,
    /// Line-based wheel deltas.
    Line,
    /// Page-based wheel deltas.
    Page,
}
_impl_init! { ConstInit: Self::Step => EventWheelUnit }
impl EventWheelUnit {
    /// Converts a web wheel-unit code into `EventWheelUnit`.
    pub const fn from_web(code: u8) -> Self {
        match code {
            0 => Self::Pixel,
            1 => Self::Line,
            2 => Self::Page,
            _ => Self::Step,
        }
    }
    /// Converts `EventWheelUnit` into a web wheel-unit code.
    ///
    /// `Step` is mapped to `Line` as the best semantic fallback for the web side.
    pub const fn to_web(self) -> u8 {
        match self {
            Self::Pixel => 0,
            Self::Line => 1,
            Self::Page => 2,
            Self::Step => 1,
        }
    }
}
