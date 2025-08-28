// devela_base::data::sort::definition
//
//! Defines and documents [`SortAlloc`].
//

use crate::Sort;

#[doc = crate::TAG_NAMESPACE!()]
/// Provides allocating sorting methods for arrays and slices of `T`, extending [`Sort`].
///
/// [`Sort`]: devela_base::Sort
///
/// # Examples
/// Using allocating methods:
/// ```
/// # use devela_base_alloc::SortAlloc;
/// let mut data = [4, 64, 4, 2, 4, 8, 8, 4, 8, 4, 2, 8, 64, 4, 8, 4, 2];
/// let freq = SortAlloc::new(&mut data[..]).counting();
/// assert_eq![data, [2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 64, 64]];
/// assert_eq![freq, [3, 7, 5, 2]];
/// ```
///
/// Using non-allocating methods through deref:
/// ```
/// # use devela_base_alloc::SortAlloc;
/// let mut arr = [4, 7, -5, 1, -13, 0];
/// SortAlloc::new(&mut arr[..]).bubble();
/// assert_eq![arr, [-13, -5, 0, 1, 4, 7]];
/// ```
#[repr(transparent)]
#[derive(Debug)]
pub struct SortAlloc<T>(pub Sort<T>);

impl<T> SortAlloc<T> {
    /// Creates a new `SortAlloc` instance wrapping the given value.
    ///
    /// The returned instance provides access to both allocating sorting methods
    /// and the underlying non-allocating [`Sort`] methods through `DerefMut`.
    pub fn new(inner: T) -> Self {
        Self(Sort(inner))
    }
}

impl<T> crate::Deref for SortAlloc<T> {
    type Target = Sort<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> crate::DerefMut for SortAlloc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
