// devela_base_alloc::data::uid::pin_box
//
//! Defines [`IdPinBox`].
//

use crate::{Box, Pin};

#[doc = crate::_TAG_UID!()]
#[doc = crate::_TAG_ALLOCATION!()]
/// A unique identifier based on a pinned heap-allocated memory address.
#[doc = crate::_doc_location!("data/uid")]
///
/// `IdPinBox` generates a unique ID by pinning a value in heap memory,
/// ensuring that the ID remains stable and unique based on the memory address.
///
/// See also [`IdPin`][crate::IdPin].
///
#[doc = crate::_doc!(vendor: "object-id")]
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
        (&raw const *self.inner).addr()
    }
}

mod impl_traits {
    use crate::{Box, Debug, FmtResult, Formatter, Hash, Hasher, IdPinBox, Ordering, Ptr};

    impl Default for IdPinBox {
        fn default() -> Self {
            Self { inner: Box::pin(0) }
        }
    }

    impl Debug for IdPinBox {
        fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
            write!(f, "{}", self.as_usize())
        }
    }
    impl Hash for IdPinBox {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.as_usize().hash(state);
        }
    }

    impl PartialEq for IdPinBox {
        fn eq(&self, other: &Self) -> bool {
            Ptr::eq(&*self.inner, &*other.inner)
        }
    }
    impl Eq for IdPinBox {}

    impl PartialOrd for IdPinBox {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for IdPinBox {
        fn cmp(&self, other: &Self) -> Ordering {
            self.as_usize().cmp(&other.as_usize())
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::{sync::Arc, vec};

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
