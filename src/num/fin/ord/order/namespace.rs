// devela/src/num/fin/ord/order/namespace.rs
//
//! Defines [`Order`].
//

#[doc = crate::_tags!(num layout namespace)]
/// Unified namespace for multi-dimensional ordering schemes.
#[doc = crate::_doc_meta!{location("num/fin/ord")}]
///
/// `Order` provides a set of static functions that map discrete coordinates
/// from higher-dimensional integer spaces to a single linear index, and back,
/// using different locality-preserving strategies.
///
/// # Methods
///
/// - [Row major ordinal encodings](#row-major-ordinal-encodings)
///   - [row_major_from_2d](#method.row_major_from_2d) ([*try*](#method.row_major_try_from_2d)).
///   - [row_major_to_2d](#method.row_major_to_2d) ([*try*](#method.row_major_try_to_2d)).
///   - [row_major_from_3d](#method.row_major_from_3d) ([*try*](#method.row_major_try_from_3d)).
///   - [row_major_to_3d](#method.row_major_to_3d) ([*try*](#method.row_major_try_to_3d)).
///
/// - [Column major ordinal encodings](#column-major-ordinal-encodings)
///   - [col_major_from_2d](#method.col_major_from_2d) ([*try*](#method.col_major_try_from_2d)).
///   - [col_major_to_2d](#method.col_major_to_2d) ([*try*](#method.col_major_try_to_2d)).
///   - [col_major_from_3d](#method.col_major_from_3d) ([*try*](#method.col_major_try_from_3d)).
///   - [col_major_to_3d](#method.col_major_to_3d) ([*try*](#method.col_major_try_to_3d)).
///
/// - [N-dimensional ordinal encodings](#n-dimensional-ordinal-encodings)
///   - [grid_volume](#method.grid_volume).
///   - [row_major_strides](#method.row_major_strides).
///   - [col_major_strides](#method.col_major_strides).
///   - [row_major_from](#method.row_major_from) ([*try*](#method.row_major_try_from)).
///   - [row_major_to](#method.row_major_to) ([*try*](#method.row_major_try_to)).
///   - [col_major_from](#method.col_major_from) ([*try*](#method.col_major_try_from)).
///   - [col_major_to](#method.col_major_to) ([*try*](#method.col_major_try_to)).
#[derive(Debug)]
pub struct Order;
