// devela::data::layout::sort::definition
//
//! Defines and documents [`Sort`].
//

#[doc = crate::_tags!(namespace num)]
/// Provides sorting methods for arrays and slices of `T`.
#[doc = crate::_doc_location!("data/layout")]
///
/// It implements the following methods for sorting exclusive slices:
/// [`bubble`][Sort#bubble],
/// [`counting_buf`][Sort#counting_buf],
/// [`insertion`][Sort#insertion],
/// [`quick_lomuto`][Sort#quick_lomuto],
/// [`quick_hoare`][Sort#quick_hoare],
/// [`quick_3way`][Sort#quick_3way],
/// [`quick_selection`][Sort#quick_selection],
/// [`quick_shaker`][Sort#quick_shaker].
///
/// And the following allocating methods:
/// [`counting`][Sort#counting],
/// [`merge`][Sort#merge].
///
/// # Examples
/// Sort copied arrays of primitives:
/// ```
/// # use devela::Sort;
/// let mut arr = [4i32, 7, -5, 1, -13, 0]; // signed primitives
/// assert_eq![Sort(arr).bubble_array(),    [-13, -5, 0, 1, 4, 7]];
/// assert_eq![Sort(arr).insertion_array(), [-13, -5, 0, 1, 4, 7]];
/// assert_eq![Sort(arr).selection_array(), [-13, -5, 0, 1, 4, 7]];
///
/// let mut arr = [4u32, 7, 5, 1, 13, 0]; // unsigned primitives
/// assert_eq![Sort(arr).bubble_array(),    [0, 1, 4, 5, 7, 13]];
/// assert_eq![Sort(arr).insertion_array(), [0, 1, 4, 5, 7, 13]];
/// assert_eq![Sort(arr).selection_array(), [0, 1, 4, 5, 7, 13]];
///
/// let mut arr = [4.01f32, 7.9, -5.4, 1.0, 0.0, -0.0]; // floating-point primitives
/// assert_eq![Sort(arr).bubble_array(),    [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// assert_eq![Sort(arr).insertion_array(), [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// assert_eq![Sort(arr).selection_array(), [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
///
/// // compile-time friendly
/// const SORTED: [f32; 6] = Sort([4.01f32, 7.9, -5.4, 1.0, 0.0, -0.0]).bubble_array();
/// assert_eq![SORTED, [-5.4, -0.0, 0.0, 1.0, 4.01, 7.9]];
/// ```
///
/// Allocating methods:
/// ```
/// # #[cfg(feature = "alloc")] { use devela::Sort;
/// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
/// let freq = Sort(&mut data[..]).counting();
/// assert_eq![data, [2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 64, 64]];
/// assert_eq![freq, [3, 7, 5, 2]];
/// # }
/// ```
///
/// # Performance
/// The `_array` suffixed methods calls the [`cswap!`] macro using the xor swap
/// algorithm, except for the floting-point version which uses a temporary variable.
///
/// [`cswap!`]: crate::cswap
#[repr(transparent)]
#[derive(Debug)]
pub struct Sort<T>(pub T);

impl<T> Sort<T> {
    ///
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}
