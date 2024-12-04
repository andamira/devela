// devela::data::sort::definition
//
//! Defines and documents [`Sort`].
//

/// Provides sorting methods for arrays and slices of `T`.
///
/// It implements the following methods for sorting exclusive slices:
/// [`bubble`][Sort#bubble],
/// [`counting`][Sort#counting],
/// [`counting_buf`][Sort#counting_buf],
/// [`insertion`][Sort#insertion],
/// [`merge`][Sort#merge],
/// [`quick_lomuto`][Sort#quick_lomuto],
/// [`quick_hoare`][Sort#quick_hoare],
/// [`quick_3way`][Sort#quick_3way],
/// [`quick_selection`][Sort#quick_selection],
/// [`quick_shaker`][Sort#quick_shaker].
///
/// # Features
/// When the corresponding feature `_sort_f[32|64]` or `_sort_[iu][8|16|32|64|128]` is enabled,
/// It implements the following *const* methods for sorting copied arrays of primitives:
/// `bubble_array`, `insertion_array`, `selection_array`.
/// In the case of floating-point primitives it uses total ordering.
///
/// # Examples
/// Sort copied arrays of primitives:
/// ```
/// # use devela::Sort;
/// # #[cfg(feature = "_sort_i32")]
/// # {
/// let mut arr = [4i32, 7, -5, 1, -13, 0]; // signed primitives
/// assert_eq![Sort(arr).bubble_array(),    [-13, -5, 0, 1, 4, 7]];
/// assert_eq![Sort(arr).insertion_array(), [-13, -5, 0, 1, 4, 7]];
/// assert_eq![Sort(arr).selection_array(), [-13, -5, 0, 1, 4, 7]];
/// # }
///
/// # #[cfg(feature = "_sort_u32")]
/// # {
/// let mut arr = [4u32, 7, 5, 1, 13, 0]; // unsigned primitives
/// assert_eq![Sort(arr).bubble_array(),    [0, 1, 4, 5, 7, 13]];
/// assert_eq![Sort(arr).insertion_array(), [0, 1, 4, 5, 7, 13]];
/// assert_eq![Sort(arr).selection_array(), [0, 1, 4, 5, 7, 13]];
/// # }
///
/// # #[cfg(feature = "_sort_f32")]
/// # {
/// let mut arr = [4.01f32, 7.9, -5.4, 1.0, 0.0, -0.0]; // floating-point primitives
/// assert_eq![Sort(arr).bubble_array(),    [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// assert_eq![Sort(arr).insertion_array(), [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// assert_eq![Sort(arr).selection_array(), [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// # }
/// ```
///
/// # Performance
/// The `_array` suffixed methods calls the [`cswap`] macro using the xor swap
/// algorithm, except for the floting-point version which uses a temporary variable.
///
/// [`cswap`]: crate::cswap
#[repr(transparent)]
pub struct Sort<T>(pub T);
