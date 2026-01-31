// devela_base_core::text::layout::tests

use super::*;
use crate::ConstInitCore;

type TextSpans = [TextSpan; 4];

// test helper
fn run(symbols: &[TextSymbol], start: u32, extent: Option<u32>) -> (TextSpans, TextLayoutStep) {
    let layout = TextLayout;
    let mut spans: TextSpans = [TextSpan::INIT; 4];
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
    assert_eq!(spans[0].start.0, 0);
    assert_eq!(spans[0].end.0, 2);
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
    assert_eq!(spans[0].start.0, 0);
    assert_eq!(spans[0].end.0, 1);
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
    assert_eq!(spans[0].start.0, 0);
    assert_eq!(spans[0].end.0, 1);
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
    assert_eq!(spans[0].start.0, 1);
    assert_eq!(spans[0].end.0, 2);
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
