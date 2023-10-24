// devela::data::dst::tests::stack

use core::any::Any;
use crate::data::{DstStack, DstArray};

type DS<T> = DstStack<T, DstArray<usize, 8>>;

#[test]
// A trivial check that ensures that methods are correctly called
fn trivial_type() {
    let mut val = DS::<dyn PartialEq<u32>>::new();
    val.push_stable(1234, |p| p).unwrap();
    val.push_stable(1233, |p| p).unwrap();
    assert!(*val.top().unwrap() != 1234);
    assert!(*val.top().unwrap() == 1233);
    val.pop();
    assert!(*val.top().unwrap() == 1234);
    assert!(*val.top().unwrap() != 1233);
}

#[test]
fn strings() {
    let mut stack: DS<str> = DS::new();
    stack.push_str("\n").unwrap();
    stack.push_str("World").unwrap();
    stack.push_str(" ").unwrap();
    stack.push_str("Hello").unwrap();

    assert_eq!(stack.top(), Some("Hello"));
    stack.pop();
    assert_eq!(stack.top(), Some(" "));
    stack.pop();
    assert_eq!(stack.top(), Some("World"));
    stack.pop();
    stack.push_str("World").unwrap();
    stack.push_str("Cruel").unwrap();
    assert_eq!(stack.top(), Some("Cruel"));
    stack.pop();
    assert_eq!(stack.top(), Some("World"));
    stack.pop();
    assert_eq!(stack.top(), Some("\n"));
    stack.pop();
    assert_eq!(stack.top(), None);
}

#[test]
fn slices() {
    let mut stack: DS<[u8]> = DS::new();

    stack.push_cloned(b"123").unwrap();
    stack.push_cloned(b"").unwrap();
    stack.push_cloned(b"abcd").unwrap();
    assert_eq!(stack.top(), Some(b"abcd" as &[_]));
    stack.pop();
    assert_eq!(stack.top(), Some(b"" as &[_]));
    stack.pop();
    assert_eq!(stack.top(), Some(b"123" as &[_]));
    stack.pop();
    assert_eq!(stack.top(), None);
}

#[test]
fn limits() {
    let mut val = stack_dst::DstStack::<dyn Any, stack_dst::buffers::Ptr2>::new();
    // Pushing when full
    val.push_stable(1usize, |p| p).unwrap();
    assert!(val.push_stable(2usize, |p| p).is_err());

    // Popping past empty (should stay empty)
    val.pop();
    assert!(val.is_empty());
    val.pop();
    assert!(val.is_empty());

    // Zero-sized types
    val.push_stable((), |p| p).unwrap();
    val.push_stable((), |p| p).unwrap();
    assert!(val.push_stable((), |p| p).is_err());
    val.pop();

    // Pushing a value when there is space, but no enough space for the entire value
    assert!(val.push_stable(1usize, |p| p).is_err());
    val.push_stable((), |p| p).unwrap();
}

#[test]
#[cfg(feature = "std")]
fn destructors() {
    struct DropWatch(std::rc::Rc<core::cell::Cell<usize>>);
    impl core::ops::Drop for DropWatch {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let v: std::rc::Rc<core::cell::Cell<_>> = Default::default();

    let mut stack = DstStack::<dyn Any, DstArray<usize, 8>>::new();
    // Successful pushes shouldn't call destructors
    stack.push_stable(DropWatch(v.clone()), |p| p).ok().unwrap();
    assert_eq!(v.get(), 0);
    stack.push_stable(DropWatch(v.clone()), |p| p).ok().unwrap();
    assert_eq!(v.get(), 0);
    stack.push_stable(DropWatch(v.clone()), |p| p).ok().unwrap();
    assert_eq!(v.get(), 0);
    stack.push_stable(DropWatch(v.clone()), |p| p).ok().unwrap();
    assert_eq!(v.get(), 0);
    // Failed push should return the value (which will be dropped)
    assert!(stack.push_stable(DropWatch(v.clone()), |p| p).is_err());
    assert_eq!(v.get(), 1);

    // Pop a value, drop increases
    stack.pop();
    assert_eq!(v.get(), 2);
    // Drop the entire stack, the rest are dropped
    drop(stack);
    assert_eq!(v.get(), 2 + 3);
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
        let mut stack = DstStack::<[Sentinel], DstArray<usize, 8>>::new();
        let _ = stack.push_cloned(&input);
    }));
    assert_eq!(COUNT.load(Ordering::SeqCst), 1);
}

#[test]
// Check that panic safety is maintained, even if the datatype isn't aligned to usize
#[cfg(feature = "std")]
fn slice_push_panic_safety_unaligned() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    struct Sentinel(bool);
    impl Clone for Sentinel {
        fn clone(&self) -> Self {
            if !self.0 {
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
    let input = [
        // 1 good followed by one bad
        Sentinel(true),
        Sentinel(false),
    ];

    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut stack = DstStack::<[Sentinel], _>::with_buffer(
            [core::mem::MaybeUninit::new(0xFFu8); 32],
        );
        let _ = stack.push_cloned(&input);
    }));
    assert_eq!(COUNT.load(Ordering::SeqCst), 1);
}
