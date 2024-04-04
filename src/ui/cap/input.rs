// devela::ui::cap::input

/// Input capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiCapInput {
    /// Gamepad input capabilities.
    pub gamepad: bool,

    /// Keyboard input capabilities.
    pub keyboard: bool,

    /// Midi input capabilities
    pub midi: bool,

    /// Mouse input capabilities.
    pub mouse: bool,

    /// Touchscreen input capabilities.
    pub touchscreen: bool,
}
