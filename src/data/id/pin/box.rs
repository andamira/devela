// devela::data::id::pin::box
//
//! Pinned memory-based unique IDs.
//

use crate::mem::Box;
use core::{pin::Pin, ptr};

/// A unique identifier based on a pinned heap-allocated memory address.
///
/// `IdPinBox` generates a unique ID by pinning a value in heap memory,
/// ensuring that the ID remains stable and unique based on the memory address.
#[derive(Clone)]
pub struct IdPinBox {
    inner: Pin<Box<u8>>,
}

impl IdPinBox {
    /// Creates a new `IdPinBox` with a unique memory address.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the unique ID as a `usize`, derived from the memory address.
    pub fn as_usize(&self) -> usize {
        ptr::addr_of!(*self.inner) as usize
    }
}

mod impl_traits {
    use super::{Box, IdPinBox};
    use core::{cmp::Ordering, fmt, hash, ptr};

    impl Default for IdPinBox {
        fn default() -> Self {
            Self { inner: Box::pin(0) }
        }
    }

    impl fmt::Debug for IdPinBox {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.as_usize())
        }
    }
    impl hash::Hash for IdPinBox {
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.as_usize().hash(state);
        }
    }

    impl PartialEq for IdPinBox {
        fn eq(&self, other: &Self) -> bool {
            ptr::eq(&*self.inner, &*other.inner)
        }
    }
    impl Eq for IdPinBox {}

    impl PartialOrd for IdPinBox {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.as_usize().cmp(&other.as_usize()))
        }
    }
    impl Ord for IdPinBox {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_usize().cmp(&other.as_usize())
        }
    }
}

#[cfg(all(test, feature = "alloc"))]
mod test {
    use crate::_dep::_alloc::{sync::Arc, vec};

    use super::IdPinBox;

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct Test {
        id: IdPinBox,
    }

    struct TestWrapper {
        inner: Arc<Test>,
    }

    impl Clone for TestWrapper {
        fn clone(&self) -> Self {
            Self { inner: self.inner.clone() }
        }
    }

    impl TestWrapper {
        fn new() -> Self {
            Self { inner: Test { id: <_>::default() }.into() }
        }
        fn id(&self) -> usize {
            self.inner.id.as_usize()
        }
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn test_stack() {
        panic!(
            "the test MUST be run for the debug target,
as there is still a chance the object generator may be inlined"
        );
    }
    #[cfg(debug_assertions)]
    #[test]
    fn test_stack() {
        #[inline(never)]
        fn generate() -> (Test, usize) {
            let t = Test { id: <_>::default() };
            let n = t.id.as_usize();
            (t, n)
        }
        let (t, n) = generate();
        assert_eq!(t.id.as_usize(), n);
    }
    #[test]
    fn test_clone_eq() {
        let t = Test { id: <_>::default() };
        let t2 = t.clone();
        assert_ne!(t.id, t2.id);
        assert_ne!(t.id.as_usize(), t2.id.as_usize());
        assert_ne!(t, t2);
        assert_eq!(t, t);
        assert_eq!(t.id, t.id);
    }
    #[test]
    fn test_heap_movement() {
        let t = Test { id: <_>::default() };
        let n = t.id.as_usize();
        let mut x = vec![t];
        assert_eq!(x[0].id.as_usize(), n);
        let t_back = x.pop().unwrap();
        assert_eq!(t_back.id.as_usize(), n);
    }
    #[test]
    fn test_arc_covered() {
        let t1 = TestWrapper::new();
        let t2 = t1.clone();
        assert_eq!(t1.id(), t2.id());
    }
}
