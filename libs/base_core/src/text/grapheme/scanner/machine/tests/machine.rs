// devela_base_core::text::grapheme::scanner::machine::tests::machine

use crate::{CharIter, GraphemeBoundary, GraphemeMachine, StringU8, array_init};

// The tests in this file are only for the public-facing `GraphemeCluster` API.
// The internal state machine implementation has its own tests under
// `tests::state`, where most of the interesting testing happens.

#[test]
#[cfg(feature = "__std")]
fn t01_grapheme_cluster_logic_directly() {
    use crate::{GraphemeBoundary, GraphemeMachine, StringU8, char_utf8};

    let input = "🧑‍🌾";
    let mut machine = GraphemeMachine::new();
    let mut remain = input;
    let mut grapheme = StringU8::<32>::new();
    let mut buf = [0u8; 4];

    // Replicate the scanner logic exactly
    while let Some((ch, len)) = char_utf8::from_str_with_len(remain) {
        let boundary = machine.next_char_utf8(ch);

        // This is the exact logic from our current next_grapheme_u8
        // grapheme.try_push_str(ch.as_str_into(&mut buf));
        // remain = &remain[len as usize..];
        // if boundary == GraphemeBoundary::Split {
        //     break;
        // }

        if boundary == GraphemeBoundary::Split && !grapheme.is_empty() {
            // This is the key difference: we break WITHOUT including the split character
            break;
        }

        // Add character AFTER checking split
        grapheme.try_push_str(ch.as_str_into(&mut buf));
        remain = &remain[len as usize..];
    }

    // Assert the result
    assert_eq!(
        grapheme.as_str(),
        "🧑‍🌾",
        "Should return complete ZWJ sequence, but got '{}' from input '{}'",
        grapheme.as_str(),
        input
    );
}

#[test]
#[cfg(feature = "__std")]
fn t02_multiple_graphemes_directly() {
    use crate::{GraphemeBoundary, GraphemeMachine, StringU8, char_utf8};

    let input = "H🧑‍🌾";
    let mut machine = GraphemeMachine::new();
    let mut remain = input;
    let mut graphemes = Vec::new();
    let mut buf = [0u8; 4];

    // First grapheme
    let mut grapheme = StringU8::<32>::new();
    while let Some((ch, len)) = char_utf8::from_str_with_len(remain) {
        let boundary = machine.next_char_utf8(ch);

        if boundary == GraphemeBoundary::Split && !grapheme.is_empty() {
            break;
        }

        grapheme.try_push_str(ch.as_str_into(&mut buf));
        remain = &remain[len as usize..];
    }
    graphemes.push(grapheme.as_str().to_string());

    // Second grapheme
    let mut grapheme = StringU8::<32>::new();
    while let Some((ch, len)) = char_utf8::from_str_with_len(remain) {
        let boundary = machine.next_char_utf8(ch);

        if boundary == GraphemeBoundary::Split && !grapheme.is_empty() {
            break;
        }

        grapheme.try_push_str(ch.as_str_into(&mut buf));
        remain = &remain[len as usize..];
    }
    graphemes.push(grapheme.as_str().to_string());

    assert_eq!(graphemes[0], "H");
    assert_eq!(graphemes[1], "🧑‍🌾");
}

// #[test]
// #[cfg(feature = "__std")]
// fn test_zwj_char_by_char() {
//     use crate::{GraphemeMachine, GraphemeBoundary, char_utf8};
//
//     let mut machine = GraphemeMachine::new();
//     let mut buf = [0u8; 4];
//
//     // Manually process the exact sequence: 🧑, ZWJ, 🌾
//     let chars = [
//         char_utf8::from_char('🧑'),    // U+1F9D1
//         char_utf8::from_char('‍'),     // U+200D (ZWJ)
//         char_utf8::from_char('🌾'),    // U+1F33E
//     ];
//
//     eprintln!("=== Manual ZWJ processing ===");
//     for (i, &ch) in chars.iter().enumerate() {
//         let boundary = machine.next_char_utf8(ch);
//         eprintln!("Char {}: '{}', boundary: {:?}, state: {:?}",
//                  i, ch.as_str_into(&mut buf), boundary, machine.state);
//     }
//
//     // The boundaries should be: Continue, Continue, Split
//     // Or maybe: Continue, Continue, Continue (if split comes after)
// }

// #[test]
// #[cfg(feature = "__std")]
// fn test_zwj_sequence_directly() {
//     use crate::{GraphemeMachine, GraphemeBoundary, char_utf8};
//
//     let input = "🧑‍🌾";
//     let mut machine = GraphemeMachine::new();
//
//     // Process each character and print the machine state and boundaries
//     let mut buf = [0u8; 4];
//
//     println!("=== Processing ZWJ sequence: '🧑‍🌾' ===");
//
//     for (i, ch) in input.chars().enumerate() {
//         let ch_utf8 = char_utf8::from_char(ch);
//         let boundary = machine.next_char_utf8(ch_utf8);
//
//         println!("Step {}: char='{}' (U+{:04X}), boundary={:?}, machine_state={:?}",
//                  i, ch, ch as u32, boundary, machine.state);
//     }
//
//     // Now test with our scanner
//     println!("\n=== Testing with GraphemeScanner ===");
//     let mut machine2 = GraphemeMachine::new();
//     let mut scanner = crate::GraphemeScanner::<char_utf8>::new(&mut machine2, input);
//
//     if let Some(grapheme) = scanner.next_grapheme_u8::<32>() {
//         println!("Scanner returned: '{}'", grapheme.as_str());
//         assert_eq!(grapheme.as_str(), "🧑‍🌾", "Scanner should return complete ZWJ sequence");
//     } else {
//         panic!("Scanner returned None");
//     }
// }

#[test]
#[rustfmt::skip]
fn core_basics() {
    let mut clusters: [StringU8::<16>; 32] =
        array_init![default [StringU8::<16>; 32], "safe", "unsafe_array"];
    let mut current_cluster = StringU8::<16>::new();
    let mut machine = GraphemeMachine::new();
    let mut buf = [0u8; 4];
    let input = "Hello!\r\nBeep 🧑‍🌾";
    let mut iter = CharIter::<&str>::new(input);
    let mut n = 0;

    while let Some(c) = iter.next_char_utf8() {
        if machine.next_char_utf8(c) == GraphemeBoundary::Split {
            if !current_cluster.is_empty() {
                clusters[n] = current_cluster;
                current_cluster.clear();
                n += 1;
            }
        }
        current_cluster.push_str(c.as_str_into(&mut buf));
    }
    if !current_cluster.is_empty() { 
        clusters[n] = current_cluster; 
    }
    let expected = ["H", "e", "l", "l", "o", "!", "\r\n", "B", "e", "e", "p", " ", "🧑‍🌾"];
    for (i, (actual, expected_str)) in clusters.iter().zip(expected.iter()).enumerate() {
        assert_eq!(actual, expected_str, "mismatch at index {}", i);
    }
}

// NOTE: left as a modle to make previous briefer in the FUTURE
#[test]
#[cfg(feature = "__std")]
fn alloc_basics() {
    let mut clusters: Vec<String> = Vec::new();
    let mut current_cluster = String::new();
    let mut machine = GraphemeMachine::new();
    let mut buf = [0u8; 4];
    let input = "🧑‍🌾";
    let mut iter = CharIter::<&str>::new(input);

    while let Some(c) = iter.next_char_utf8() {
        if machine.next_char_utf8(c) == GraphemeBoundary::Split {
            if !current_cluster.is_empty() {
                clusters.push(current_cluster.clone());
                current_cluster.clear();
            }
        }
        current_cluster.push_str(c.as_str_into(&mut buf));
    }
    if !current_cluster.is_empty() {
        clusters.push(current_cluster.clone());
    }
    let expected = ["🧑‍🌾"];
    assert_eq!(clusters, &expected);
}

#[test]
fn end_of_input() {
    let mut machine = GraphemeMachine::new();
    let input = "Hello!\r\nBeep 🧑‍🌾";

    let mut iter = CharIter::<&str>::new(input);
    while let Some(c) = iter.next_char_utf8() {
        // WORKS
        machine.end_of_input(); // effectively forces a cluster boundary
        if machine.next_char_utf8(c) != GraphemeBoundary::Split {
            panic!("non-split after end_of_input came before {c:?}");
        }
    }
}

#[test]
#[cfg(feature = "__std")]
fn next_char_from_str() {
    use GraphemeBoundary::*;
    let mut machine = GraphemeMachine::new();
    let input = "Hello!\r\nBeep 🧑‍🌾";
    let got: Vec<_> = machine.next_char_from_str(input).collect();
    assert_eq!(
        got,
        &[
            (Split, 'H'),
            (Split, 'e'),
            (Split, 'l'),
            (Split, 'l'),
            (Split, 'o'),
            (Split, '!'),
            (Split, '\r'),
            (Continue, '\n'),
            (Split, 'B'),
            (Split, 'e'),
            (Split, 'e'),
            (Split, 'p'),
            (Split, ' '),
            (Split, '🧑'),
            (Continue, '\u{200D}'), // zero-width joiner
            (Continue, '🌾'),
        ]
    );
}

#[test]
const fn const_iter() {
    const INPUT: &str = "Hello!\r\nBeep 🧑‍🌾";
    const CHAR: char = const {
        let mut machine = GraphemeMachine::new();
        let mut iter = machine.next_char_from_str(INPUT);

        let mut c = '\0';
        let mut i = 0;
        while i < 9 {
            i += 1;
            if let Some((cluster, character)) = iter.next() {
                if cluster.eq(GraphemeBoundary::Split) {
                    c = character;
                }
            }
        }
        c
    };
    assert![CHAR == 'B'];
}
