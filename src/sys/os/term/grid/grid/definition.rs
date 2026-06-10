// devela::sys::os::term::grid::grid::definition
//
//! Defines [`TermGrid`].
//

use crate::{Extent2, PhantomData};

#[doc = crate::_tags!(term data_structure)]
/// A dense row-major grid of terminal-space elements.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
///
/// The grid occupies the first `width × height` elements of its storage.
/// Any remaining storage is retained but lies outside the grid.
///
/// # Methods
///
/// - [Construction and structural access](#construction-and-structural-access)
///   - [new](#method.new).
///   - [extent](#method.extent).
///   - [width](#method.width).
///   - [height](#method.height).
///   - [len](#method.len).
///   - [is_empty](#method.is_empty).
///   - [storage](#method.storage)
///     ([*into*](#method.into_storage), [*slice*](#method.storage_slice)).
///   - [as_slice](#method.as_slice).
///   - [spare_len](#method.spare_len).
///
/// - [Coordinate queries](#coordinate-queries)
///   - [contains](#method.contains).
///   - [index_of](#method.index_of).
///   - [position_of](#method.position_of).
///   - [get](#method.get) ([*xy*](#method.get_xy)).
///   - [row](#method.row).
///
/// - [Mutable access](#mutable-access)
///   - [as_mut_slice](#method.as_mut_slice).
///   - [storage_slice_mut](#method.storage_slice_mut).
///   - [get_mut](#method.get_mut) ([*xy*](#method.get_xy_mut)).
///   - [row_mut](#method.row_mut).
///   - [set](#method.set) ([*xy*](#method.set_xy)).
///
/// - [Copy-oriented operations](#copy-oriented-operations)
///   - [fill](#method.fill).
///   - [get_copy](#method.get_copy) ([*xy*](#method.get_xy_copy)).
//
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TermGrid<E, S> {
    pub(super) storage: S,
    pub(super) extent: Extent2<usize>,
    pub(super) len: usize,
    pub(super) _element: PhantomData<fn() -> E>,
}
