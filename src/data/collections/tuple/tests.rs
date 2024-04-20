// devela::data::collections::tuple::tests

use super::*;

type TestTuple = (i32, f32, &'static str, bool);

const TUPLE: TestTuple = (10, 20.5, "Hello", true);

#[test]
fn tuple_arity() {
    assert_eq!(TUPLE.arity(), 4);
    assert_eq!(TUPLE.arity(), TestTuple::ARITY);
}

#[test]
fn tuple_head() {
    assert_eq!(TUPLE.head(), &10);
}

#[test]
fn tuple_tail() {
    assert_eq!(TUPLE.tail(), &true);
}

#[test]
fn tuple_head_mut() {
    let mut tuple = TUPLE;
    assert_eq!(tuple.head(), &10);
    *tuple.head_mut() = 15;
    assert_eq!(tuple.head(), &15);
}

#[test]
fn tuple_tail_mut() {
    let mut tuple = TUPLE;
    assert_eq!(tuple.tail(), &true);
    *tuple.tail_mut() = false;
    assert_eq!(tuple.tail(), &false);
}

#[test]
fn tuple_split_head() {
    let (head, no_head) = TUPLE.split_head();
    assert_eq!(head, 10);
    assert_eq!(no_head, (20.5, "Hello", true));
}

#[test]
fn tuple_split_tail() {
    let (no_tail, tail) = TUPLE.split_tail();
    assert_eq!(no_tail, (10, 20.5, "Hello"));
    assert_eq!(tail, true);
}

#[test]
fn tuple_no_head() {
    assert_eq!(TUPLE.no_head(), (20.5, "Hello", true));
}

#[test]
fn tuple_no_tail() {
    assert_eq!(TUPLE.no_tail(), (10, 20.5, "Hello"));
}

#[test]
fn tuple_append() {
    let result = TUPLE.append('@');
    assert_eq!(result, (10, 20.5, "Hello", true, '@'));
}

#[test]
fn tuple_prepend() {
    let result = TUPLE.prepend('@');
    assert_eq!(result, ('@', 10, 20.5, "Hello", true));
}

#[test]
fn tuple_nth() {
    assert![matches![TUPLE.nth(2), TupleElement::_2("Hello")]];

    // match TUPLE.nth(2) {
    //     TupleElement::_2(value) => assert_eq!(value, "Hello"),
    //     _ => panic!("Unexpected result"),
    // }
}
#[test]
fn tuple_nth_ref() {
    assert![matches![TUPLE.nth_ref(2), TupleElementRef::_2(&"Hello")]];
}
#[test] #[rustfmt::skip]
fn tuple_nth_mut() {
    let mut tuple = TUPLE;
    assert![matches![tuple.nth_mut(2), TupleElementMut::_2(&mut "Hello")]];
}
