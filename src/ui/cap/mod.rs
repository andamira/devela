// devela::ui::cap
//
//!
//

#[cfg(doc)]
use super::UiService;

mod color;
mod input;
mod window;
pub use {color::*, input::*, window::*};

/// The capabilities supported by a [`UiService`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UiCap {
    /// Color capabilities.
    pub color: Option<UiCapColor>,

    /// Input capabilities.
    pub input: Option<UiCapInput>,

    /// Windowing capabilities.
    pub window: Option<UiCapWindow>,
}
