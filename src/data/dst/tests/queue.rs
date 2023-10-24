// devela::data::dst::tests::queue

use crate::data::{DstArray, DstQueue};

type DstFifo<T> = DstQueue<T, DstArray<usize, 8>>;

#[test]
// A trivial check that ensures that methods are correctly called
fn trivial_type() {
    let mut val = DstFifo::<dyn PartialEq<u32>>::new();
    val.push_back_stable(1234, |p| p).unwrap();
    val.push_back_stable(1233, |p| p).unwrap();
    assert!(*val.front().unwrap() == 1234);
    assert!(*val.front().unwrap() != 1233);
    val.pop_front();
    assert!(*val.front().unwrap() != 1234);
    assert!(*val.front().unwrap() == 1233);
}

#[test]
#[cfg(feature = "std")]
fn slice_push_panic_safety() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    struct Sentinel(bool);
    impl Clone for Sentinel {
        fn clone(&self) -> Self {
            if self.0 {
                panic!();
            } else {
                Sentinel(self.0)
            }
        }
    }
    impl Drop for Sentinel {
        fn drop(&mut self) {
            COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }
    let input = [Sentinel(false), Sentinel(true)];

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut stack = DstFifo::<[Sentinel]>::new();
        let _ = stack.push_cloned(&input);
    }));
    assert_eq!(COUNT.load(Ordering::SeqCst), 1);
}

#[test]
fn retain() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static FLAGS: AtomicUsize = AtomicUsize::new(0);
    struct Sentinel(usize);
    impl core::ops::Drop for Sentinel {
        fn drop(&mut self) {
            let flag = 1 << self.0;
            let v = FLAGS.fetch_or(1 << self.0, Ordering::SeqCst);
            assert!(v & flag == 0);
        }
    }
    impl AsRef<Sentinel> for Sentinel {
        fn as_ref(&self) -> &Sentinel {
            self
        }
    }
    let mut stack: DstQueue<dyn AsRef<Sentinel>, DstArray<usize, 16>> = DstQueue::new();
    stack.push_back_stable(Sentinel(0), |v| v).ok().unwrap();
    stack.push_back_stable(Sentinel(1), |v| v).ok().unwrap();
    stack.push_back_stable(Sentinel(2), |v| v).ok().unwrap();
    stack.push_back_stable(Sentinel(3), |v| v).ok().unwrap();
    stack.push_back_stable(Sentinel(4), |v| v).ok().unwrap();

    stack.retain(|v| v.as_ref().0 > 2);
    assert_eq!(FLAGS.load(Ordering::SeqCst), 0b00_111);
    {
        let mut it = stack.iter().map(|v| v.as_ref().0);
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);
    }
    drop(stack);
    assert_eq!(FLAGS.load(Ordering::SeqCst), 0b11_111);
}
