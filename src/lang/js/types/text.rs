// devela::lang:js::types::text

#[cfg(doc)]
use crate::Js;

/// Text Metrics.
///
/// Represents the size of rendered text, measured by [`Js::measure_text`].
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct JsTextMetrics {
    /// The width of the rendered text.
    pub width: f32,
    /// The distance from the baseline to the highest point.
    pub ascent: f32,
    /// The distance from the baseline to the lowest point.
    pub descent: f32,
}

/// Full Text Metrics.
///
/// Represents the size of rendered text, measured by [`Js::measure_text_full`].
///
/// Includes all available text measurement properties.
/// - <https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct JsTextMetricsFull {
    /// The width of the rendered text.
    pub width: f32,
    /// Distance from the alignment point to the leftmost glyph edge.
    pub left: f32,
    /// Distance from the alignment point to the rightmost glyph edge.
    pub right: f32,
    /// Distance from the baseline to the highest glyph edge.
    pub ascent: f32,
    /// Distance from the baseline to the lowest glyph edge.
    pub descent: f32,
    /// The topmost possible bounding box for text.
    pub font_ascent: f32,
    /// The lowest possible bounding box for text.
    pub font_descent: f32,
    /// Height from the baseline to the top of the `em` square.
    pub em_ascent: f32,
    /// Height from the baseline to the bottom of the `em` square.
    pub em_descent: f32,
    /// Hanging baseline position.
    pub hanging_baseline: f32,
    /// Alphabetic baseline position.
    pub alphabetic_baseline: f32,
    /// Ideographic baseline position.
    pub ideographic_baseline: f32,
}
