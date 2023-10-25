use crate::data::{DstArray, DstValue};
use core::{cell::Cell, fmt};

type DstValue2w<DST /*: ?Sized*/> = DstValue<DST, DstArray<usize, 2>>;
type DstValue8w<DST /*: ?Sized*/> = DstValue<DST, DstArray<usize, 8>>;

// A trivial check that ensures that methods are correctly called
#[test]
fn trivial_type() {
    let val = DstValue2w::<dyn PartialEq<u32>>::new(1234u32, |p| p).unwrap();
    assert!(*val == 1234);
    assert!(*val != 1233);
}

// Create an instance with a Drop implementation, and ensure the drop handler
// fires when destructed. This also ensures that lifetimes are correctly handled
#[test]
fn ensure_drop() {
    #[derive(Debug)]
    struct Struct<'a>(&'a Cell<bool>);
    impl<'a> Drop for Struct<'a> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    let flag = Cell::new(false);
    let val = DstValue2w::<dyn fmt::Debug>::new(Struct(&flag), |p| p).unwrap();
    assert!(flag.get() == false);
    drop(val);
    assert!(flag.get() == true);
}

#[test]
fn many_instances() {
    trait TestTrait {
        fn get_value(&self) -> u32;
    }

    #[inline(never)]
    fn instance_one() -> DstValue2w<dyn TestTrait> {
        #[derive(Debug)]
        struct OneStruct(u32);
        impl TestTrait for OneStruct {
            fn get_value(&self) -> u32 {
                self.0
            }
        }
        DstValue2w::new(OneStruct(12345), |p| p as _).unwrap()
    }

    #[inline(never)]
    fn instance_two() -> DstValue2w<dyn TestTrait> {
        #[derive(Debug)]
        struct TwoStruct;
        impl TestTrait for TwoStruct {
            fn get_value(&self) -> u32 {
                54321
            }
        }
        DstValue2w::new(TwoStruct, |p| p as _).unwrap()
    }

    let i1 = instance_one();
    let i2 = instance_two();
    assert_eq!(i1.get_value(), 12345);
    assert_eq!(i2.get_value(), 54321);
}

#[test]
#[cfg(feature = "alloc")]
fn closure() {
    use ::_alloc::{format, string::String};

    let v1 = 1234u64;
    let c: DstValue8w<dyn Fn() -> String> = DstValue8w::new(|| format!("{}", v1), |p| p as _)
        .map_err(|_| "Oops")
        .unwrap();
    assert_eq!(c(), "1234");
}

#[test]
fn oversize() {
    use core::any::Any;
    const MAX_SIZE_PTRS: usize = 7;
    assert!(DstValue8w::<dyn Any>::new([0usize; MAX_SIZE_PTRS], |p| p).is_ok());
    assert!(DstValue8w::<dyn Any>::new([0usize; MAX_SIZE_PTRS + 1], |p| p).is_err());
}

#[test]
fn option() {
    use core::any::Any;
    assert!(Some(DstValue8w::<dyn Any>::new("foo", |p| p).unwrap()).is_some());
}

#[test]
#[should_panic]
fn stable_closure_different_pointer() {
    use fmt::Debug;
    static BIG_VALUE: [i32; 4] = [0, 0, 0, 0];
    // Type confusion via a different pointer
    let _ = DstValue8w::<dyn Debug>::new(123, |_| &BIG_VALUE as &dyn Debug);
}
#[test]
#[should_panic]
fn stable_closure_subset() {
    use fmt::Debug;
    let _ = DstValue8w::<dyn Debug>::new((1, 2), |v| &v.0 as &dyn Debug);
}
