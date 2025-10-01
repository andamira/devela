// devela_base_core::text::grapheme::scanner::machine::tests::state

use super::super::GraphemeMachineState;
#[cfg(feature = "__std")]
use super::trie::UNICODE_GRAPHEME_CLUSTER_TESTS;
use crate::{GraphemePropCb, GraphemeProps, char_utf8};

#[test]
fn character_categories() {
    // This is a very non-exhaustive and mostly arbitrary set of characters
    // to test just as a signal that the property lookup code is generally
    // working. `unicode_test_table` is a more thorough test that covers
    // both individual character categorization and the segmentation
    // state machine.

    use GraphemePropCb::*;
    fn prop(c: char) -> GraphemePropCb {
        GraphemeProps::for_char_utf8(char_utf8::from_char(c)).gcb_property()
    }

    assert_eq!(prop(' '), None);
    assert_eq!(prop('\r'), CR);
    assert_eq!(prop('\n'), LF);
    assert_eq!(prop('\t'), Control);
    assert_eq!(prop('\u{200D}'), Zwj);
    assert_eq!(prop('\u{1F1E6}'), RegionalIndicator);
    assert_eq!(prop('\u{1F9D1}'), ExtendedPictographic);
    assert_eq!(prop('\u{1F33E}'), ExtendedPictographic);
    assert_eq!(prop('\u{0C41}'), SpacingMark);
}

#[test]
#[cfg(feature = "__std")]
fn crlf() {
    use GraphemeMachineState::*;
    let got: Vec<_> = transitions(&[
        GraphemeProps::None,
        GraphemeProps::CR,
        GraphemeProps::LF,
        GraphemeProps::None,
    ])
    .collect();
    assert_eq!(
        got,
        &[
            (true, GraphemeProps::None, Base),
            (true, GraphemeProps::CR, Base),
            (false, GraphemeProps::LF, Base),
            (true, GraphemeProps::None, Base)
        ]
    );
}

#[test]
#[cfg(feature = "__std")]
fn emoji_flags() {
    use GraphemeMachineState::*;
    let got: Vec<_> = transitions(&[
        GraphemeProps::None,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::None,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::None,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::None,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::RegionalIndicator,
        GraphemeProps::None,
    ])
    .collect();
    assert_eq!(
        got,
        &[
            (true, GraphemeProps::None, Base),
            (true, GraphemeProps::RegionalIndicator, AwaitRegionalPair),
            (true, GraphemeProps::None, Base),
            (true, GraphemeProps::RegionalIndicator, AwaitRegionalPair),
            (false, GraphemeProps::RegionalIndicator, Base),
            (true, GraphemeProps::None, Base),
            (true, GraphemeProps::RegionalIndicator, AwaitRegionalPair),
            (false, GraphemeProps::RegionalIndicator, Base),
            (true, GraphemeProps::RegionalIndicator, AwaitRegionalPair),
            (true, GraphemeProps::None, Base),
            (true, GraphemeProps::RegionalIndicator, AwaitRegionalPair),
            (false, GraphemeProps::RegionalIndicator, Base),
            (true, GraphemeProps::RegionalIndicator, AwaitRegionalPair),
            (false, GraphemeProps::RegionalIndicator, Base),
            (true, GraphemeProps::None, Base),
        ]
    );
}

#[test]
#[cfg(feature = "__std")]
fn unicode_test_table() {
    let mut failures = 0;
    let mut buf = [0u8; 4];

    for test in UNICODE_GRAPHEME_CLUSTER_TESTS {
        let input = str::from_utf8(test.input).expect("invalid UTF-8 in test input");
        let mut remain = input;
        let mut state = GraphemeMachineState::Base;
        let mut prev: Option<GraphemeProps> = None;
        let mut got: Vec<Box<[u8]>> = Vec::new();
        let mut current: Vec<u8> = Vec::new();
        loop {
            let Some((next, len)) = char_utf8::from_str_with_len(remain) else {
                break;
            };
            let next_props = GraphemeProps::for_char_utf8(next);
            let (boundary, next_state) = state.transition(prev, next_props);
            if boundary {
                if !current.is_empty() {
                    let boxed = current.clone().into_boxed_slice();
                    got.push(boxed);
                    current.clear();
                }
            }
            // current.extend_from_slice(next.as_bytes());
            // current.extend_from_slice(&next.to_utf8_bytes());
            current.extend_from_slice(next.as_bytes_into(&mut buf));
            // remain = rest;
            remain = &remain[len as usize..];
            prev = Some(next_props);
            state = next_state;
        }
        if !current.is_empty() {
            let boxed = current.clone().into_boxed_slice();
            got.push(boxed);
            current.clear();
        }
        if !result_matches(&got, test.expected) {
            println!("- test failed: {}", test.desc);
            println!("  input: {:x?}", test.input);
            println!("  got:   {:x?}", got);
            println!("  want:  {:x?}", test.expected);
            failures += 1;
        }
    }
    if failures != 0 {
        panic!("{failures} tests failed");
    }

    fn result_matches(got: &Vec<Box<[u8]>>, want: &[&[u8]]) -> bool {
        if got.len() != want.len() {
            return false;
        }
        for (got, want) in got.iter().zip(want.iter().copied()) {
            if got.len() != want.len() {
                return false;
            }
            for (got, want) in got.iter().zip(want) {
                if got != want {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
#[cfg(feature = "__std")]
fn emoji_extend() {
    use GraphemeMachineState::*;
    let got: Vec<_> = transitions(&[
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::Zwj,
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::Extend,
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::Extend,
        GraphemeProps::Zwj,
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::Extend,
        GraphemeProps::Extend,
        GraphemeProps::Zwj,
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
        //
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::Extend,
        GraphemeProps::Extend,
        GraphemeProps::Zwj,
        GraphemeProps::Extend,
        GraphemeProps::ExtendedPictographic,
        GraphemeProps::None,
    ])
    .collect();
    assert_eq!(
        got,
        &[
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (false, GraphemeProps::Zwj, AfterZwj),
            (false, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (false, GraphemeProps::Extend, BeforeZwj),
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (false, GraphemeProps::Extend, BeforeZwj),
            (false, GraphemeProps::Zwj, AfterZwj),
            (false, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (false, GraphemeProps::Extend, BeforeZwj),
            (false, GraphemeProps::Extend, BeforeZwj),
            (false, GraphemeProps::Zwj, AfterZwj),
            (false, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
            //
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (false, GraphemeProps::Extend, BeforeZwj),
            (false, GraphemeProps::Extend, BeforeZwj),
            (false, GraphemeProps::Zwj, AfterZwj),
            (false, GraphemeProps::Extend, Base),
            (true, GraphemeProps::ExtendedPictographic, BeforeZwj),
            (true, GraphemeProps::None, Base),
        ]
    );
}

fn transitions(
    cats: &[GraphemeProps],
) -> impl Iterator<Item = (bool, GraphemeProps, GraphemeMachineState)> + use<'_> {
    struct Iter<'a> {
        remain: &'a [GraphemeProps],
        state: GraphemeMachineState,
        prev: Option<GraphemeProps>,
    }
    impl<'a> Iterator for Iter<'a> {
        type Item = (bool, GraphemeProps, GraphemeMachineState);

        fn next(&mut self) -> Option<Self::Item> {
            let Some((next, remain)) = self.remain.split_first() else {
                return None;
            };
            let prev = self.prev;
            let next = *next;
            let (split, next_state) = self.state.transition(prev, next);
            self.remain = remain;
            self.state = next_state;
            self.prev = Some(next);
            Some((split, next, next_state))
        }
    }

    Iter {
        remain: cats,
        state: GraphemeMachineState::Base,
        prev: None,
    }
}
