// devela/src/code/util/enumset/_test.rs

#![allow(dead_code)]
use crate::enumset;

enumset! {
    enum TestEnum(TestEnumSet: u8) {
        A,
        B(bool),
        C { x: u8 },
    }
}
#[test]
fn uses_set_macro() {
    assert_eq!(TestEnum::VARIANTS, 3);
    assert_eq!(TestEnumSet::A.bits(), 0b001);
    assert_eq!(TestEnumSet::B.bits(), 0b010);
    assert_eq!(TestEnumSet::C.bits(), 0b100);
    assert_eq!(TestEnum::empty_set().bits(), 0);
    assert_eq!(TestEnum::full_set().bits(), 0b111);
    let ab = TestEnumSet::A.union(TestEnumSet::B);
    assert!(ab.contains(TestEnumSet::A));
    assert!(ab.contains(TestEnumSet::B));
    assert!(!ab.contains(TestEnumSet::C));
    let mut s = TestEnumSet::A;
    s.insert(TestEnumSet::B);
    assert_eq!(s.bits(), 0b011);
    s.remove(TestEnumSet::A);
    assert_eq!(s.bits(), 0b010);
    s.toggle(TestEnumSet::C);
    assert_eq!(s.bits(), 0b110);
    s.clear();
    assert_eq!(s.bits(), 0);
    assert!(s.is_empty());
    assert_eq!((TestEnumSet::A | TestEnumSet::B).bits(), 0b011);
    assert_eq!((TestEnumSet::A | TestEnumSet::B) & TestEnumSet::B, TestEnumSet::B);
    assert_eq!((TestEnumSet::A | TestEnumSet::B) - TestEnumSet::A, TestEnumSet::B);
    assert_eq!((TestEnumSet::A | TestEnumSet::B) ^ TestEnumSet::B, TestEnumSet::A);
    assert_eq!((!TestEnumSet::A).bits(), 0b110);
}
#[test]
fn is_in_set() {
    let a = TestEnum::A;
    let b = TestEnum::B(true);
    let c = TestEnum::C { x: 7 };
    assert_eq!(a.to_set(), TestEnumSet::A);
    assert_eq!(b.to_set(), TestEnumSet::B);
    assert_eq!(c.to_set(), TestEnumSet::C);
    assert!(a.is_in(TestEnumSet::A));
    assert!(b.is_in(TestEnumSet::A | TestEnumSet::B));
    assert!(c.is_in(TestEnumSet::C));
    assert!(!c.is_in(TestEnumSet::A | TestEnumSet::B));
}
#[test]
fn generics() {
    enumset! {
        enum GenEnum<'a, T>(pub GenEnumSet: u8) {
            A(T),
            B(&'a str),
        }
        impl enum {
            pub fn is_a(&self) -> bool {
                self.is_in(GenEnumSet::A)
            }
        }
    }
    let a = GenEnum::A(());
    assert!(a.is_a());
}
#[test]
fn generic_payload_variant_methods() {
    enumset! {
        #[derive(Clone, Copy, Debug)]
        enum E<'a, T>(pub ES: u8) [where T: Copy] {
            A(&'a T),
            B { value: T },
        }
    }
    let x = 7u8;
    let a = E::A(&x);
    let b = E::B { value: x };
    let mut set = ES::default();
    assert!(!set.contains_variant(&a));
    set.insert_variant(&a);
    assert!(set.contains_variant(&a));
    assert!(set.has_variant(&a));
    set = set.with_variant(&b);
    assert!(set.contains_variant(&b));
    set.remove_variant(&a);
    assert!(!set.contains_variant(&a));
    set.toggle_variant(&b);
    assert!(!set.contains_variant(&b));
    assert!(set.with_variant(&a).without_variant(&a).is_empty());
    assert!(set.toggled_variant(&a).contains_variant(&a));
}
#[test]
fn generic_payload_from_enum() {
    enumset! {
        #[derive(Clone, Copy, Debug)]
        enum E<'a, T>(pub ES: u8) [where T: Copy] {
            A(&'a T),
            B(T),
        }
    }
    let x = 3u8;
    let a = E::A(&x);
    let b = E::B(x);
    let sa: ES = a.into();
    let sb: ES = (&b).into();
    assert!(sa.contains(ES::A));
    assert!(sb.contains(ES::B));
}
#[test]
fn unit_only_iteration() {
    enumset! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum Mode(pub ModeSet: u8) {
            Read,
            Write,
            Exec,
        }
    }
    assert_eq!(Mode::ALL, [Mode::Read, Mode::Write, Mode::Exec]);
    let set = ModeSet::Read.with(ModeSet::Exec);
    let mut seen = [false; 3];
    set.for_each(|m| match m {
        Mode::Read => seen[0] = true,
        Mode::Write => seen[1] = true,
        Mode::Exec => seen[2] = true,
    });
    assert_eq!(seen, [true, false, true]);
    let mut count = 0;
    let completed = set.for_each_while(|_| {
        count += 1;
        false
    });
    assert!(!completed);
    assert_eq!(count, 1);
}

/**
```compile_fail
# use devela::enumset;
enumset! { enum E(pub ES: u8) { A(bool) } }
let _ = E::ALL;
```
**/
#[allow(dead_code)]
fn unit_only_constant() {}
