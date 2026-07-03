// devela/src/text/layout/wrap/_test.rs

use super::*;
use crate::{TextIndex, assert_eq};

#[test]
fn segments_from_chars_splits_boxes_glue_and_hard_breaks() {
    let mut segs = [TextSegment::INIT; 8];
    let len = TextSegment::segments_from_chars("one  two\nthree", &mut segs);
    assert_eq![len, 5];
    assert_eq![segs[0].kind, TextSegmentKind::Box];
    assert_eq![segs[0].span.start(), TextIndex(0)];
    assert_eq![segs[0].span.end(), TextIndex(3)];
    assert_eq![segs[0].span.units, 3];
    assert_eq![segs[1].kind, TextSegmentKind::Glue];
    assert_eq![segs[1].span.start(), TextIndex(3)];
    assert_eq![segs[1].span.end(), TextIndex(5)];
    assert_eq![segs[1].span.units, 1];
    assert_eq![segs[2].kind, TextSegmentKind::Box];
    assert_eq![segs[2].span.start(), TextIndex(5)];
    assert_eq![segs[2].span.end(), TextIndex(8)];
    assert_eq![segs[2].span.units, 3];
    assert_eq![segs[3].kind, TextSegmentKind::HardBreak];
    assert_eq![segs[3].span.start(), TextIndex(8)];
    assert_eq![segs[3].span.end(), TextIndex(9)];
    assert_eq![segs[3].span.units, 0];
    assert_eq![segs[4].kind, TextSegmentKind::Box];
    assert_eq![segs[4].span.start(), TextIndex(9)];
    assert_eq![segs[4].span.end(), TextIndex(14)];
    assert_eq![segs[4].span.units, 5];
}
#[test]
fn segments_from_chars_treats_crlf_as_one_hard_break() {
    let mut segs = [TextSegment::INIT; 8];
    let len = TextSegment::segments_from_chars("a\r\nb", &mut segs);
    assert_eq![len, 3];
    assert_eq![segs[0].kind, TextSegmentKind::Box];
    assert_eq![segs[0].span.start(), TextIndex(0)];
    assert_eq![segs[0].span.end(), TextIndex(1)];
    assert_eq![segs[1].kind, TextSegmentKind::HardBreak];
    assert_eq![segs[1].span.start(), TextIndex(1)];
    assert_eq![segs[1].span.end(), TextIndex(3)];
    assert_eq![segs[2].kind, TextSegmentKind::Box];
    assert_eq![segs[2].span.start(), TextIndex(3)];
    assert_eq![segs[2].span.end(), TextIndex(4)];
}
#[test]
fn wrap_iter_wraps_between_boxes() {
    let mut segs = [TextSegment::INIT; 8];
    let len = TextSegment::segments_from_chars("one two three", &mut segs);
    let mut wrap = TextWrapIter::new(&segs[..len], 7);
    let line1 = wrap.next_line().unwrap();
    assert_eq![line1.span.start(), TextIndex(0)];
    assert_eq![line1.span.end(), TextIndex(7)];
    assert_eq![line1.span.units, 7];
    assert_eq![line1.break_kind, TextBreakKind::Between];
    let line2 = wrap.next_line().unwrap();
    assert_eq![line2.span.start(), TextIndex(8)];
    assert_eq![line2.span.end(), TextIndex(13)];
    assert_eq![line2.span.units, 5];
    assert_eq![line2.break_kind, TextBreakKind::Between];
    assert_eq![wrap.next_line(), None];
}
#[test]
fn wrap_iter_skips_leading_glue() {
    let mut segs = [TextSegment::INIT; 8];
    let len = TextSegment::segments_from_chars("   alpha", &mut segs);
    let mut wrap = TextWrapIter::new(&segs[..len], 10);
    let line = wrap.next_line().unwrap();
    assert_eq![line.span.start(), TextIndex(3)];
    assert_eq![line.span.end(), TextIndex(8)];
    assert_eq![line.span.units, 5];
    assert_eq![wrap.next_line(), None];
}
#[test]
fn wrap_iter_hard_break_forces_line() {
    let mut segs = [TextSegment::INIT; 8];
    let len = TextSegment::segments_from_chars("one\ntwo", &mut segs);
    let mut wrap = TextWrapIter::new(&segs[..len], 20);
    let line1 = wrap.next_line().unwrap();
    assert_eq![line1.span.start(), TextIndex(0)];
    assert_eq![line1.span.end(), TextIndex(3)];
    assert_eq![line1.span.units, 3];
    assert_eq![line1.break_kind, TextBreakKind::Hard];
    let line2 = wrap.next_line().unwrap();
    assert_eq![line2.span.start(), TextIndex(4)];
    assert_eq![line2.span.end(), TextIndex(7)];
    assert_eq![line2.span.units, 3];
    assert_eq![line2.break_kind, TextBreakKind::Between];
    assert_eq![wrap.next_line(), None];
}
#[test]
fn wrap_iter_splits_long_box() {
    let mut segs = [TextSegment::INIT; 8];
    let len = TextSegment::segments_from_chars("abcdefghij", &mut segs);
    let mut wrap = TextWrapIter::new(&segs[..len], 4);
    let line1 = wrap.next_line().unwrap();
    assert_eq![line1.span.start(), TextIndex(0)];
    assert_eq![line1.span.end(), TextIndex(4)];
    assert_eq![line1.span.units, 4];
    assert_eq![line1.break_kind, TextBreakKind::Inside];
    let line2 = wrap.next_line().unwrap();
    assert_eq![line2.span.start(), TextIndex(4)];
    assert_eq![line2.span.end(), TextIndex(8)];
    assert_eq![line2.span.units, 4];
    assert_eq![line2.break_kind, TextBreakKind::Inside];
    let line3 = wrap.next_line().unwrap();
    assert_eq![line3.span.start(), TextIndex(8)];
    assert_eq![line3.span.end(), TextIndex(10)];
    assert_eq![line3.span.units, 2];
    assert_eq![line3.break_kind, TextBreakKind::Between];
    assert_eq![wrap.next_line(), None];
}
#[test]
fn wrap_iter_width_zero_returns_none() {
    let mut segs = [TextSegment::INIT; 4];
    let len = TextSegment::segments_from_chars("abc", &mut segs);
    let mut wrap = TextWrapIter::new(&segs[..len], 0);
    assert_eq![wrap.next_line(), None];
}
// TEMP DELETE TODO
// #[test]
// fn wraps_simple_paragraph() {
//     let lines = collect_lines("Me cubro junto a la puerta", 10);
//     assert_eq!(
//         lines,
//         vec![
//             ("Me cubro".to_string(), TextBreakKind::Between),
//             ("junto a la".to_string(), TextBreakKind::Between),
//             ("puerta".to_string(), TextBreakKind::Between),
//         ]
//     );
// }
// #[test]
// fn splits_long_word_inside() {
//     let lines = collect_lines("ventilación", 4);
//     assert_eq!(
//         lines,
//         vec![
//             ("vent".to_string(), TextBreakKind::Inside),
//             ("ilac".to_string(), TextBreakKind::Inside),
//             ("ión".to_string(), TextBreakKind::Between),
//         ]
//     );
// }
// #[test]
// fn wraps_mixed_text() {
//     let lines = collect_lines("hola ventilación mundo", 6);
//     assert_eq!(
//         lines,
//         vec![
//             ("hola".to_string(), TextBreakKind::Between),
//             ("ventil".to_string(), TextBreakKind::Inside),
//             ("ación".to_string(), TextBreakKind::Between),
//             ("mundo".to_string(), TextBreakKind::Between),
//         ]
//     );
// }
// #[test]
// fn wraps_if_paragraph() {
//     let text = "Me cubro junto a la puerta, evitando el ángulo de visión de la mirilla.";
//     let lines = collect_lines(text, 20);
//     assert_eq!(
//         lines,
//         vec![
//             ("Me cubro junto a la".to_string(), TextBreakKind::Between),
//             ("puerta, evitando el".to_string(), TextBreakKind::Between),
//             ("ángulo de visión de".to_string(), TextBreakKind::Between),
//             ("la mirilla.".to_string(), TextBreakKind::Between),
//         ]
//     );
// }
