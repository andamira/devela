// devela_base_core::media::visual::draw::traits
//
//! Defines [`Canvas`], [`CanvasRead`], [`CanvasTextel`].
//

use crate::{Extent, Position, Region};

#[doc = crate::_tags!(image)]
/// A writable 2D drawing surface over abstract spatial units.
#[doc = crate::_doc_location!("media/visual/draw")]
///
/// `Canvas` defines the minimal common contract for placing color on a drawable
/// region, independently of backend, storage, or presentation strategy.
///
/// It does not imply readback, layout, text rendering, or window/runtime control.
pub trait Canvas {
    /// The spatial unit used by this canvas.
    ///
    /// This unit is abstract and backend-defined.
    /// It may represent pixels, text cells, or another drawable measure.
    type Unit;

    /// The semantic color value accepted by this canvas.
    ///
    /// This is a paint value, not necessarily the backend's native storage format.
    type Color;

    /// The error type returned by canvas operations.
    type Error;

    /// Returns the drawable extent of the canvas.
    fn canvas_extent(&self) -> Extent<Self::Unit, 2>;

    /// Fills the whole canvas with `color`.
    fn canvas_clear(&mut self, color: Self::Color) -> Result<(), Self::Error>;

    /// Places a single color mark at `pos`.
    ///
    /// Interpretation depends on the backend realization.
    /// For example, this may affect one pixel, one cell, or one minimal drawable unit.
    fn canvas_set_color(
        &mut self,
        pos: Position<Self::Unit, 2>,
        color: Self::Color,
    ) -> Result<(), Self::Error>;

    /// Fills `region` with `color`.
    fn canvas_fill_region(
        &mut self,
        rect: Region<Self::Unit, 2>,
        color: Self::Color,
    ) -> Result<(), Self::Error>;
}

#[doc = crate::_tags!(image)]
/// Read access to colors from a canvas-like surface.
#[doc = crate::_doc_location!("media/visual/draw")]
///
/// This is separate from [`Canvas`] because some draw targets support writing
/// but not efficient or meaningful readback.
pub trait CanvasRead: Canvas {
    /// Returns the color at `pos`.
    ///
    /// Readback may be unsupported, lossy, or backend-derived.
    fn canvas_get_color(&self, pos: Position<Self::Unit, 2>) -> Result<Self::Color, Self::Error>;
}

#[doc = crate::_tags!(image text)]
/// A [`Canvas`] that can place text render elements.
#[doc = crate::_doc_location!("media/visual/draw")]
///
/// `CanvasTextel` extends color-based drawing with discrete text-oriented marks,
/// such as terminal cells or other glyph-bearing render units.
///
/// A textel is a render artifact, not a layout primitive.
pub trait CanvasTextel: Canvas {
    /// A text render element placed on the canvas.
    ///
    /// A textel is a render artifact, not a layout unit.
    type Textel;

    /// Places a single textel at `pos`.
    fn canvas_put_textel(
        &mut self,
        pos: Position<Self::Unit, 2>,
        textel: Self::Textel,
    ) -> Result<(), Self::Error>;
}
