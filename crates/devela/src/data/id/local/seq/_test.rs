//
//!
//

use crate::{AnyExt, AtomicOrdering, id_seq};

#[test]
fn id_seq_start_uniqueness_end() {
    id_seq![TestIdSeqU8a, u8];
    id_seq![TestIdSeqU8b, u8];
    assert_eq![TestIdSeqU8a::generated_ids(), 0];
    assert_eq![TestIdSeqU8a::remaining_ids(), u8::MAX];
    let u8a_id0 = TestIdSeqU8a::new().unwrap();
    let u8b_id0 = TestIdSeqU8b::new().unwrap();
    // Types are different; values may be the same.
    assert_ne![u8a_id0.type_of(), u8b_id0.type_of()];
    assert_eq![u8a_id0.value(), 0];
    assert_eq![u8b_id0.value(), 0];
    let u8a_id1 = TestIdSeqU8a::new().unwrap();
    assert_eq![u8a_id1.value(), 1];
    assert_eq![TestIdSeqU8a::generated_ids(), 2];
    assert_eq![TestIdSeqU8a::remaining_ids(), u8::MAX - 2];
}
#[test]
fn id_seq_exhaustion_is_sticky() {
    id_seq![TestIdSeqU8Exhaust, u8];
    for expected in 0..u8::MAX {
        let id = TestIdSeqU8Exhaust::new().unwrap();
        assert_eq![id.value(), expected];
    }
    assert_eq![TestIdSeqU8Exhaust::generated_ids(), u8::MAX];
    assert_eq![TestIdSeqU8Exhaust::remaining_ids(), 0];
    // MAX is the terminal state and must never be emitted.
    assert_eq![TestIdSeqU8Exhaust::new(), None];
    // Exhaustion must remain permanent.
    assert_eq![TestIdSeqU8Exhaust::new(), None];
    assert_eq![TestIdSeqU8Exhaust::new(), None];
    assert_eq![TestIdSeqU8Exhaust::generated_ids(), u8::MAX];
    assert_eq![TestIdSeqU8Exhaust::remaining_ids(), 0];
}
#[test]
fn id_seq_custom_ordering() {
    id_seq![TestIdSeqU8Ordering, u8];
    let a = TestIdSeqU8Ordering::new_with_ordering(AtomicOrdering::SeqCst).unwrap();
    let b = TestIdSeqU8Ordering::new_with_ordering_unchecked(AtomicOrdering::AcqRel);
    let c = TestIdSeqU8Ordering::new_with_ordering(AtomicOrdering::Release).unwrap();
    assert_eq![a.value(), 0];
    assert_eq![b.value(), 1];
    assert_eq![c.value(), 2];
}
#[test]
#[cfg(feature = "alloc")]
fn id_seq_iter() {
    use crate::Vec;
    id_seq![TestIdSeqU8Iter, u8];
    let ids: Vec<_> = TestIdSeqU8Iter::iter().take(10).collect();
    let expected: Vec<u8> = (0..10).collect();
    assert_eq![ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected];
    let ids: Vec<_> =
        TestIdSeqU8Iter::iter_with_ordering(AtomicOrdering::SeqCst).take(10).collect();
    let expected: Vec<u8> = (10..20).collect();
    assert_eq![ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected];
}
#[test]
#[cfg(feature = "alloc")]
fn id_seq_iter_stops_at_max() {
    use crate::Vec;
    id_seq![TestIdSeqU8IterStops, u8];
    type Id = TestIdSeqU8IterStops;
    // Generate 0..=251.
    let _: Vec<_> = Id::iter().take(252).collect();
    // The final valid IDs are 252, 253 and 254.
    let ids: Vec<_> = Id::iter().collect();
    let expected = Vec::from([252, 253, 254]);
    assert_eq![ids.iter().map(|id| id.value()).collect::<Vec<_>>(), expected];
    assert_eq![Id::generated_ids(), u8::MAX];
    assert_eq![Id::remaining_ids(), 0];
    // Iterating again remains empty.
    assert_eq![Id::iter().next(), None];
}
#[test]
#[cfg(feature = "std")]
fn id_seq_unchecked_panics_on_exhaustion() {
    use std::panic::catch_unwind;
    id_seq![TestIdSeqU8Panics, u8];
    type Id = TestIdSeqU8Panics;
    for _ in 0..u8::MAX {
        let _ = Id::new_unchecked();
    }
    let result = catch_unwind(|| {
        let _ = Id::new_unchecked();
    });
    assert![result.is_err(), "expected panic after exhausting the ID sequence"];
}
