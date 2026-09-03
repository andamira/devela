// devela/src/text/layout/_test.rs

use super::*;
use crate::{ConstInit, TextCursor, TextIndex, TextUnit};

type TextLayoutSpans = [TextLayoutSpan; 4];

// test helper
fn run(
    symbols: &[TextSymbol],
    start: u32,
    extent: Option<u32>,
) -> (TextLayoutSpans, TextLayoutStep) {
    let layout = TextLayout;
    let mut spans: TextLayoutSpans = [TextLayoutSpan::INIT; 4];
    let step = layout.step(
        symbols,
        TextCursor { index: TextIndex(start) },
        extent.map(TextUnit::from),
        &mut spans,
    );
    (spans, step)
}

/// Example: Atomic fit
#[test]
fn atomic_full_fit() {
    let symbols = [
        TextSymbol { units: 3, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 0, Some(5));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 2);
    assert_eq!(spans[0].units, 5);
    assert_eq!(step.fit, TextFit::Full);
    assert!(step.carry.is_none());
}
/// Example: Atomic overflow
#[test]
fn atomic_partial_fit() {
    let symbols = [
        TextSymbol { units: 3, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 0, Some(4));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 1);
    assert_eq!(spans[0].units, 3);
    assert_eq!(step.fit, TextFit::Partial);
    assert_eq!(step.carry.unwrap().index.0, 1);
}
/// Example: Breakable partial consumption
#[test]
fn breakable_partial_consumption() {
    let symbols = [TextSymbol { units: 6, cohesion: TextCohesion::Breakable }];
    let (spans, step) = run(&symbols, 0, Some(4));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 1);
    assert_eq!(spans[0].units, 4);
    assert_eq!(step.fit, TextFit::Partial);
    assert_eq!(step.carry.unwrap().index.0, 0);
}
/// Example: Elidable symbol skipped
#[test]
fn elidable_is_skipped() {
    let symbols = [
        TextSymbol { units: 2, cohesion: TextCohesion::Elidable },
        TextSymbol { units: 5, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 0, Some(5));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 1);
    assert_eq!(spans[0].end().0, 2);
    assert_eq!(spans[0].units, 5);
    assert_eq!(step.fit, TextFit::Full);
}
/// Example: Nothing fits
#[test]
fn nothing_fits() {
    let symbols = [
        TextSymbol { units: 3, cohesion: TextCohesion::Elidable },
        TextSymbol { units: 4, cohesion: TextCohesion::Atomic },
    ];
    let (_spans, step) = run(&symbols, 0, Some(2));
    assert_eq!(step.span_count, 0);
    assert_eq!(step.fit, TextFit::None);
    assert_eq!(step.carry.unwrap().index.0, 1);
}
/// Example: unlimited extent consumes all required symbols.
#[test]
fn unlimited_extent_full_fit() {
    let symbols = [
        TextSymbol { units: 3, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 4, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 0, None);
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 3);
    assert_eq!(spans[0].units, 9);
    assert_eq!(step.consumed, 9);
    assert_eq!(step.fit, TextFit::Full);
    assert!(step.carry.is_none());
}
/// Example: starting from a non-zero cursor.
#[test]
fn starts_from_cursor() {
    let symbols = [
        TextSymbol { units: 1, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 3, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 1, Some(5));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 1);
    assert_eq!(spans[0].end().0, 3);
    assert_eq!(spans[0].units, 5);
    assert_eq!(step.consumed, 5);
    assert_eq!(step.fit, TextFit::Full);
    assert!(step.carry.is_none());
}
/// Example: zero extent consumes nothing but preserves carry.
#[test]
fn zero_extent_consumes_nothing() {
    let symbols = [
        TextSymbol { units: 1, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 1, cohesion: TextCohesion::Atomic },
    ];
    let (_spans, step) = run(&symbols, 0, Some(0));
    assert_eq!(step.span_count, 0);
    assert_eq!(step.consumed, 0);
    assert_eq!(step.fit, TextFit::None);
    assert_eq!(step.carry.unwrap().index.0, 0);
}
/// Example: breakable exact fit advances past the symbol.
#[test]
fn breakable_exact_fit() {
    let symbols = [
        TextSymbol { units: 4, cohesion: TextCohesion::Breakable },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 0, Some(4));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 1);
    assert_eq!(spans[0].units, 4);
    assert_eq!(step.consumed, 4);
    assert_eq!(step.fit, TextFit::Partial);
    assert_eq!(step.carry.unwrap().index.0, 1);
}
/// Example: elidable symbols inside a span do not consume units.
#[test]
fn elidable_inside_span_does_not_consume_units() {
    let symbols = [
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 9, cohesion: TextCohesion::Elidable },
        TextSymbol { units: 3, cohesion: TextCohesion::Atomic },
    ];
    let (spans, step) = run(&symbols, 0, Some(5));
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 3);
    assert_eq!(spans[0].units, 5);
    assert_eq!(step.consumed, 5);
    assert_eq!(step.fit, TextFit::Full);
    assert!(step.carry.is_none());
}
/// Example: default symbol config is whitespace monospace with no elision.
#[test]
fn symbol_config_default() {
    let cfg = TextSymbolConfig::default();
    assert_eq!(cfg.break_mode, TextBreakMode::Whitespace);
    assert_eq!(cfg.width_mode, TextelWidthMode::Mono);
    assert_eq!(cfg.elide_mode, TextElideMode::None);
    assert_eq!(cfg, TextSymbolConfig::DEFAULT);
    assert_eq!(cfg, TextSymbolConfig::whitespace_mono());
}
/// Example: symbol config constructors.
#[test]
fn symbol_config_constructors() {
    let word = TextSymbolConfig::word_mono();
    assert_eq!(word.break_mode, TextBreakMode::Word);
    assert_eq!(word.width_mode, TextelWidthMode::Mono);
    assert_eq!(word.elide_mode, TextElideMode::None);
    let grapheme = TextSymbolConfig::grapheme_mono();
    assert_eq!(grapheme.break_mode, TextBreakMode::Grapheme);
    assert_eq!(grapheme.width_mode, TextelWidthMode::Mono);
    assert_eq!(grapheme.elide_mode, TextElideMode::None);
}
/// Example: symbol config builders.
#[test]
fn symbol_config_builders() {
    let cfg = TextSymbolConfig::DEFAULT
        .with_break_mode(TextBreakMode::Grapheme)
        .with_width_mode(TextelWidthMode::EastAsian)
        .with_elide_mode(TextElideMode::Whitespace);
    assert_eq!(cfg.break_mode, TextBreakMode::Grapheme);
    assert_eq!(cfg.width_mode, TextelWidthMode::EastAsian);
    assert_eq!(cfg.elide_mode, TextElideMode::Whitespace);
}
/// Example: line iterator repeats fixed-width layout steps.
#[test]
fn line_iter_repeats_fixed_width_steps() {
    let layout = TextLayout;
    let symbols = [
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
    ];
    let mut iter = TextLineIter::new(&layout, &symbols, 4);
    let mut spans: TextLayoutSpans = [TextLayoutSpan::INIT; 4];
    let step1 = iter.next(&mut spans).unwrap();
    assert_eq!(step1.span_count, 1);
    assert_eq!(spans[0].start().0, 0);
    assert_eq!(spans[0].end().0, 2);
    assert_eq!(spans[0].units, 4);
    assert_eq!(step1.fit, TextFit::Partial);
    assert_eq!(iter.cursor().index.0, 2);
    assert!(!iter.is_done());
    let step2 = iter.next(&mut spans).unwrap();
    assert_eq!(step2.span_count, 1);
    assert_eq!(spans[0].start().0, 2);
    assert_eq!(spans[0].end().0, 3);
    assert_eq!(spans[0].units, 2);
    assert_eq!(step2.fit, TextFit::Full);
    assert!(iter.is_done());
    assert!(iter.next(&mut spans).is_none());
}
/// Example: line iterator can start from a cursor.
#[test]
fn line_iter_from_cursor() {
    let layout = TextLayout;
    let symbols = [
        TextSymbol { units: 2, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 3, cohesion: TextCohesion::Atomic },
        TextSymbol { units: 4, cohesion: TextCohesion::Atomic },
    ];
    let mut iter =
        TextLineIter::from_cursor(&layout, &symbols, TextCursor { index: TextIndex(1) }, 7);
    let mut spans: TextLayoutSpans = [TextLayoutSpan::INIT; 4];
    let step = iter.next(&mut spans).unwrap();
    assert_eq!(step.span_count, 1);
    assert_eq!(spans[0].start().0, 1);
    assert_eq!(spans[0].end().0, 3);
    assert_eq!(spans[0].units, 7);
    assert_eq!(step.fit, TextFit::Full);
    assert!(iter.is_done());
}
/// Example: line iterator width zero produces no step.
#[test]
fn line_iter_width_zero() {
    let layout = TextLayout;
    let symbols = [TextSymbol { units: 1, cohesion: TextCohesion::Atomic }];
    let mut iter = TextLineIter::new(&layout, &symbols, 0);
    let mut spans: TextLayoutSpans = [TextLayoutSpan::INIT; 4];
    assert!(iter.next(&mut spans).is_none());
    assert!(iter.is_done());
}
