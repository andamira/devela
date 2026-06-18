// devela/src/sys/mem/view/slice/namespace/mod.rs
//
//! Defines [`Slice`].
//

#[doc = crate::_tags!(lifetime namespace)]
/// Slice-related operations, most of them *const*.
#[doc = crate::_doc_meta!{location("sys/mem/view")}]
///
/// It is designed as a utility namespace and does not hold or wrap data itself.
/// Instead, it operates on slices provided directly as arguments to its static methods.
///
/// # Methods
///
/// - [core slice operations](#core-slice-operations)
///   - [clone](#method.clone)
///   - [copy](#method.copy)
///   - [get](#method.get) ([*mut*](#method.get_mut))
///   - [from_ref](#method.from_ref) ([*mut*](#method.from_mut))
///   - [from_raw_parts](#method.from_raw_parts)<sup title="unsafe method">⚠</sup>
///     ([*mut*](#method.from_raw_parts_mut)<sup title="unsafe method">⚠</sup>)
///
/// - [`range*` API methods](#range-api-methods-for-subslicing):<br/>
///   - [range_to](#method.range_to)
///    ([*checked*](#method.range_to_checked),
///     [*unchecked*](#method.range_to_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.range_to_mut),
///     [*mut_checked*](#method.range_to_mut_checked),
///     [*mut_unchecked*](#method.range_to_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[..end]`
///   - [range_to_inclusive](#method.range_to_inclusive)
///    ([*checked*](#method.range_to_inclusive_checked),
///     [*unchecked*](#method.range_to_inclusive_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.range_to_inclusive_mut),
///     [*mut_checked*](#method.range_to_inclusive_mut_checked),
///     [*mut_unchecked*](#method.range_to_inclusive_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[..=end]`
///   - [range_from](#method.range_from),
///    ([*checked*](#method.range_from_checked),
///     [*unchecked*](#method.range_from_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.range_from_mut),
///     [*mut_checked*](#method.range_from_mut_checked),
///     [*mut_unchecked*](#method.range_from_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[start..]`
///   - [range](#method.range)
///    ([*checked*](#method.range_checked),
///     [*unchecked*](#method.range_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.range_mut),
///     [*mut_checked*](#method.range_mut_checked),
///     [*mut_unchecked*](#method.range_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[start..end]`
///   - [range_inclusive](#method.range_inclusive)
///    ([*checked*](#method.range_inclusive_checked),
///     [*unchecked*](#method.range_inclusive_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.range_inclusive_mut),
///     [*mut_checked*](#method.range_inclusive_mut_checked),
///     [*mut_unchecked*](#method.range_inclusive_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[start..=end]`
///
/// - [`take*` API methods](#take-api-methods-for-subslicing):<br/>
///   - [take_first](#method.take_first)
///    ([*checked*](#method.take_first_checked),
///     [*unchecked*](#method.take_first_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.take_first_mut),
///     [*mut_checked*](#method.take_first_mut_checked),
///     [*mut_unchecked*](#method.take_first_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[..n]`
///   - [take_last](#method.take_last)
///    ([*checked*](#method.take_last_checked),
///     [*unchecked*](#method.take_last_unchecked)<sup title="unsafe method">⚠</sup>,
///     [_mut_](#method.take_last_mut),
///     [*mut_checked*](#method.take_last_mut_checked),
///     [*mut_unchecked*](#method.take_last_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[len - n..]`
///   - [take_omit_last](#method.take_omit_last)
///    ([*checked*](#method.take_omit_last_checked),
///     [*unchecked*](#method.take_omit_last_unchecked)<sup title="unsafe method">⚠</sup>
///     [_mut_](#method.take_omit_last_mut),
///     [*mut_checked*](#method.take_omit_last_mut_checked),
///     [*mut_unchecked*](#method.take_omit_last_mut_unchecked)<sup title="unsafe method">⚠</sup>)
///     ≈ `&slice[..len - n]`
///
/// - [`*split*` API methods](#split-api-methods-for-subslicing):<br/>
///   - [lsplit](#method.lsplit)
///    ([*mut*](#method.lsplit_mut))
///   - [rsplit](#method.rsplit)
///    ([*mut*](#method.rsplit_mut))
///   - [msplit_left](#method.msplit_left)
///    ([*mut*](#method.msplit_left_mut))
///   - [msplit_right](#method.msplit_right)
///    ([*mut*](#method.msplit_right_mut))
///
/// - [`*chunk*` API methods](#chunk-api-methods-for-subslicing):<br/>
///   - [chunks_exact](#method.chunks_exact)
///    ([*mut*](#method.chunks_exact_mut))
///   - [chunk](#method.chunk)
///    ([*mut*](#method.chunks_mut))
///
/// - [specific methods for byte slices](#methods-for-byte-slices)
/// - [`eq` methods for slices of (slices of) primitives](#method.eq)
///
/// See also: [`slice!`] and [`SliceExt`][crate::SliceExt].
#[derive(Debug)]
pub struct Slice<T>(crate::PhantomData<T>);
