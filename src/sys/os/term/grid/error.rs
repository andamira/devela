// devela::sys::os::term::grid::error
//
//! Defines [`TermGridError`].
//

use crate::{NotEnoughSpace, TermColor};

#[doc = crate::_tags!(term data_structure error)]
/// Terminal grid construction, access, and rendering error.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TermGridError {
    /// The grid extent overflows the index representation.
    ExtentOverflow,

    /// The storage has fewer elements than the grid requires.
    NotEnoughElements {
        /// The required number of elements.
        required: usize,
        /// The available number of elements.
        available: usize,
    },

    /// The renderer's byte-frame storage has insufficient free capacity.
    RenderSpace(NotEnoughSpace),
    /// The positioned grid exceeds the renderer's coordinate representation.
    RenderPositionOverflow,

    /// A terminal color requires composition before ANSI encoding.
    UnresolvedColor {
        /// Whether the unresolved color is the foreground color.
        foreground: bool,
        /// The unresolved color.
        color: TermColor,
    },
    // UnsupportedStyle(TermStyleExt)
    // InvalidOccupancy(_)
    // InvalidTextel(_)
}
crate::impl_trait! {
    fmt::Display+Error for TermGridError |self, f| match self {
        Self::ExtentOverflow => f.write_str("TermGrid extent overflows the index representation"),
        Self::NotEnoughElements { required, available } =>
            write!(f, "TermGrid requires {required} elements, but only {available} are available"),
        Self::RenderSpace(err) => write!(f, "insufficient terminal renderer space: {err}"),
        Self::RenderPositionOverflow =>
            f.write_str("positioned TermGrid exceeds the terminal coordinate representation"),
        Self::UnresolvedColor { foreground: fg, color } => {
            if *fg { write!(f, "TermGrid rendering can't resolve the foreground {color:?}") }
            else { write!(f, "TermGrid rendering can't resolve the background {color:?}") }
        }
    }
}
impl TermGridError {
    /// Creates an insufficient-element error.
    pub const fn not_enough_elements(required: usize, available: usize) -> Self {
        Self::NotEnoughElements { required, available }
    }
    /// Creates an unresolved-color rendering error.
    pub const fn unresolved_color(foreground: bool, color: TermColor) -> Self {
        Self::UnresolvedColor { foreground, color }
    }
}
impl From<NotEnoughSpace> for TermGridError {
    fn from(err: NotEnoughSpace) -> TermGridError {
        Self::RenderSpace(err)
    }
}
