// devela::data::collections::vec::chunk
//
//
// IMPROVE: bring benchmarks

use crate::{vec_ as vec, Rc, RefCell, Vec};

#[cfg(test)]
mod tests;

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A persistent data structure with efficient append and concatenation operations.
///
/// # Overview
/// `VecChunk<A>` is an immutable data structure that allows O(1) complexity for append and
/// concatenation operations through structural sharing. It uses [`Rc`] (Reference Counting)
/// for efficient memory management.
///
/// # Performance
/// - Append operation: O(1)
/// - Concatenation operation: O(1)
/// - Converting to Vec: O(n)
///
/// # Implementation Details
/// The data structure is implemented as an enum with three variants:
/// - `Empty`: Represents an empty chunk
/// - `Append`: Represents a single element appended to another chunk
/// - `Concat`: Represents the concatenation of two chunks
///
/// # Example
/// ```
/// # use devela::VecChunk;
/// let mut chunk = VecChunk::default();
/// chunk = chunk.append(1);
/// chunk = chunk.append(2);
///
/// let other_chunk = VecChunk::default().append(3).append(4);
/// let combined = chunk.concat(other_chunk);
///
/// assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
/// ```
///
/// # References
/// - [Persistent Data Structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
/// - [Structural Sharing](https://hypirion.com/musings/understanding-persistent-vector-pt-1)
///
/// # Derived Work
#[doc = include_str!("./MODIFICATIONS.md")]
#[must_use]
#[derive(Clone)]
pub enum VecChunk<A> {
    /// Represents an empty chunk with no elements
    Empty,
    /// Represents a chunk containing exactly one element
    Single(A),
    /// Represents the concatenation of two chunks, enabling O(1) concatenation
    Concat(Rc<VecChunk<A>>, Rc<VecChunk<A>>),
    /// Represents a collection of elements
    Collect(Rc<RefCell<Vec<A>>>),
    /// Represents a lazy transformation that flattens elements
    TransformFlatten(Rc<VecChunk<A>>, Rc<dyn Fn(A) -> VecChunk<A>>),
}

impl<A> Default for VecChunk<A> {
    /// Creates a new empty chunk.
    ///
    /// This is equivalent to using [`VecChunk::Empty`].
    fn default() -> Self {
        VecChunk::Empty
    }
}

impl<A> VecChunk<A> {
    /// Creates a new chunk containing a single element.
    ///
    /// # Arguments
    /// * `a` - The element to store in the chunk
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk: VecChunk<i32> = VecChunk::new(100);
    /// assert!(!chunk.is_null());
    /// ```
    pub fn new(a: A) -> Self {
        VecChunk::Single(a)
    }

    /// Returns `true` if the chunk is empty.
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk: VecChunk<i32> = VecChunk::default();
    /// assert!(chunk.is_null());
    ///
    /// let non_empty = chunk.append(42);
    /// assert!(!non_empty.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        match self {
            VecChunk::Empty => true,
            VecChunk::Collect(vec) => vec.borrow().is_empty(),
            _ => false,
        }
    }

    /// Append a new element to the chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Append` variant
    /// that references the existing chunk through an [`Rc`].
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk = VecChunk::default().append(1).append(2);
    /// assert_eq!(chunk.as_vec(), vec![1, 2]);
    /// ```
    pub fn append(self, a: A) -> Self {
        self.concat(VecChunk::new(a))
    }

    /// Prepend a new element to the beginning of the chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Concat` variant
    /// that references the existing chunk through an [`Rc`].
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk = VecChunk::default().prepend(1).prepend(2);
    /// assert_eq!(chunk.as_vec(), vec![2, 1]);
    /// ```
    pub fn prepend(self, a: A) -> Self {
        if self.is_null() {
            VecChunk::new(a)
        } else {
            VecChunk::new(a).concat(self)
        }
    }

    /// Concatenates this chunk with another chunk.
    ///
    /// This operation has O(1) complexity as it creates a new `Concat` variant
    /// that references both chunks through [`Rc`]s.
    ///
    /// # Performance Optimization
    /// If either chunk is empty, returns the other chunk instead of creating
    /// a new `Concat` variant.
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk1 = VecChunk::default().append(1).append(2);
    /// let chunk2 = VecChunk::default().append(3).append(4);
    /// let combined = chunk1.concat(chunk2);
    /// assert_eq!(combined.as_vec(), vec![1, 2, 3, 4]);
    /// ```
    pub fn concat(self, other: VecChunk<A>) -> VecChunk<A> {
        match (self, other) {
            // Handle null cases
            (VecChunk::Empty, other) => other,
            (this, VecChunk::Empty) => this,
            (VecChunk::Single(a), VecChunk::Single(b)) => {
                VecChunk::Collect(Rc::new(RefCell::new(vec![a, b])))
            }
            (VecChunk::Collect(vec), VecChunk::Single(a)) => {
                if Rc::strong_count(&vec) == 1 {
                    // Only clone if there are no other references
                    vec.borrow_mut().push(a);
                    VecChunk::Collect(vec)
                } else {
                    VecChunk::Concat(Rc::new(VecChunk::Collect(vec)), Rc::new(VecChunk::Single(a)))
                }
            }
            // Handle all other cases with Concat
            (this, that) => VecChunk::Concat(Rc::new(this), Rc::new(that)),
        }
    }

    /// Transforms each element in the chunk using the provided function.
    ///
    /// This method creates a lazy representation of the transformation without actually
    /// performing it. The transformation is only executed when [`as_vec`](VecChunk::as_vec)
    /// or [`as_vec_mut`](VecChunk::as_vec_mut) is called.
    ///
    /// # Performance
    /// - Creating the transformation: O(1)
    /// - Executing the transformation (during [`as_vec`](VecChunk::as_vec)): O(n)
    ///
    /// # Arguments
    /// * `f` - A function that takes a reference to an element of type `A` and returns
    ///         a new element of type `A`
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk = VecChunk::default().append(1).append(2).append(3);
    /// // This operation is O(1) and doesn't actually transform the elements
    /// let doubled = chunk.transform(|x| x * 2);
    /// // The transformation happens here, when we call as_vec()
    /// assert_eq!(doubled.as_vec(), vec![2, 4, 6]);
    /// ```
    pub fn transform(self, f: impl Fn(A) -> A + 'static) -> Self {
        self.transform_flatten(move |a| VecChunk::new(f(a)))
    }

    /// Materializes a chunk by converting it into a collected form.
    ///
    /// This method evaluates any lazy transformations and creates a new chunk containing
    /// all elements in a `Collect` variant. This can be useful for performance when you
    /// plan to reuse the chunk multiple times, as it prevents re-evaluation of transformations.
    ///
    /// # Performance
    /// - Time complexity: O(n) where n is the number of elements
    /// - Space complexity: O(n) as it creates a new vector containing all elements
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk = VecChunk::default()
    ///     .append(1)
    ///     .append(2)
    ///     .transform(|x| x * 2);  // Lazy transformation
    ///
    /// // Materialize the chunk to evaluate the transformation once
    /// let materialized = chunk.materialize();
    ///
    /// assert_eq!(materialized.as_vec(), vec![2, 4]);
    /// ```
    pub fn materialize(self) -> VecChunk<A>
    where
        A: Clone,
    {
        VecChunk::Collect(Rc::new(RefCell::new(self.as_vec())))
    }

    /// Transforms each element in the chunk into a new chunk and flattens the result.
    ///
    /// This method creates a lazy representation of the transformation without actually
    /// performing it. The transformation is only executed when [`as_vec`](VecChunk::as_vec)
    /// or [`as_vec_mut`](VecChunk::as_vec_mut) is called.
    ///
    /// # Performance
    /// - Creating the transformation: O(1)
    /// - Executing the transformation (during [`as_vec`](VecChunk::as_vec)): O(n)
    ///
    /// # Arguments
    /// * `f` - A function that takes an element of type `A` and returns
    ///         a new `VecChunk<A>`
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk = VecChunk::default().append(1).append(2);
    /// // Transform each number x into a chunk containing [x, x+1]
    /// let expanded = chunk.transform_flatten(|x| {
    ///     VecChunk::default().append(x).append(x + 1)
    /// });
    /// assert_eq!(expanded.as_vec(), vec![1, 2, 2, 3]);
    /// ```
    pub fn transform_flatten(self, f: impl Fn(A) -> VecChunk<A> + 'static) -> Self {
        VecChunk::TransformFlatten(Rc::new(self), Rc::new(f))
    }

    /// Converts the chunk into a vector of references to its elements.
    ///
    /// This operation has O(n) complexity where n is the number of elements
    /// in the chunk.
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let chunk = VecChunk::default().append(1).append(2).append(3);
    /// assert_eq!(chunk.as_vec(), vec![1, 2, 3]);
    /// ```
    pub fn as_vec(&self) -> Vec<A>
    where
        A: Clone,
    {
        let mut vec = Vec::new();
        self.as_vec_mut(&mut vec);
        vec
    }

    /// Helper method that populates a vector with references to the chunk's elements.
    ///
    /// This method is used internally by [`as_vec`](VecChunk::as_vec) to avoid
    /// allocating multiple vectors during the traversal.
    ///
    /// # Arguments
    /// * `buf` - A mutable reference to a vector that will be populated with
    ///           references to the chunk's elements
    pub fn as_vec_mut(&self, buf: &mut Vec<A>)
    where
        A: Clone,
    {
        match self {
            VecChunk::Empty => {}
            VecChunk::Single(a) => {
                buf.push(a.clone());
            }
            VecChunk::Concat(a, b) => {
                a.as_vec_mut(buf);
                b.as_vec_mut(buf);
            }
            VecChunk::TransformFlatten(a, f) => {
                let mut tmp = Vec::new();
                a.as_vec_mut(&mut tmp);
                for elem in tmp.into_iter() {
                    f(elem).as_vec_mut(buf);
                }
            }
            VecChunk::Collect(vec) => {
                buf.extend(vec.borrow().iter().cloned());
            }
        }
    }
}

impl<A> FromIterator<A> for VecChunk<A> {
    /// Creates a chunk from an iterator.
    ///
    /// # Example
    /// ```
    /// # use devela::VecChunk;
    /// let vec = vec![1, 2, 3];
    /// let chunk: VecChunk<_> = vec.into_iter().collect();
    /// assert_eq!(chunk.as_vec(), vec![1, 2, 3]);
    /// ```
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let vec: Vec<_> = iter.into_iter().collect();
        VecChunk::Collect(Rc::new(RefCell::new(vec)))
    }
}
