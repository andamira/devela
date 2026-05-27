// devela::num::grain::wide::examples
//
//! Defines [`Lane4_i32Example`].
//

use crate::lane;

lane! {
    #[doc = crate::_tags!(example code num)]
    /// Example fixed-width pack of 4 × `i32` lanes.
    #[doc = crate::_doc_meta!{location("num/grain")}]
    ///
    /// Generated with [`lane!`].
    ///
    /// # Methods
    ///
    /// - [Common methods for all integers and floating-point primitives](#common-methods-for-all-integers-and-floating-point-primitives)
    ///   - [from_byte_values](#method.from_byte_values).
    ///   - [from_slice](#method.from_slice).
    ///   - [splat](#method.splat).
    ///   - [add_assign](#method.add_assign) ([*plain*](#method.add_assign_plain),
    ///     [*simd*](#method.add_assign_simd), [*wide*](#method.add_assign_wide)).
    ///   - [sub_assign](#method.sub_assign) ([*plain*](#method.sub_assign_plain),
    ///     [*simd*](#method.sub_assign_simd), [*wide*](#method.sub_assign_wide)).
    ///   - [mul_assign](#method.mul_assign) ([*plain*](#method.mul_assign_plain),
    ///     [*simd*](#method.mul_assign_simd), [*wide*](#method.mul_assign_wide)).
    ///   - [rem_assign](#method.rem_assign) ([*plain*](#method.rem_assign_plain)).
    ///   - [div_assign](#method.div_assign) ([*plain*](#method.div_assign_plain),
    ///     [*simd*](#method.div_assign_simd)).
    ///   - [neg_assign](#method.neg_assign).
    ///   - [add_scalar_assign](#method.add_scalar_assign)
    ///     ([*plain*](#method.add_scalar_assign_plain)).
    ///   - [sub_scalar_assign](#method.sub_scalar_assign)
    ///     ([*plain*](#method.sub_scalar_assign_plain)).
    ///   - [mul_scalar_assign](#method.mul_scalar_assign)
    ///     ([*plain*](#method.mul_scalar_assign_plain)).
    ///   - [rem_scalar_assign](#method.rem_scalar_assign)
    ///     ([*plain*](#method.rem_scalar_assign_plain)).
    ///   - [div_scalar_assign](#method.div_scalar_assign)
    ///     ([*plain*](#method.div_scalar_assign_plain)).
    ///   - [min](#method.min) ([*plain*](#method.min_plain), [*wide*](#method.min_wide)).
    ///   - [max](#method.max) ([*plain*](#method.max_plain), [*wide*](#method.max_wide)).
    ///   - [clamp_assign](#method.clamp_assign) ([*plain*](#method.clamp_assign_plain)).
    ///   - [min_reduce](#method.min_reduce) ([*plain*](#method.min_reduce_plain)).
    ///   - [max_reduce](#method.max_reduce) ([*plain*](#method.max_reduce_plain)).
    ///   - [sum](#method.sum) ([*plain*](#method.sum_plain)).
    ///
    /// - [Methods for integer primitives](#methods-for-integer-primitives)
    ///   - [div_assign](#method.div_assign).
    ///   - [neg_assign_plain](#method.neg_assign_plain).
    ///   - [saturating_add_assign_plain](#method.saturating_add_assign_plain).
    ///   - [wrapping_add_assign_plain](#method.wrapping_add_assign_plain).
    ///   - [saturating_sub_assign_plain](#method.saturating_sub_assign_plain).
    ///   - [wrapping_sub_assign_plain](#method.wrapping_sub_assign_plain).
    ///   - [wrapping_mul_assign_plain](#method.wrapping_mul_assign_plain).
    ///   - [bitand_assign](#method.bitand_assign) ([*plain*](#method.bitand_assign_plain),
    ///     [*simd*](#method.bitand_assign_simd), [*wide*](#method.bitand_assign_wide)).
    ///   - [bitor_assign](#method.bitor_assign) ([*plain*](#method.bitor_assign_plain),
    ///     [*simd*](#method.bitor_assign_simd), [*wide*](#method.bitor_assign_wide)).
    ///   - [bitxor_assign](#method.bitxor_assign) ([*plain*](#method.bitxor_assign_plain),
    ///     [*simd*](#method.bitxor_assign_simd), [*wide*](#method.bitxor_assign_wide)).
    ///   - [shl_assign](#method.shl_assign) ([*plain*](#method.shl_assign_plain),
    ///     [*simd*](#method.shl_assign_simd), [*wide*](#method.shl_assign_wide)).
    ///   - [shr_assign](#method.shr_assign) ([*plain*](#method.shr_assign_plain),
    ///     [*simd*](#method.shr_assign_simd), [*wide*](#method.shr_assign_wide)).
    ///   - [min_simd](#method.min_simd).
    ///   - [max_simd](#method.max_simd).
    ///
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(non_camel_case_types)]
    pub struct Lane4_i32Example pub lanes(4); unsigned(i32);
}
