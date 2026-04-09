// devela::text::grapheme::scanner::machine::tests::trie

pub(super) struct SegmentationTest {
    pub desc: &'static str,
    pub input: &'static [u8],
    pub expected: &'static [&'static [u8]],
}

pub(super) static UNICODE_GRAPHEME_CLUSTER_TESTS: &[SegmentationTest] = &[
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"  ", // "  "
        expected: &[
            b" ", // " "
            b" ", // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b" \xcc\x88 ", // " ̈ "
        expected: &[
            b" \xcc\x88", // " ̈"
            b" ",         // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b" \r", // " \r"
        expected: &[
            b" ",  // " "
            b"\r", // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b" \xcc\x88\r", // " ̈\r"
        expected: &[
            b" \xcc\x88", // " ̈"
            b"\r",        // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b" \n", // " \n"
        expected: &[
            b" ",  // " "
            b"\n", // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b" \xcc\x88\n", // " ̈\n"
        expected: &[
            b" \xcc\x88", // " ̈"
            b"\n",        // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b" \x01", // " \x01"
        expected: &[
            b" ",    // " "
            b"\x01", // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b" \xcc\x88\x01", // " ̈\x01"
        expected: &[
            b" \xcc\x88", // " ̈"
            b"\x01",      // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b" \xe2\x80\x8c", // " \u200c"
        expected: &[
            b" \xe2\x80\x8c", // " \u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b" \xcc\x88\xe2\x80\x8c", // " ̈\u200c"
        expected: &[
            b" \xcc\x88\xe2\x80\x8c", // " ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b" \xf0\x9f\x87\xa6", // " 🇦"
        expected: &[
            b" ",                // " "
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b" \xcc\x88\xf0\x9f\x87\xa6", // " ̈🇦"
        expected: &[
            b" \xcc\x88",        // " ̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b" \xd8\x80", // " \u0600"
        expected: &[
            b" ",        // " "
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b" \xcc\x88\xd8\x80", // " ̈\u0600"
        expected: &[
            b" \xcc\x88", // " ̈"
            b"\xd8\x80",  // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b" \xe0\xa8\x83", // " ਃ"
        expected: &[
            b" \xe0\xa8\x83", // " ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xa8\x83", // " ̈ਃ"
        expected: &[
            b" \xcc\x88\xe0\xa8\x83", // " ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b" \xe1\x84\x80", // " ᄀ"
        expected: &[
            b" ",            // " "
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b" \xcc\x88\xe1\x84\x80", // " ̈ᄀ"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b" \xe1\x85\xa0", // " ᅠ"
        expected: &[
            b" ",            // " "
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b" \xcc\x88\xe1\x85\xa0", // " ̈ᅠ"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b" \xe1\x86\xa8", // " ᆨ"
        expected: &[
            b" ",            // " "
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b" \xcc\x88\xe1\x86\xa8", // " ̈ᆨ"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b" \xea\xb0\x80", // " 가"
        expected: &[
            b" ",            // " "
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b" \xcc\x88\xea\xb0\x80", // " ̈가"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b" \xea\xb0\x81", // " 각"
        expected: &[
            b" ",            // " "
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b" \xcc\x88\xea\xb0\x81", // " ̈각"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b" \xe0\xa4\x83", // " ः"
        expected: &[
            b" \xe0\xa4\x83", // " ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xa4\x83", // " ̈ः"
        expected: &[
            b" \xcc\x88\xe0\xa4\x83", // " ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b" \xe0\xa4\x84", // " ऄ"
        expected: &[
            b" ",            // " "
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xa4\x84", // " ̈ऄ"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b" \xe0\xb5\x8e", // " ൎ"
        expected: &[
            b" ",            // " "
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xb5\x8e", // " ̈ൎ"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b" \xe0\xa4\x95", // " क"
        expected: &[
            b" ",            // " "
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xa4\x95", // " ̈क"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b" \xe2\x8c\x9a", // " ⌚"
        expected: &[
            b" ",            // " "
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b" \xcc\x88\xe2\x8c\x9a", // " ̈⌚"
        expected: &[
            b" \xcc\x88",    // " ̈"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b" \xcc\x80", // " ̀"
        expected: &[
            b" \xcc\x80", // " ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b" \xcc\x88\xcc\x80", // " ̈̀"
        expected: &[
            b" \xcc\x88\xcc\x80", // " ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b" \xe0\xa4\x80", // " ऀ"
        expected: &[
            b" \xe0\xa4\x80", // " ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xa4\x80", // " ̈ऀ"
        expected: &[
            b" \xcc\x88\xe0\xa4\x80", // " ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b" \xe0\xa5\x8d", // " ्"
        expected: &[
            b" \xe0\xa5\x8d", // " ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b" \xcc\x88\xe0\xa5\x8d", // " ्̈"
        expected: &[
            b" \xcc\x88\xe0\xa5\x8d", // " ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b" \xe2\x80\x8d", // " \u200d"
        expected: &[
            b" \xe2\x80\x8d", // " \u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b" \xcc\x88\xe2\x80\x8d", // " ̈\u200d"
        expected: &[
            b" \xcc\x88\xe2\x80\x8d", // " ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b" \xcd\xb8", // " \u0378"
        expected: &[
            b" ",        // " "
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b" \xcc\x88\xcd\xb8", // " ̈\u0378"
        expected: &[
            b" \xcc\x88", // " ̈"
            b"\xcd\xb8",  // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] SPACE (Other) ÷ [0.3]",
        input: b"\r ", // "\r "
        expected: &[
            b"\r", // "\r"
            b" ",  // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\r\xcc\x88 ", // "\r̈ "
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x88", // "̈"
            b" ",        // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\r\r", // "\r\r"
        expected: &[
            b"\r", // "\r"
            b"\r", // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\r\xcc\x88\r", // "\r̈\r"
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x88", // "̈"
            b"\r",       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) × [3.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\r\n", // "\r\n"
        expected: &[
            b"\r\n", // "\r\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\r\xcc\x88\n", // "\r̈\n"
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x88", // "̈"
            b"\n",       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\r\x01", // "\r\x01"
        expected: &[
            b"\r",   // "\r"
            b"\x01", // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\r\xcc\x88\x01", // "\r̈\x01"
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x88", // "̈"
            b"\x01",     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\r\xe2\x80\x8c", // "\r\u200c"
        expected: &[
            b"\r",           // "\r"
            b"\xe2\x80\x8c", // "\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\r\xcc\x88\xe2\x80\x8c", // "\r̈\u200c"
        expected: &[
            b"\r",                   // "\r"
            b"\xcc\x88\xe2\x80\x8c", // "̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\r\xf0\x9f\x87\xa6", // "\r🇦"
        expected: &[
            b"\r",               // "\r"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\r\xcc\x88\xf0\x9f\x87\xa6", // "\r̈🇦"
        expected: &[
            b"\r",               // "\r"
            b"\xcc\x88",         // "̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\r\xd8\x80", // "\r\u0600"
        expected: &[
            b"\r",       // "\r"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\r\xcc\x88\xd8\x80", // "\r̈\u0600"
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x88", // "̈"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\r\xe0\xa8\x83", // "\rਃ"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xa8\x83", // "ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xa8\x83", // "\r̈ਃ"
        expected: &[
            b"\r",                   // "\r"
            b"\xcc\x88\xe0\xa8\x83", // "̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\r\xe1\x84\x80", // "\rᄀ"
        expected: &[
            b"\r",           // "\r"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\r\xcc\x88\xe1\x84\x80", // "\r̈ᄀ"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\r\xe1\x85\xa0", // "\rᅠ"
        expected: &[
            b"\r",           // "\r"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\r\xcc\x88\xe1\x85\xa0", // "\r̈ᅠ"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\r\xe1\x86\xa8", // "\rᆨ"
        expected: &[
            b"\r",           // "\r"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\r\xcc\x88\xe1\x86\xa8", // "\r̈ᆨ"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\r\xea\xb0\x80", // "\r가"
        expected: &[
            b"\r",           // "\r"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\r\xcc\x88\xea\xb0\x80", // "\r̈가"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\r\xea\xb0\x81", // "\r각"
        expected: &[
            b"\r",           // "\r"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\r\xcc\x88\xea\xb0\x81", // "\r̈각"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\r\xe0\xa4\x83", // "\rः"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xa4\x83", // "ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xa4\x83", // "\r̈ः"
        expected: &[
            b"\r",                   // "\r"
            b"\xcc\x88\xe0\xa4\x83", // "̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\r\xe0\xa4\x84", // "\rऄ"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xa4\x84", // "\r̈ऄ"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\r\xe0\xb5\x8e", // "\rൎ"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xb5\x8e", // "\r̈ൎ"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\r\xe0\xa4\x95", // "\rक"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xa4\x95", // "\r̈क"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\r\xe2\x8c\x9a", // "\r⌚"
        expected: &[
            b"\r",           // "\r"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\r\xcc\x88\xe2\x8c\x9a", // "\r̈⌚"
        expected: &[
            b"\r",           // "\r"
            b"\xcc\x88",     // "̈"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xcc\x80", // "\r̀"
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x80", // "̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xcc\x88\xcc\x80", // "\r̈̀"
        expected: &[
            b"\r",               // "\r"
            b"\xcc\x88\xcc\x80", // "̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xe0\xa4\x80", // "\rऀ"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xa4\x80", // "ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xa4\x80", // "\r̈ऀ"
        expected: &[
            b"\r",                   // "\r"
            b"\xcc\x88\xe0\xa4\x80", // "̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xe0\xa5\x8d", // "\r्"
        expected: &[
            b"\r",           // "\r"
            b"\xe0\xa5\x8d", // "्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xcc\x88\xe0\xa5\x8d", // "\r्̈"
        expected: &[
            b"\r",                   // "\r"
            b"\xcc\x88\xe0\xa5\x8d", // "्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xe2\x80\x8d", // "\r\u200d"
        expected: &[
            b"\r",           // "\r"
            b"\xe2\x80\x8d", // "\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\r\xcc\x88\xe2\x80\x8d", // "\r̈\u200d"
        expected: &[
            b"\r",                   // "\r"
            b"\xcc\x88\xe2\x80\x8d", // "̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\r\xcd\xb8", // "\r\u0378"
        expected: &[
            b"\r",       // "\r"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\r\xcc\x88\xcd\xb8", // "\r̈\u0378"
        expected: &[
            b"\r",       // "\r"
            b"\xcc\x88", // "̈"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] SPACE (Other) ÷ [0.3]",
        input: b"\n ", // "\n "
        expected: &[
            b"\n", // "\n"
            b" ",  // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\n\xcc\x88 ", // "\n̈ "
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
            b" ",        // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\n\r", // "\n\r"
        expected: &[
            b"\n", // "\n"
            b"\r", // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\n\xcc\x88\r", // "\n̈\r"
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
            b"\r",       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\n\n", // "\n\n"
        expected: &[
            b"\n", // "\n"
            b"\n", // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\n\xcc\x88\n", // "\n̈\n"
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
            b"\n",       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\n\x01", // "\n\x01"
        expected: &[
            b"\n",   // "\n"
            b"\x01", // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\n\xcc\x88\x01", // "\n̈\x01"
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
            b"\x01",     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\n\xe2\x80\x8c", // "\n\u200c"
        expected: &[
            b"\n",           // "\n"
            b"\xe2\x80\x8c", // "\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\n\xcc\x88\xe2\x80\x8c", // "\n̈\u200c"
        expected: &[
            b"\n",                   // "\n"
            b"\xcc\x88\xe2\x80\x8c", // "̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\n\xf0\x9f\x87\xa6", // "\n🇦"
        expected: &[
            b"\n",               // "\n"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\n\xcc\x88\xf0\x9f\x87\xa6", // "\n̈🇦"
        expected: &[
            b"\n",               // "\n"
            b"\xcc\x88",         // "̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\n\xd8\x80", // "\n\u0600"
        expected: &[
            b"\n",       // "\n"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\n\xcc\x88\xd8\x80", // "\n̈\u0600"
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\n\xe0\xa8\x83", // "\nਃ"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xa8\x83", // "ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xa8\x83", // "\n̈ਃ"
        expected: &[
            b"\n",                   // "\n"
            b"\xcc\x88\xe0\xa8\x83", // "̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\n\xe1\x84\x80", // "\nᄀ"
        expected: &[
            b"\n",           // "\n"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\n\xcc\x88\xe1\x84\x80", // "\n̈ᄀ"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\n\xe1\x85\xa0", // "\nᅠ"
        expected: &[
            b"\n",           // "\n"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\n\xcc\x88\xe1\x85\xa0", // "\n̈ᅠ"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\n\xe1\x86\xa8", // "\nᆨ"
        expected: &[
            b"\n",           // "\n"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\n\xcc\x88\xe1\x86\xa8", // "\n̈ᆨ"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\n\xea\xb0\x80", // "\n가"
        expected: &[
            b"\n",           // "\n"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\n\xcc\x88\xea\xb0\x80", // "\n̈가"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\n\xea\xb0\x81", // "\n각"
        expected: &[
            b"\n",           // "\n"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\n\xcc\x88\xea\xb0\x81", // "\n̈각"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\n\xe0\xa4\x83", // "\nः"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xa4\x83", // "ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xa4\x83", // "\n̈ः"
        expected: &[
            b"\n",                   // "\n"
            b"\xcc\x88\xe0\xa4\x83", // "̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\n\xe0\xa4\x84", // "\nऄ"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xa4\x84", // "\n̈ऄ"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\n\xe0\xb5\x8e", // "\nൎ"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xb5\x8e", // "\n̈ൎ"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\n\xe0\xa4\x95", // "\nक"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xa4\x95", // "\n̈क"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\n\xe2\x8c\x9a", // "\n⌚"
        expected: &[
            b"\n",           // "\n"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\n\xcc\x88\xe2\x8c\x9a", // "\n̈⌚"
        expected: &[
            b"\n",           // "\n"
            b"\xcc\x88",     // "̈"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xcc\x80", // "\ǹ"
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x80", // "̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xcc\x88\xcc\x80", // "\n̈̀"
        expected: &[
            b"\n",               // "\n"
            b"\xcc\x88\xcc\x80", // "̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xe0\xa4\x80", // "\nऀ"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xa4\x80", // "ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xa4\x80", // "\n̈ऀ"
        expected: &[
            b"\n",                   // "\n"
            b"\xcc\x88\xe0\xa4\x80", // "̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xe0\xa5\x8d", // "\n्"
        expected: &[
            b"\n",           // "\n"
            b"\xe0\xa5\x8d", // "्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xcc\x88\xe0\xa5\x8d", // "\n्̈"
        expected: &[
            b"\n",                   // "\n"
            b"\xcc\x88\xe0\xa5\x8d", // "्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xe2\x80\x8d", // "\n\u200d"
        expected: &[
            b"\n",           // "\n"
            b"\xe2\x80\x8d", // "\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\n\xcc\x88\xe2\x80\x8d", // "\n̈\u200d"
        expected: &[
            b"\n",                   // "\n"
            b"\xcc\x88\xe2\x80\x8d", // "̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\n\xcd\xb8", // "\n\u0378"
        expected: &[
            b"\n",       // "\n"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\n\xcc\x88\xcd\xb8", // "\n̈\u0378"
        expected: &[
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] SPACE (Other) ÷ [0.3]",
        input: b"\x01 ", // "\x01 "
        expected: &[
            b"\x01", // "\x01"
            b" ",    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\x01\xcc\x88 ", // "\x01̈ "
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x88", // "̈"
            b" ",        // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\x01\r", // "\x01\r"
        expected: &[
            b"\x01", // "\x01"
            b"\r",   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\x01\xcc\x88\r", // "\x01̈\r"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x88", // "̈"
            b"\r",       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\x01\n", // "\x01\n"
        expected: &[
            b"\x01", // "\x01"
            b"\n",   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\x01\xcc\x88\n", // "\x01̈\n"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x88", // "̈"
            b"\n",       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\x01\x01", // "\x01\x01"
        expected: &[
            b"\x01", // "\x01"
            b"\x01", // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\x01\xcc\x88\x01", // "\x01̈\x01"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x88", // "̈"
            b"\x01",     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\x01\xe2\x80\x8c", // "\x01\u200c"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe2\x80\x8c", // "\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe2\x80\x8c", // "\x01̈\u200c"
        expected: &[
            b"\x01",                 // "\x01"
            b"\xcc\x88\xe2\x80\x8c", // "̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\x01\xf0\x9f\x87\xa6", // "\x01🇦"
        expected: &[
            b"\x01",             // "\x01"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\x01\xcc\x88\xf0\x9f\x87\xa6", // "\x01̈🇦"
        expected: &[
            b"\x01",             // "\x01"
            b"\xcc\x88",         // "̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\x01\xd8\x80", // "\x01\u0600"
        expected: &[
            b"\x01",     // "\x01"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\x01\xcc\x88\xd8\x80", // "\x01̈\u0600"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x88", // "̈"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\x01\xe0\xa8\x83", // "\x01ਃ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xa8\x83", // "ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xa8\x83", // "\x01̈ਃ"
        expected: &[
            b"\x01",                 // "\x01"
            b"\xcc\x88\xe0\xa8\x83", // "̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\x01\xe1\x84\x80", // "\x01ᄀ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe1\x84\x80", // "\x01̈ᄀ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\x01\xe1\x85\xa0", // "\x01ᅠ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe1\x85\xa0", // "\x01̈ᅠ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\x01\xe1\x86\xa8", // "\x01ᆨ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe1\x86\xa8", // "\x01̈ᆨ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\x01\xea\xb0\x80", // "\x01가"
        expected: &[
            b"\x01",         // "\x01"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\x01\xcc\x88\xea\xb0\x80", // "\x01̈가"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\x01\xea\xb0\x81", // "\x01각"
        expected: &[
            b"\x01",         // "\x01"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\x01\xcc\x88\xea\xb0\x81", // "\x01̈각"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\x01\xe0\xa4\x83", // "\x01ः"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xa4\x83", // "ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xa4\x83", // "\x01̈ः"
        expected: &[
            b"\x01",                 // "\x01"
            b"\xcc\x88\xe0\xa4\x83", // "̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\x01\xe0\xa4\x84", // "\x01ऄ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xa4\x84", // "\x01̈ऄ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\x01\xe0\xb5\x8e", // "\x01ൎ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xb5\x8e", // "\x01̈ൎ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\x01\xe0\xa4\x95", // "\x01क"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xa4\x95", // "\x01̈क"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\x01\xe2\x8c\x9a", // "\x01⌚"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe2\x8c\x9a", // "\x01̈⌚"
        expected: &[
            b"\x01",         // "\x01"
            b"\xcc\x88",     // "̈"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xcc\x80", // "\x01̀"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x80", // "̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xcc\x88\xcc\x80", // "\x01̈̀"
        expected: &[
            b"\x01",             // "\x01"
            b"\xcc\x88\xcc\x80", // "̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xe0\xa4\x80", // "\x01ऀ"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xa4\x80", // "ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xa4\x80", // "\x01̈ऀ"
        expected: &[
            b"\x01",                 // "\x01"
            b"\xcc\x88\xe0\xa4\x80", // "̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xe0\xa5\x8d", // "\x01्"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe0\xa5\x8d", // "्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe0\xa5\x8d", // "\x01्̈"
        expected: &[
            b"\x01",                 // "\x01"
            b"\xcc\x88\xe0\xa5\x8d", // "्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xe2\x80\x8d", // "\x01\u200d"
        expected: &[
            b"\x01",         // "\x01"
            b"\xe2\x80\x8d", // "\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\x01\xcc\x88\xe2\x80\x8d", // "\x01̈\u200d"
        expected: &[
            b"\x01",                 // "\x01"
            b"\xcc\x88\xe2\x80\x8d", // "̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\x01\xcd\xb8", // "\x01\u0378"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <START OF HEADING> (Control) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\x01\xcc\x88\xcd\xb8", // "\x01̈\u0378"
        expected: &[
            b"\x01",     // "\x01"
            b"\xcc\x88", // "̈"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8c ", // "\u200c "
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88 ", // "\u200c̈ "
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe2\x80\x8c\r", // "\u200c\r"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\r", // "\u200c̈\r"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe2\x80\x8c\n", // "\u200c\n"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\n", // "\u200c̈\n"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe2\x80\x8c\x01", // "\u200c\x01"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\x01", // "\u200c̈\x01"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe2\x80\x8c", // "\u200c\u200c"
        expected: &[
            b"\xe2\x80\x8c\xe2\x80\x8c", // "\u200c\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe2\x80\x8c", // "\u200c̈\u200c"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xe2\x80\x8c", // "\u200c̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xf0\x9f\x87\xa6", // "\u200c🇦"
        expected: &[
            b"\xe2\x80\x8c",     // "\u200c"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xf0\x9f\x87\xa6", // "\u200c̈🇦"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xd8\x80", // "\u200c\u0600"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xd8\x80", // "\u200c̈\u0600"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xa8\x83", // "\u200cਃ"
        expected: &[
            b"\xe2\x80\x8c\xe0\xa8\x83", // "\u200cਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xa8\x83", // "\u200c̈ਃ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xe0\xa8\x83", // "\u200c̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe1\x84\x80", // "\u200cᄀ"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe1\x84\x80", // "\u200c̈ᄀ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe1\x85\xa0", // "\u200cᅠ"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe1\x85\xa0", // "\u200c̈ᅠ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe1\x86\xa8", // "\u200cᆨ"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe1\x86\xa8", // "\u200c̈ᆨ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xea\xb0\x80", // "\u200c가"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xea\xb0\x80", // "\u200c̈가"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xea\xb0\x81", // "\u200c각"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xea\xb0\x81", // "\u200c̈각"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xa4\x83", // "\u200cः"
        expected: &[
            b"\xe2\x80\x8c\xe0\xa4\x83", // "\u200cः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xa4\x83", // "\u200c̈ः"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xe0\xa4\x83", // "\u200c̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xa4\x84", // "\u200cऄ"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xa4\x84", // "\u200c̈ऄ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xb5\x8e", // "\u200cൎ"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xb5\x8e", // "\u200c̈ൎ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xa4\x95", // "\u200cक"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xa4\x95", // "\u200c̈क"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe2\x8c\x9a", // "\u200c⌚"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe2\x8c\x9a", // "\u200c̈⌚"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x80", // "\u200c̀"
        expected: &[
            b"\xe2\x80\x8c\xcc\x80", // "\u200c̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xcc\x80", // "\u200c̈̀"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xcc\x80", // "\u200c̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xa4\x80", // "\u200cऀ"
        expected: &[
            b"\xe2\x80\x8c\xe0\xa4\x80", // "\u200cऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xa4\x80", // "\u200c̈ऀ"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xe0\xa4\x80", // "\u200c̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe0\xa5\x8d", // "\u200c्"
        expected: &[
            b"\xe2\x80\x8c\xe0\xa5\x8d", // "\u200c्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe0\xa5\x8d", // "\u200c्̈"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xe0\xa5\x8d", // "\u200c्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xe2\x80\x8d", // "\u200c\u200d"
        expected: &[
            b"\xe2\x80\x8c\xe2\x80\x8d", // "\u200c\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xe2\x80\x8d", // "\u200c̈\u200d"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88\xe2\x80\x8d", // "\u200c̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcd\xb8", // "\u200c\u0378"
        expected: &[
            b"\xe2\x80\x8c", // "\u200c"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH NON-JOINER (Extend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8c\xcc\x88\xcd\xb8", // "\u200c̈\u0378"
        expected: &[
            b"\xe2\x80\x8c\xcc\x88", // "\u200c̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6 ", // "🇦 "
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b" ",                // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88 ", // "🇦̈ "
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b" ",                        // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\r", // "🇦\r"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\r",               // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\r", // "🇦̈\r"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\r",                       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\n", // "🇦\n"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\n",               // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\n", // "🇦̈\n"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\n",                       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\x01", // "🇦\x01"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\x01",             // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\x01", // "🇦̈\x01"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\x01",                     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe2\x80\x8c", // "🇦\u200c"
        expected: &[
            b"\xf0\x9f\x87\xa6\xe2\x80\x8c", // "🇦\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe2\x80\x8c", // "🇦̈\u200c"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xe2\x80\x8c", // "🇦̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [12.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa6", // "🇦🇦"
        expected: &[
            b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa6", // "🇦🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xf0\x9f\x87\xa6", // "🇦̈🇦"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xf0\x9f\x87\xa6",         // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xd8\x80", // "🇦\u0600"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xd8\x80",         // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xd8\x80", // "🇦̈\u0600"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xd8\x80",                 // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xa8\x83", // "🇦ਃ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xe0\xa8\x83", // "🇦ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa8\x83", // "🇦̈ਃ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa8\x83", // "🇦̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe1\x84\x80", // "🇦ᄀ"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe1\x84\x80",     // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe1\x84\x80", // "🇦̈ᄀ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe1\x84\x80",             // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe1\x85\xa0", // "🇦ᅠ"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe1\x85\xa0",     // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe1\x85\xa0", // "🇦̈ᅠ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe1\x85\xa0",             // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe1\x86\xa8", // "🇦ᆨ"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe1\x86\xa8",     // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe1\x86\xa8", // "🇦̈ᆨ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe1\x86\xa8",             // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xea\xb0\x80", // "🇦가"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xea\xb0\x80",     // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xea\xb0\x80", // "🇦̈가"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xea\xb0\x80",             // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xea\xb0\x81", // "🇦각"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xea\xb0\x81",     // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xea\xb0\x81", // "🇦̈각"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xea\xb0\x81",             // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xa4\x83", // "🇦ः"
        expected: &[
            b"\xf0\x9f\x87\xa6\xe0\xa4\x83", // "🇦ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa4\x83", // "🇦̈ः"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa4\x83", // "🇦̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xa4\x84", // "🇦ऄ"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe0\xa4\x84",     // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa4\x84", // "🇦̈ऄ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe0\xa4\x84",             // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xb5\x8e", // "🇦ൎ"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe0\xb5\x8e",     // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xb5\x8e", // "🇦̈ൎ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe0\xb5\x8e",             // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xa4\x95", // "🇦क"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe0\xa4\x95",     // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa4\x95", // "🇦̈क"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe0\xa4\x95",             // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe2\x8c\x9a", // "🇦⌚"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xe2\x8c\x9a",     // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe2\x8c\x9a", // "🇦̈⌚"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xe2\x8c\x9a",             // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x80", // "🇦̀"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x80", // "🇦̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xcc\x80", // "🇦̈̀"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xcc\x80", // "🇦̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xa4\x80", // "🇦ऀ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xe0\xa4\x80", // "🇦ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa4\x80", // "🇦̈ऀ"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa4\x80", // "🇦̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe0\xa5\x8d", // "🇦्"
        expected: &[
            b"\xf0\x9f\x87\xa6\xe0\xa5\x8d", // "🇦्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa5\x8d", // "🇦्̈"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xe0\xa5\x8d", // "🇦्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xe2\x80\x8d", // "🇦\u200d"
        expected: &[
            b"\xf0\x9f\x87\xa6\xe2\x80\x8d", // "🇦\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xe2\x80\x8d", // "🇦̈\u200d"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88\xe2\x80\x8d", // "🇦̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcd\xb8", // "🇦\u0378"
        expected: &[
            b"\xf0\x9f\x87\xa6", // "🇦"
            b"\xcd\xb8",         // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xcc\x88\xcd\xb8", // "🇦̈\u0378"
        expected: &[
            b"\xf0\x9f\x87\xa6\xcc\x88", // "🇦̈"
            b"\xcd\xb8",                 // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] SPACE (Other) ÷ [0.3]",
        input: b"\xd8\x80 ", // "\u0600 "
        expected: &[
            b"\xd8\x80 ", // "\u0600 "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88 ", // "\u0600̈ "
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b" ",                // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xd8\x80\r", // "\u0600\r"
        expected: &[
            b"\xd8\x80", // "\u0600"
            b"\r",       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\r", // "\u0600̈\r"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\r",               // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xd8\x80\n", // "\u0600\n"
        expected: &[
            b"\xd8\x80", // "\u0600"
            b"\n",       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\n", // "\u0600̈\n"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\n",               // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xd8\x80\x01", // "\u0600\x01"
        expected: &[
            b"\xd8\x80", // "\u0600"
            b"\x01",     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\x01", // "\u0600̈\x01"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\x01",             // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xd8\x80\xe2\x80\x8c", // "\u0600\u200c"
        expected: &[
            b"\xd8\x80\xe2\x80\x8c", // "\u0600\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe2\x80\x8c", // "\u0600̈\u200c"
        expected: &[
            b"\xd8\x80\xcc\x88\xe2\x80\x8c", // "\u0600̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xd8\x80\xf0\x9f\x87\xa6", // "\u0600🇦"
        expected: &[
            b"\xd8\x80\xf0\x9f\x87\xa6", // "\u0600🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xf0\x9f\x87\xa6", // "\u0600̈🇦"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xd8\x80\xd8\x80", // "\u0600\u0600"
        expected: &[
            b"\xd8\x80\xd8\x80", // "\u0600\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xd8\x80", // "\u0600̈\u0600"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xd8\x80",         // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xa8\x83", // "\u0600ਃ"
        expected: &[
            b"\xd8\x80\xe0\xa8\x83", // "\u0600ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xa8\x83", // "\u0600̈ਃ"
        expected: &[
            b"\xd8\x80\xcc\x88\xe0\xa8\x83", // "\u0600̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xd8\x80\xe1\x84\x80", // "\u0600ᄀ"
        expected: &[
            b"\xd8\x80\xe1\x84\x80", // "\u0600ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe1\x84\x80", // "\u0600̈ᄀ"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe1\x84\x80",     // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xd8\x80\xe1\x85\xa0", // "\u0600ᅠ"
        expected: &[
            b"\xd8\x80\xe1\x85\xa0", // "\u0600ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe1\x85\xa0", // "\u0600̈ᅠ"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe1\x85\xa0",     // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xd8\x80\xe1\x86\xa8", // "\u0600ᆨ"
        expected: &[
            b"\xd8\x80\xe1\x86\xa8", // "\u0600ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe1\x86\xa8", // "\u0600̈ᆨ"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe1\x86\xa8",     // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xd8\x80\xea\xb0\x80", // "\u0600가"
        expected: &[
            b"\xd8\x80\xea\xb0\x80", // "\u0600가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xea\xb0\x80", // "\u0600̈가"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xea\xb0\x80",     // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xd8\x80\xea\xb0\x81", // "\u0600각"
        expected: &[
            b"\xd8\x80\xea\xb0\x81", // "\u0600각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xea\xb0\x81", // "\u0600̈각"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xea\xb0\x81",     // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xa4\x83", // "\u0600ः"
        expected: &[
            b"\xd8\x80\xe0\xa4\x83", // "\u0600ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xa4\x83", // "\u0600̈ः"
        expected: &[
            b"\xd8\x80\xcc\x88\xe0\xa4\x83", // "\u0600̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xa4\x84", // "\u0600ऄ"
        expected: &[
            b"\xd8\x80\xe0\xa4\x84", // "\u0600ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xa4\x84", // "\u0600̈ऄ"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe0\xa4\x84",     // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xb5\x8e", // "\u0600ൎ"
        expected: &[
            b"\xd8\x80\xe0\xb5\x8e", // "\u0600ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xb5\x8e", // "\u0600̈ൎ"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe0\xb5\x8e",     // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xa4\x95", // "\u0600क"
        expected: &[
            b"\xd8\x80\xe0\xa4\x95", // "\u0600क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xa4\x95", // "\u0600̈क"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe0\xa4\x95",     // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xd8\x80\xe2\x8c\x9a", // "\u0600⌚"
        expected: &[
            b"\xd8\x80\xe2\x8c\x9a", // "\u0600⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe2\x8c\x9a", // "\u0600̈⌚"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xe2\x8c\x9a",     // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x80", // "\u0600̀"
        expected: &[
            b"\xd8\x80\xcc\x80", // "\u0600̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xcc\x80", // "\u0600̈̀"
        expected: &[
            b"\xd8\x80\xcc\x88\xcc\x80", // "\u0600̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xa4\x80", // "\u0600ऀ"
        expected: &[
            b"\xd8\x80\xe0\xa4\x80", // "\u0600ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xa4\x80", // "\u0600̈ऀ"
        expected: &[
            b"\xd8\x80\xcc\x88\xe0\xa4\x80", // "\u0600̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xe0\xa5\x8d", // "\u0600्"
        expected: &[
            b"\xd8\x80\xe0\xa5\x8d", // "\u0600्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe0\xa5\x8d", // "\u0600्̈"
        expected: &[
            b"\xd8\x80\xcc\x88\xe0\xa5\x8d", // "\u0600्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xe2\x80\x8d", // "\u0600\u200d"
        expected: &[
            b"\xd8\x80\xe2\x80\x8d", // "\u0600\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xe2\x80\x8d", // "\u0600̈\u200d"
        expected: &[
            b"\xd8\x80\xcc\x88\xe2\x80\x8d", // "\u0600̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.2] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xd8\x80\xcd\xb8", // "\u0600\u0378"
        expected: &[
            b"\xd8\x80\xcd\xb8", // "\u0600\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC NUMBER SIGN (Prepend) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xd8\x80\xcc\x88\xcd\xb8", // "\u0600̈\u0378"
        expected: &[
            b"\xd8\x80\xcc\x88", // "\u0600̈"
            b"\xcd\xb8",         // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa8\x83 ", // "ਃ "
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88 ", // "ਃ̈ "
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa8\x83\r", // "ਃ\r"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\r", // "ਃ̈\r"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa8\x83\n", // "ਃ\n"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\n", // "ਃ̈\n"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa8\x83\x01", // "ਃ\x01"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\x01", // "ਃ̈\x01"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe2\x80\x8c", // "ਃ\u200c"
        expected: &[
            b"\xe0\xa8\x83\xe2\x80\x8c", // "ਃ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe2\x80\x8c", // "ਃ̈\u200c"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xe2\x80\x8c", // "ਃ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xf0\x9f\x87\xa6", // "ਃ🇦"
        expected: &[
            b"\xe0\xa8\x83",     // "ਃ"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xf0\x9f\x87\xa6", // "ਃ̈🇦"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xd8\x80", // "ਃ\u0600"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xd8\x80", // "ਃ̈\u0600"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xa8\x83", // "ਃਃ"
        expected: &[
            b"\xe0\xa8\x83\xe0\xa8\x83", // "ਃਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xa8\x83", // "ਃ̈ਃ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xe0\xa8\x83", // "ਃ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe1\x84\x80", // "ਃᄀ"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe1\x84\x80", // "ਃ̈ᄀ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe1\x85\xa0", // "ਃᅠ"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe1\x85\xa0", // "ਃ̈ᅠ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe1\x86\xa8", // "ਃᆨ"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe1\x86\xa8", // "ਃ̈ᆨ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xea\xb0\x80", // "ਃ가"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xea\xb0\x80", // "ਃ̈가"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xea\xb0\x81", // "ਃ각"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xea\xb0\x81", // "ਃ̈각"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xa4\x83", // "ਃः"
        expected: &[
            b"\xe0\xa8\x83\xe0\xa4\x83", // "ਃः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xa4\x83", // "ਃ̈ः"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xe0\xa4\x83", // "ਃ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xa4\x84", // "ਃऄ"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xa4\x84", // "ਃ̈ऄ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xb5\x8e", // "ਃൎ"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xb5\x8e", // "ਃ̈ൎ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xa4\x95", // "ਃक"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xa4\x95", // "ਃ̈क"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe2\x8c\x9a", // "ਃ⌚"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe2\x8c\x9a", // "ਃ̈⌚"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x80", // "ਃ̀"
        expected: &[
            b"\xe0\xa8\x83\xcc\x80", // "ਃ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xcc\x80", // "ਃ̈̀"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xcc\x80", // "ਃ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xa4\x80", // "ਃऀ"
        expected: &[
            b"\xe0\xa8\x83\xe0\xa4\x80", // "ਃऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xa4\x80", // "ਃ̈ऀ"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xe0\xa4\x80", // "ਃ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe0\xa5\x8d", // "ਃ्"
        expected: &[
            b"\xe0\xa8\x83\xe0\xa5\x8d", // "ਃ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe0\xa5\x8d", // "ਃ्̈"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xe0\xa5\x8d", // "ਃ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xe2\x80\x8d", // "ਃ\u200d"
        expected: &[
            b"\xe0\xa8\x83\xe2\x80\x8d", // "ਃ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xe2\x80\x8d", // "ਃ̈\u200d"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88\xe2\x80\x8d", // "ਃ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcd\xb8", // "ਃ\u0378"
        expected: &[
            b"\xe0\xa8\x83", // "ਃ"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] GURMUKHI SIGN VISARGA (SpacingMark) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa8\x83\xcc\x88\xcd\xb8", // "ਃ̈\u0378"
        expected: &[
            b"\xe0\xa8\x83\xcc\x88", // "ਃ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe1\x84\x80 ", // "ᄀ "
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88 ", // "ᄀ̈ "
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe1\x84\x80\r", // "ᄀ\r"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\r", // "ᄀ̈\r"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe1\x84\x80\n", // "ᄀ\n"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\n", // "ᄀ̈\n"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe1\x84\x80\x01", // "ᄀ\x01"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\x01", // "ᄀ̈\x01"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe2\x80\x8c", // "ᄀ\u200c"
        expected: &[
            b"\xe1\x84\x80\xe2\x80\x8c", // "ᄀ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe2\x80\x8c", // "ᄀ̈\u200c"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xe2\x80\x8c", // "ᄀ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe1\x84\x80\xf0\x9f\x87\xa6", // "ᄀ🇦"
        expected: &[
            b"\xe1\x84\x80",     // "ᄀ"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xf0\x9f\x87\xa6", // "ᄀ̈🇦"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe1\x84\x80\xd8\x80", // "ᄀ\u0600"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xd8\x80", // "ᄀ̈\u0600"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xa8\x83", // "ᄀਃ"
        expected: &[
            b"\xe1\x84\x80\xe0\xa8\x83", // "ᄀਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xa8\x83", // "ᄀ̈ਃ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xe0\xa8\x83", // "ᄀ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [6.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe1\x84\x80", // "ᄀᄀ"
        expected: &[
            b"\xe1\x84\x80\xe1\x84\x80", // "ᄀᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe1\x84\x80", // "ᄀ̈ᄀ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [6.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe1\x85\xa0", // "ᄀᅠ"
        expected: &[
            b"\xe1\x84\x80\xe1\x85\xa0", // "ᄀᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe1\x85\xa0", // "ᄀ̈ᅠ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe1\x86\xa8", // "ᄀᆨ"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe1\x86\xa8", // "ᄀ̈ᆨ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [6.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe1\x84\x80\xea\xb0\x80", // "ᄀ가"
        expected: &[
            b"\xe1\x84\x80\xea\xb0\x80", // "ᄀ가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xea\xb0\x80", // "ᄀ̈가"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [6.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe1\x84\x80\xea\xb0\x81", // "ᄀ각"
        expected: &[
            b"\xe1\x84\x80\xea\xb0\x81", // "ᄀ각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xea\xb0\x81", // "ᄀ̈각"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xa4\x83", // "ᄀः"
        expected: &[
            b"\xe1\x84\x80\xe0\xa4\x83", // "ᄀः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xa4\x83", // "ᄀ̈ः"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xe0\xa4\x83", // "ᄀ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xa4\x84", // "ᄀऄ"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xa4\x84", // "ᄀ̈ऄ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xb5\x8e", // "ᄀൎ"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xb5\x8e", // "ᄀ̈ൎ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xa4\x95", // "ᄀक"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xa4\x95", // "ᄀ̈क"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe2\x8c\x9a", // "ᄀ⌚"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe2\x8c\x9a", // "ᄀ̈⌚"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x80", // "ᄀ̀"
        expected: &[
            b"\xe1\x84\x80\xcc\x80", // "ᄀ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xcc\x80", // "ᄀ̈̀"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xcc\x80", // "ᄀ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xa4\x80", // "ᄀऀ"
        expected: &[
            b"\xe1\x84\x80\xe0\xa4\x80", // "ᄀऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xa4\x80", // "ᄀ̈ऀ"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xe0\xa4\x80", // "ᄀ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe0\xa5\x8d", // "ᄀ्"
        expected: &[
            b"\xe1\x84\x80\xe0\xa5\x8d", // "ᄀ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe0\xa5\x8d", // "ᄀ्̈"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xe0\xa5\x8d", // "ᄀ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe2\x80\x8d", // "ᄀ\u200d"
        expected: &[
            b"\xe1\x84\x80\xe2\x80\x8d", // "ᄀ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xe2\x80\x8d", // "ᄀ̈\u200d"
        expected: &[
            b"\xe1\x84\x80\xcc\x88\xe2\x80\x8d", // "ᄀ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcd\xb8", // "ᄀ\u0378"
        expected: &[
            b"\xe1\x84\x80", // "ᄀ"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe1\x84\x80\xcc\x88\xcd\xb8", // "ᄀ̈\u0378"
        expected: &[
            b"\xe1\x84\x80\xcc\x88", // "ᄀ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe1\x85\xa0 ", // "ᅠ "
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88 ", // "ᅠ̈ "
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe1\x85\xa0\r", // "ᅠ\r"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\r", // "ᅠ̈\r"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe1\x85\xa0\n", // "ᅠ\n"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\n", // "ᅠ̈\n"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe1\x85\xa0\x01", // "ᅠ\x01"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\x01", // "ᅠ̈\x01"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe2\x80\x8c", // "ᅠ\u200c"
        expected: &[
            b"\xe1\x85\xa0\xe2\x80\x8c", // "ᅠ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe2\x80\x8c", // "ᅠ̈\u200c"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xe2\x80\x8c", // "ᅠ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xf0\x9f\x87\xa6", // "ᅠ🇦"
        expected: &[
            b"\xe1\x85\xa0",     // "ᅠ"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xf0\x9f\x87\xa6", // "ᅠ̈🇦"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xd8\x80", // "ᅠ\u0600"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xd8\x80", // "ᅠ̈\u0600"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xa8\x83", // "ᅠਃ"
        expected: &[
            b"\xe1\x85\xa0\xe0\xa8\x83", // "ᅠਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xa8\x83", // "ᅠ̈ਃ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xe0\xa8\x83", // "ᅠ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe1\x84\x80", // "ᅠᄀ"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe1\x84\x80", // "ᅠ̈ᄀ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [7.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe1\x85\xa0", // "ᅠᅠ"
        expected: &[
            b"\xe1\x85\xa0\xe1\x85\xa0", // "ᅠᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe1\x85\xa0", // "ᅠ̈ᅠ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [7.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe1\x86\xa8", // "ᅠᆨ"
        expected: &[
            b"\xe1\x85\xa0\xe1\x86\xa8", // "ᅠᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe1\x86\xa8", // "ᅠ̈ᆨ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xea\xb0\x80", // "ᅠ가"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xea\xb0\x80", // "ᅠ̈가"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xea\xb0\x81", // "ᅠ각"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xea\xb0\x81", // "ᅠ̈각"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xa4\x83", // "ᅠः"
        expected: &[
            b"\xe1\x85\xa0\xe0\xa4\x83", // "ᅠः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xa4\x83", // "ᅠ̈ः"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xe0\xa4\x83", // "ᅠ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xa4\x84", // "ᅠऄ"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xa4\x84", // "ᅠ̈ऄ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xb5\x8e", // "ᅠൎ"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xb5\x8e", // "ᅠ̈ൎ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xa4\x95", // "ᅠक"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xa4\x95", // "ᅠ̈क"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe2\x8c\x9a", // "ᅠ⌚"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe2\x8c\x9a", // "ᅠ̈⌚"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x80", // "ᅠ̀"
        expected: &[
            b"\xe1\x85\xa0\xcc\x80", // "ᅠ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xcc\x80", // "ᅠ̈̀"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xcc\x80", // "ᅠ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xa4\x80", // "ᅠऀ"
        expected: &[
            b"\xe1\x85\xa0\xe0\xa4\x80", // "ᅠऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xa4\x80", // "ᅠ̈ऀ"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xe0\xa4\x80", // "ᅠ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe0\xa5\x8d", // "ᅠ्"
        expected: &[
            b"\xe1\x85\xa0\xe0\xa5\x8d", // "ᅠ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe0\xa5\x8d", // "ᅠ्̈"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xe0\xa5\x8d", // "ᅠ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xe2\x80\x8d", // "ᅠ\u200d"
        expected: &[
            b"\xe1\x85\xa0\xe2\x80\x8d", // "ᅠ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xe2\x80\x8d", // "ᅠ̈\u200d"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88\xe2\x80\x8d", // "ᅠ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcd\xb8", // "ᅠ\u0378"
        expected: &[
            b"\xe1\x85\xa0", // "ᅠ"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JUNGSEONG FILLER (V) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe1\x85\xa0\xcc\x88\xcd\xb8", // "ᅠ̈\u0378"
        expected: &[
            b"\xe1\x85\xa0\xcc\x88", // "ᅠ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe1\x86\xa8 ", // "ᆨ "
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88 ", // "ᆨ̈ "
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe1\x86\xa8\r", // "ᆨ\r"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\r", // "ᆨ̈\r"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe1\x86\xa8\n", // "ᆨ\n"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\n", // "ᆨ̈\n"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe1\x86\xa8\x01", // "ᆨ\x01"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\x01", // "ᆨ̈\x01"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe2\x80\x8c", // "ᆨ\u200c"
        expected: &[
            b"\xe1\x86\xa8\xe2\x80\x8c", // "ᆨ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe2\x80\x8c", // "ᆨ̈\u200c"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xe2\x80\x8c", // "ᆨ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xf0\x9f\x87\xa6", // "ᆨ🇦"
        expected: &[
            b"\xe1\x86\xa8",     // "ᆨ"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xf0\x9f\x87\xa6", // "ᆨ̈🇦"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xd8\x80", // "ᆨ\u0600"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xd8\x80", // "ᆨ̈\u0600"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xa8\x83", // "ᆨਃ"
        expected: &[
            b"\xe1\x86\xa8\xe0\xa8\x83", // "ᆨਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xa8\x83", // "ᆨ̈ਃ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xe0\xa8\x83", // "ᆨ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe1\x84\x80", // "ᆨᄀ"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe1\x84\x80", // "ᆨ̈ᄀ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe1\x85\xa0", // "ᆨᅠ"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe1\x85\xa0", // "ᆨ̈ᅠ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [8.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe1\x86\xa8", // "ᆨᆨ"
        expected: &[
            b"\xe1\x86\xa8\xe1\x86\xa8", // "ᆨᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe1\x86\xa8", // "ᆨ̈ᆨ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xea\xb0\x80", // "ᆨ가"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xea\xb0\x80", // "ᆨ̈가"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xea\xb0\x81", // "ᆨ각"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xea\xb0\x81", // "ᆨ̈각"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xa4\x83", // "ᆨः"
        expected: &[
            b"\xe1\x86\xa8\xe0\xa4\x83", // "ᆨः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xa4\x83", // "ᆨ̈ः"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xe0\xa4\x83", // "ᆨ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xa4\x84", // "ᆨऄ"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xa4\x84", // "ᆨ̈ऄ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xb5\x8e", // "ᆨൎ"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xb5\x8e", // "ᆨ̈ൎ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xa4\x95", // "ᆨक"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xa4\x95", // "ᆨ̈क"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe2\x8c\x9a", // "ᆨ⌚"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe2\x8c\x9a", // "ᆨ̈⌚"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x80", // "ᆨ̀"
        expected: &[
            b"\xe1\x86\xa8\xcc\x80", // "ᆨ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xcc\x80", // "ᆨ̈̀"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xcc\x80", // "ᆨ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xa4\x80", // "ᆨऀ"
        expected: &[
            b"\xe1\x86\xa8\xe0\xa4\x80", // "ᆨऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xa4\x80", // "ᆨ̈ऀ"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xe0\xa4\x80", // "ᆨ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe0\xa5\x8d", // "ᆨ्"
        expected: &[
            b"\xe1\x86\xa8\xe0\xa5\x8d", // "ᆨ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe0\xa5\x8d", // "ᆨ्̈"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xe0\xa5\x8d", // "ᆨ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xe2\x80\x8d", // "ᆨ\u200d"
        expected: &[
            b"\xe1\x86\xa8\xe2\x80\x8d", // "ᆨ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xe2\x80\x8d", // "ᆨ̈\u200d"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88\xe2\x80\x8d", // "ᆨ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcd\xb8", // "ᆨ\u0378"
        expected: &[
            b"\xe1\x86\xa8", // "ᆨ"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL JONGSEONG KIYEOK (T) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe1\x86\xa8\xcc\x88\xcd\xb8", // "ᆨ̈\u0378"
        expected: &[
            b"\xe1\x86\xa8\xcc\x88", // "ᆨ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xea\xb0\x80 ", // "가 "
        expected: &[
            b"\xea\xb0\x80", // "가"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88 ", // "가̈ "
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xea\xb0\x80\r", // "가\r"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\r", // "가̈\r"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xea\xb0\x80\n", // "가\n"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\n", // "가̈\n"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xea\xb0\x80\x01", // "가\x01"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\x01", // "가̈\x01"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe2\x80\x8c", // "가\u200c"
        expected: &[
            b"\xea\xb0\x80\xe2\x80\x8c", // "가\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe2\x80\x8c", // "가̈\u200c"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xe2\x80\x8c", // "가̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xea\xb0\x80\xf0\x9f\x87\xa6", // "가🇦"
        expected: &[
            b"\xea\xb0\x80",     // "가"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xf0\x9f\x87\xa6", // "가̈🇦"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xea\xb0\x80\xd8\x80", // "가\u0600"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xd8\x80", // "가̈\u0600"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xa8\x83", // "가ਃ"
        expected: &[
            b"\xea\xb0\x80\xe0\xa8\x83", // "가ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xa8\x83", // "가̈ਃ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xe0\xa8\x83", // "가̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe1\x84\x80", // "가ᄀ"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe1\x84\x80", // "가̈ᄀ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [7.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe1\x85\xa0", // "가ᅠ"
        expected: &[
            b"\xea\xb0\x80\xe1\x85\xa0", // "가ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe1\x85\xa0", // "가̈ᅠ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [7.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe1\x86\xa8", // "각"
        expected: &[
            b"\xea\xb0\x80\xe1\x86\xa8", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe1\x86\xa8", // "가̈ᆨ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xea\xb0\x80\xea\xb0\x80", // "가가"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xea\xb0\x80", // "가̈가"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xea\xb0\x80\xea\xb0\x81", // "가각"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xea\xb0\x81", // "가̈각"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xa4\x83", // "가ः"
        expected: &[
            b"\xea\xb0\x80\xe0\xa4\x83", // "가ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xa4\x83", // "가̈ः"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xe0\xa4\x83", // "가̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xa4\x84", // "가ऄ"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xa4\x84", // "가̈ऄ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xb5\x8e", // "가ൎ"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xb5\x8e", // "가̈ൎ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xa4\x95", // "가क"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xa4\x95", // "가̈क"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe2\x8c\x9a", // "가⌚"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe2\x8c\x9a", // "가̈⌚"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x80", // "가̀"
        expected: &[
            b"\xea\xb0\x80\xcc\x80", // "가̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xcc\x80", // "가̈̀"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xcc\x80", // "가̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xa4\x80", // "가ऀ"
        expected: &[
            b"\xea\xb0\x80\xe0\xa4\x80", // "가ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xa4\x80", // "가̈ऀ"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xe0\xa4\x80", // "가̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe0\xa5\x8d", // "가्"
        expected: &[
            b"\xea\xb0\x80\xe0\xa5\x8d", // "가्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe0\xa5\x8d", // "가्̈"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xe0\xa5\x8d", // "가्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe2\x80\x8d", // "가\u200d"
        expected: &[
            b"\xea\xb0\x80\xe2\x80\x8d", // "가\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xe2\x80\x8d", // "가̈\u200d"
        expected: &[
            b"\xea\xb0\x80\xcc\x88\xe2\x80\x8d", // "가̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcd\xb8", // "가\u0378"
        expected: &[
            b"\xea\xb0\x80", // "가"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xea\xb0\x80\xcc\x88\xcd\xb8", // "가̈\u0378"
        expected: &[
            b"\xea\xb0\x80\xcc\x88", // "가̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xea\xb0\x81 ", // "각 "
        expected: &[
            b"\xea\xb0\x81", // "각"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88 ", // "각̈ "
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xea\xb0\x81\r", // "각\r"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\r", // "각̈\r"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xea\xb0\x81\n", // "각\n"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\n", // "각̈\n"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xea\xb0\x81\x01", // "각\x01"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\x01", // "각̈\x01"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe2\x80\x8c", // "각\u200c"
        expected: &[
            b"\xea\xb0\x81\xe2\x80\x8c", // "각\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe2\x80\x8c", // "각̈\u200c"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xe2\x80\x8c", // "각̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xea\xb0\x81\xf0\x9f\x87\xa6", // "각🇦"
        expected: &[
            b"\xea\xb0\x81",     // "각"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xf0\x9f\x87\xa6", // "각̈🇦"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xea\xb0\x81\xd8\x80", // "각\u0600"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xd8\x80", // "각̈\u0600"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xa8\x83", // "각ਃ"
        expected: &[
            b"\xea\xb0\x81\xe0\xa8\x83", // "각ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xa8\x83", // "각̈ਃ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xe0\xa8\x83", // "각̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe1\x84\x80", // "각ᄀ"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe1\x84\x80", // "각̈ᄀ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe1\x85\xa0", // "각ᅠ"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe1\x85\xa0", // "각̈ᅠ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [8.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe1\x86\xa8", // "각ᆨ"
        expected: &[
            b"\xea\xb0\x81\xe1\x86\xa8", // "각ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe1\x86\xa8", // "각̈ᆨ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xea\xb0\x81\xea\xb0\x80", // "각가"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xea\xb0\x80", // "각̈가"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xea\xb0\x81\xea\xb0\x81", // "각각"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xea\xb0\x81", // "각̈각"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xa4\x83", // "각ः"
        expected: &[
            b"\xea\xb0\x81\xe0\xa4\x83", // "각ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xa4\x83", // "각̈ः"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xe0\xa4\x83", // "각̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xa4\x84", // "각ऄ"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xa4\x84", // "각̈ऄ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xb5\x8e", // "각ൎ"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xb5\x8e", // "각̈ൎ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xa4\x95", // "각क"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xa4\x95", // "각̈क"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe2\x8c\x9a", // "각⌚"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe2\x8c\x9a", // "각̈⌚"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x80", // "각̀"
        expected: &[
            b"\xea\xb0\x81\xcc\x80", // "각̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xcc\x80", // "각̈̀"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xcc\x80", // "각̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xa4\x80", // "각ऀ"
        expected: &[
            b"\xea\xb0\x81\xe0\xa4\x80", // "각ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xa4\x80", // "각̈ऀ"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xe0\xa4\x80", // "각̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe0\xa5\x8d", // "각्"
        expected: &[
            b"\xea\xb0\x81\xe0\xa5\x8d", // "각्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe0\xa5\x8d", // "각्̈"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xe0\xa5\x8d", // "각्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe2\x80\x8d", // "각\u200d"
        expected: &[
            b"\xea\xb0\x81\xe2\x80\x8d", // "각\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xe2\x80\x8d", // "각̈\u200d"
        expected: &[
            b"\xea\xb0\x81\xcc\x88\xe2\x80\x8d", // "각̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcd\xb8", // "각\u0378"
        expected: &[
            b"\xea\xb0\x81", // "각"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xea\xb0\x81\xcc\x88\xcd\xb8", // "각̈\u0378"
        expected: &[
            b"\xea\xb0\x81\xcc\x88", // "각̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x83 ", // "ः "
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88 ", // "ः̈ "
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x83\r", // "ः\r"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\r", // "ः̈\r"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x83\n", // "ः\n"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\n", // "ः̈\n"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x83\x01", // "ः\x01"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\x01", // "ः̈\x01"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe2\x80\x8c", // "ः\u200c"
        expected: &[
            b"\xe0\xa4\x83\xe2\x80\x8c", // "ः\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe2\x80\x8c", // "ः̈\u200c"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xe2\x80\x8c", // "ः̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xf0\x9f\x87\xa6", // "ः🇦"
        expected: &[
            b"\xe0\xa4\x83",     // "ः"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xf0\x9f\x87\xa6", // "ः̈🇦"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xd8\x80", // "ः\u0600"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xd8\x80", // "ः̈\u0600"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xa8\x83", // "ःਃ"
        expected: &[
            b"\xe0\xa4\x83\xe0\xa8\x83", // "ःਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xa8\x83", // "ः̈ਃ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xe0\xa8\x83", // "ः̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe1\x84\x80", // "ःᄀ"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe1\x84\x80", // "ः̈ᄀ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe1\x85\xa0", // "ःᅠ"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe1\x85\xa0", // "ः̈ᅠ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe1\x86\xa8", // "ःᆨ"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe1\x86\xa8", // "ः̈ᆨ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xea\xb0\x80", // "ः가"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xea\xb0\x80", // "ः̈가"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xea\xb0\x81", // "ः각"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xea\xb0\x81", // "ः̈각"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xa4\x83", // "ःः"
        expected: &[
            b"\xe0\xa4\x83\xe0\xa4\x83", // "ःः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xa4\x83", // "ः̈ः"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xe0\xa4\x83", // "ः̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xa4\x84", // "ःऄ"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xa4\x84", // "ः̈ऄ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xb5\x8e", // "ःൎ"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xb5\x8e", // "ः̈ൎ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xa4\x95", // "ःक"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xa4\x95", // "ः̈क"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe2\x8c\x9a", // "ः⌚"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe2\x8c\x9a", // "ः̈⌚"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x80", // "ः̀"
        expected: &[
            b"\xe0\xa4\x83\xcc\x80", // "ः̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xcc\x80", // "ः̈̀"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xcc\x80", // "ः̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xa4\x80", // "ःऀ"
        expected: &[
            b"\xe0\xa4\x83\xe0\xa4\x80", // "ःऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xa4\x80", // "ः̈ऀ"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xe0\xa4\x80", // "ः̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe0\xa5\x8d", // "ः्"
        expected: &[
            b"\xe0\xa4\x83\xe0\xa5\x8d", // "ः्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe0\xa5\x8d", // "ः्̈"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xe0\xa5\x8d", // "ः्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xe2\x80\x8d", // "ः\u200d"
        expected: &[
            b"\xe0\xa4\x83\xe2\x80\x8d", // "ः\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xe2\x80\x8d", // "ः̈\u200d"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88\xe2\x80\x8d", // "ः̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcd\xb8", // "ः\u0378"
        expected: &[
            b"\xe0\xa4\x83", // "ः"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x83\xcc\x88\xcd\xb8", // "ः̈\u0378"
        expected: &[
            b"\xe0\xa4\x83\xcc\x88", // "ः̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x84 ", // "ऄ "
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88 ", // "ऄ̈ "
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x84\r", // "ऄ\r"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\r", // "ऄ̈\r"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x84\n", // "ऄ\n"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\n", // "ऄ̈\n"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x84\x01", // "ऄ\x01"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\x01", // "ऄ̈\x01"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe2\x80\x8c", // "ऄ\u200c"
        expected: &[
            b"\xe0\xa4\x84\xe2\x80\x8c", // "ऄ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe2\x80\x8c", // "ऄ̈\u200c"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xe2\x80\x8c", // "ऄ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xf0\x9f\x87\xa6", // "ऄ🇦"
        expected: &[
            b"\xe0\xa4\x84",     // "ऄ"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xf0\x9f\x87\xa6", // "ऄ̈🇦"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xd8\x80", // "ऄ\u0600"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xd8\x80", // "ऄ̈\u0600"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xa8\x83", // "ऄਃ"
        expected: &[
            b"\xe0\xa4\x84\xe0\xa8\x83", // "ऄਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xa8\x83", // "ऄ̈ਃ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xe0\xa8\x83", // "ऄ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe1\x84\x80", // "ऄᄀ"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe1\x84\x80", // "ऄ̈ᄀ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe1\x85\xa0", // "ऄᅠ"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe1\x85\xa0", // "ऄ̈ᅠ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe1\x86\xa8", // "ऄᆨ"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe1\x86\xa8", // "ऄ̈ᆨ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xea\xb0\x80", // "ऄ가"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xea\xb0\x80", // "ऄ̈가"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xea\xb0\x81", // "ऄ각"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xea\xb0\x81", // "ऄ̈각"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xa4\x83", // "ऄः"
        expected: &[
            b"\xe0\xa4\x84\xe0\xa4\x83", // "ऄः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xa4\x83", // "ऄ̈ः"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xe0\xa4\x83", // "ऄ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xa4\x84", // "ऄऄ"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xa4\x84", // "ऄ̈ऄ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xb5\x8e", // "ऄൎ"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xb5\x8e", // "ऄ̈ൎ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xa4\x95", // "ऄक"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xa4\x95", // "ऄ̈क"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe2\x8c\x9a", // "ऄ⌚"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe2\x8c\x9a", // "ऄ̈⌚"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x80", // "ऄ̀"
        expected: &[
            b"\xe0\xa4\x84\xcc\x80", // "ऄ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xcc\x80", // "ऄ̈̀"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xcc\x80", // "ऄ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xa4\x80", // "ऄऀ"
        expected: &[
            b"\xe0\xa4\x84\xe0\xa4\x80", // "ऄऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xa4\x80", // "ऄ̈ऀ"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xe0\xa4\x80", // "ऄ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe0\xa5\x8d", // "ऄ्"
        expected: &[
            b"\xe0\xa4\x84\xe0\xa5\x8d", // "ऄ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe0\xa5\x8d", // "ऄ्̈"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xe0\xa5\x8d", // "ऄ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xe2\x80\x8d", // "ऄ\u200d"
        expected: &[
            b"\xe0\xa4\x84\xe2\x80\x8d", // "ऄ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xe2\x80\x8d", // "ऄ̈\u200d"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88\xe2\x80\x8d", // "ऄ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcd\xb8", // "ऄ\u0378"
        expected: &[
            b"\xe0\xa4\x84", // "ऄ"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x84\xcc\x88\xcd\xb8", // "ऄ̈\u0378"
        expected: &[
            b"\xe0\xa4\x84\xcc\x88", // "ऄ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xb5\x8e ", // "ൎ "
        expected: &[
            b"\xe0\xb5\x8e ", // "ൎ "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88 ", // "ൎ̈ "
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\r", // "ൎ\r"
        expected: &[
            b"\xe0\xb5\x8e", // "ൎ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\r", // "ൎ̈\r"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\n", // "ൎ\n"
        expected: &[
            b"\xe0\xb5\x8e", // "ൎ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\n", // "ൎ̈\n"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\x01", // "ൎ\x01"
        expected: &[
            b"\xe0\xb5\x8e", // "ൎ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\x01", // "ൎ̈\x01"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe2\x80\x8c", // "ൎ\u200c"
        expected: &[
            b"\xe0\xb5\x8e\xe2\x80\x8c", // "ൎ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe2\x80\x8c", // "ൎ̈\u200c"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xe2\x80\x8c", // "ൎ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xf0\x9f\x87\xa6", // "ൎ🇦"
        expected: &[
            b"\xe0\xb5\x8e\xf0\x9f\x87\xa6", // "ൎ🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xf0\x9f\x87\xa6", // "ൎ̈🇦"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xd8\x80", // "ൎ\u0600"
        expected: &[
            b"\xe0\xb5\x8e\xd8\x80", // "ൎ\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xd8\x80", // "ൎ̈\u0600"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xa8\x83", // "ൎਃ"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xa8\x83", // "ൎਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xa8\x83", // "ൎ̈ਃ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xe0\xa8\x83", // "ൎ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe1\x84\x80", // "ൎᄀ"
        expected: &[
            b"\xe0\xb5\x8e\xe1\x84\x80", // "ൎᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe1\x84\x80", // "ൎ̈ᄀ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe1\x85\xa0", // "ൎᅠ"
        expected: &[
            b"\xe0\xb5\x8e\xe1\x85\xa0", // "ൎᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe1\x85\xa0", // "ൎ̈ᅠ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe1\x86\xa8", // "ൎᆨ"
        expected: &[
            b"\xe0\xb5\x8e\xe1\x86\xa8", // "ൎᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe1\x86\xa8", // "ൎ̈ᆨ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xea\xb0\x80", // "ൎ가"
        expected: &[
            b"\xe0\xb5\x8e\xea\xb0\x80", // "ൎ가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xea\xb0\x80", // "ൎ̈가"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xea\xb0\x81", // "ൎ각"
        expected: &[
            b"\xe0\xb5\x8e\xea\xb0\x81", // "ൎ각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xea\xb0\x81", // "ൎ̈각"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xa4\x83", // "ൎः"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xa4\x83", // "ൎः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xa4\x83", // "ൎ̈ः"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xe0\xa4\x83", // "ൎ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xa4\x84", // "ൎऄ"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xa4\x84", // "ൎऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xa4\x84", // "ൎ̈ऄ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xb5\x8e", // "ൎൎ"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xb5\x8e", // "ൎൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xb5\x8e", // "ൎ̈ൎ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xa4\x95", // "ൎक"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xa4\x95", // "ൎक"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xa4\x95", // "ൎ̈क"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe2\x8c\x9a", // "ൎ⌚"
        expected: &[
            b"\xe0\xb5\x8e\xe2\x8c\x9a", // "ൎ⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe2\x8c\x9a", // "ൎ̈⌚"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x80", // "ൎ̀"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x80", // "ൎ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xcc\x80", // "ൎ̈̀"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xcc\x80", // "ൎ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xa4\x80", // "ൎऀ"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xa4\x80", // "ൎऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xa4\x80", // "ൎ̈ऀ"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xe0\xa4\x80", // "ൎ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe0\xa5\x8d", // "ൎ्"
        expected: &[
            b"\xe0\xb5\x8e\xe0\xa5\x8d", // "ൎ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe0\xa5\x8d", // "ൎ्̈"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xe0\xa5\x8d", // "ൎ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xe2\x80\x8d", // "ൎ\u200d"
        expected: &[
            b"\xe0\xb5\x8e\xe2\x80\x8d", // "ൎ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xe2\x80\x8d", // "ൎ̈\u200d"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88\xe2\x80\x8d", // "ൎ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.2] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcd\xb8", // "ൎ\u0378"
        expected: &[
            b"\xe0\xb5\x8e\xcd\xb8", // "ൎ\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xb5\x8e\xcc\x88\xcd\xb8", // "ൎ̈\u0378"
        expected: &[
            b"\xe0\xb5\x8e\xcc\x88", // "ൎ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x95 ", // "क "
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88 ", // "क̈ "
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x95\r", // "क\r"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\r", // "क̈\r"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x95\n", // "क\n"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\n", // "क̈\n"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x95\x01", // "क\x01"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\x01", // "क̈\x01"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe2\x80\x8c", // "क\u200c"
        expected: &[
            b"\xe0\xa4\x95\xe2\x80\x8c", // "क\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe2\x80\x8c", // "क̈\u200c"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xe2\x80\x8c", // "क̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xf0\x9f\x87\xa6", // "क🇦"
        expected: &[
            b"\xe0\xa4\x95",     // "क"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xf0\x9f\x87\xa6", // "क̈🇦"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xd8\x80", // "क\u0600"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xd8\x80", // "क̈\u0600"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa8\x83", // "कਃ"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa8\x83", // "कਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xa8\x83", // "क̈ਃ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xe0\xa8\x83", // "क̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe1\x84\x80", // "कᄀ"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe1\x84\x80", // "क̈ᄀ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe1\x85\xa0", // "कᅠ"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe1\x85\xa0", // "क̈ᅠ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe1\x86\xa8", // "कᆨ"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe1\x86\xa8", // "क̈ᆨ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xea\xb0\x80", // "क가"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xea\xb0\x80", // "क̈가"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xea\xb0\x81", // "क각"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xea\xb0\x81", // "क̈각"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\x83", // "कः"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa4\x83", // "कः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xa4\x83", // "क̈ः"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xe0\xa4\x83", // "क̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\x84", // "कऄ"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xa4\x84", // "क̈ऄ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xb5\x8e", // "कൎ"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xb5\x8e", // "क̈ൎ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\x95", // "कक"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xa4\x95", // "क̈क"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe2\x8c\x9a", // "क⌚"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe2\x8c\x9a", // "क̈⌚"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x80", // "क̀"
        expected: &[
            b"\xe0\xa4\x95\xcc\x80", // "क̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xcc\x80", // "क̈̀"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xcc\x80", // "क̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\x80", // "कऀ"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa4\x80", // "कऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xa4\x80", // "क̈ऀ"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xe0\xa4\x80", // "क̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8d", // "क्"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d", // "क्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe0\xa5\x8d", // "क्̈"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xe0\xa5\x8d", // "क्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe2\x80\x8d", // "क\u200d"
        expected: &[
            b"\xe0\xa4\x95\xe2\x80\x8d", // "क\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xe2\x80\x8d", // "क̈\u200d"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88\xe2\x80\x8d", // "क̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcd\xb8", // "क\u0378"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xcc\x88\xcd\xb8", // "क̈\u0378"
        expected: &[
            b"\xe0\xa4\x95\xcc\x88", // "क̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe2\x8c\x9a ", // "⌚ "
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88 ", // "⌚̈ "
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\r", // "⌚\r"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\r", // "⌚̈\r"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\n", // "⌚\n"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\n", // "⌚̈\n"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\x01", // "⌚\x01"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\x01", // "⌚̈\x01"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe2\x80\x8c", // "⌚\u200c"
        expected: &[
            b"\xe2\x8c\x9a\xe2\x80\x8c", // "⌚\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe2\x80\x8c", // "⌚̈\u200c"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xe2\x80\x8c", // "⌚̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xf0\x9f\x87\xa6", // "⌚🇦"
        expected: &[
            b"\xe2\x8c\x9a",     // "⌚"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xf0\x9f\x87\xa6", // "⌚̈🇦"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xd8\x80", // "⌚\u0600"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xd8\x80", // "⌚̈\u0600"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xa8\x83", // "⌚ਃ"
        expected: &[
            b"\xe2\x8c\x9a\xe0\xa8\x83", // "⌚ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xa8\x83", // "⌚̈ਃ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xe0\xa8\x83", // "⌚̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe1\x84\x80", // "⌚ᄀ"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe1\x84\x80", // "⌚̈ᄀ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe1\x85\xa0", // "⌚ᅠ"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe1\x85\xa0", // "⌚̈ᅠ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe1\x86\xa8", // "⌚ᆨ"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe1\x86\xa8", // "⌚̈ᆨ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xea\xb0\x80", // "⌚가"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xea\xb0\x80", // "⌚̈가"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xea\xb0\x81", // "⌚각"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xea\xb0\x81", // "⌚̈각"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xa4\x83", // "⌚ः"
        expected: &[
            b"\xe2\x8c\x9a\xe0\xa4\x83", // "⌚ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xa4\x83", // "⌚̈ः"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xe0\xa4\x83", // "⌚̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xa4\x84", // "⌚ऄ"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xa4\x84", // "⌚̈ऄ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xb5\x8e", // "⌚ൎ"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xb5\x8e", // "⌚̈ൎ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xa4\x95", // "⌚क"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xa4\x95", // "⌚̈क"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe2\x8c\x9a", // "⌚⌚"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe2\x8c\x9a", // "⌚̈⌚"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x80", // "⌚̀"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x80", // "⌚̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xcc\x80", // "⌚̈̀"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xcc\x80", // "⌚̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xa4\x80", // "⌚ऀ"
        expected: &[
            b"\xe2\x8c\x9a\xe0\xa4\x80", // "⌚ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xa4\x80", // "⌚̈ऀ"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xe0\xa4\x80", // "⌚̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe0\xa5\x8d", // "⌚्"
        expected: &[
            b"\xe2\x8c\x9a\xe0\xa5\x8d", // "⌚्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe0\xa5\x8d", // "⌚्̈"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xe0\xa5\x8d", // "⌚्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xe2\x80\x8d", // "⌚\u200d"
        expected: &[
            b"\xe2\x8c\x9a\xe2\x80\x8d", // "⌚\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xe2\x80\x8d", // "⌚̈\u200d"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88\xe2\x80\x8d", // "⌚̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcd\xb8", // "⌚\u0378"
        expected: &[
            b"\xe2\x8c\x9a", // "⌚"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] WATCH (ExtPict) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe2\x8c\x9a\xcc\x88\xcd\xb8", // "⌚̈\u0378"
        expected: &[
            b"\xe2\x8c\x9a\xcc\x88", // "⌚̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xcc\x80 ", // "̀ "
        expected: &[
            b"\xcc\x80", // "̀"
            b" ",        // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88 ", // "̀̈ "
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b" ",                // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xcc\x80\r", // "̀\r"
        expected: &[
            b"\xcc\x80", // "̀"
            b"\r",       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\r", // "̀̈\r"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\r",               // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xcc\x80\n", // "̀\n"
        expected: &[
            b"\xcc\x80", // "̀"
            b"\n",       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\n", // "̀̈\n"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\n",               // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xcc\x80\x01", // "̀\x01"
        expected: &[
            b"\xcc\x80", // "̀"
            b"\x01",     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\x01", // "̀̈\x01"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\x01",             // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xcc\x80\xe2\x80\x8c", // "̀\u200c"
        expected: &[
            b"\xcc\x80\xe2\x80\x8c", // "̀\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe2\x80\x8c", // "̀̈\u200c"
        expected: &[
            b"\xcc\x80\xcc\x88\xe2\x80\x8c", // "̀̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xcc\x80\xf0\x9f\x87\xa6", // "̀🇦"
        expected: &[
            b"\xcc\x80",         // "̀"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xf0\x9f\x87\xa6", // "̀̈🇦"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xcc\x80\xd8\x80", // "̀\u0600"
        expected: &[
            b"\xcc\x80", // "̀"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xd8\x80", // "̀̈\u0600"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xd8\x80",         // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xa8\x83", // "̀ਃ"
        expected: &[
            b"\xcc\x80\xe0\xa8\x83", // "̀ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xa8\x83", // "̀̈ਃ"
        expected: &[
            b"\xcc\x80\xcc\x88\xe0\xa8\x83", // "̀̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xcc\x80\xe1\x84\x80", // "̀ᄀ"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe1\x84\x80", // "̀̈ᄀ"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe1\x84\x80",     // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xcc\x80\xe1\x85\xa0", // "̀ᅠ"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe1\x85\xa0", // "̀̈ᅠ"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe1\x85\xa0",     // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xcc\x80\xe1\x86\xa8", // "̀ᆨ"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe1\x86\xa8", // "̀̈ᆨ"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe1\x86\xa8",     // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xcc\x80\xea\xb0\x80", // "̀가"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xea\xb0\x80", // "̀̈가"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xea\xb0\x80",     // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xcc\x80\xea\xb0\x81", // "̀각"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xea\xb0\x81", // "̀̈각"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xea\xb0\x81",     // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xa4\x83", // "̀ः"
        expected: &[
            b"\xcc\x80\xe0\xa4\x83", // "̀ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xa4\x83", // "̀̈ः"
        expected: &[
            b"\xcc\x80\xcc\x88\xe0\xa4\x83", // "̀̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xa4\x84", // "̀ऄ"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xa4\x84", // "̀̈ऄ"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe0\xa4\x84",     // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xb5\x8e", // "̀ൎ"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xb5\x8e", // "̀̈ൎ"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe0\xb5\x8e",     // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xa4\x95", // "̀क"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xa4\x95", // "̀̈क"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe0\xa4\x95",     // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xcc\x80\xe2\x8c\x9a", // "̀⌚"
        expected: &[
            b"\xcc\x80",     // "̀"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe2\x8c\x9a", // "̀̈⌚"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xe2\x8c\x9a",     // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x80", // "̀̀"
        expected: &[
            b"\xcc\x80\xcc\x80", // "̀̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xcc\x80", // "̀̈̀"
        expected: &[
            b"\xcc\x80\xcc\x88\xcc\x80", // "̀̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xa4\x80", // "̀ऀ"
        expected: &[
            b"\xcc\x80\xe0\xa4\x80", // "̀ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xa4\x80", // "̀̈ऀ"
        expected: &[
            b"\xcc\x80\xcc\x88\xe0\xa4\x80", // "̀̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xe0\xa5\x8d", // "्̀"
        expected: &[
            b"\xcc\x80\xe0\xa5\x8d", // "्̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe0\xa5\x8d", // "्̀̈"
        expected: &[
            b"\xcc\x80\xcc\x88\xe0\xa5\x8d", // "्̀̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xe2\x80\x8d", // "̀\u200d"
        expected: &[
            b"\xcc\x80\xe2\x80\x8d", // "̀\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xe2\x80\x8d", // "̀̈\u200d"
        expected: &[
            b"\xcc\x80\xcc\x88\xe2\x80\x8d", // "̀̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xcc\x80\xcd\xb8", // "̀\u0378"
        expected: &[
            b"\xcc\x80", // "̀"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xcc\x80\xcc\x88\xcd\xb8", // "̀̈\u0378"
        expected: &[
            b"\xcc\x80\xcc\x88", // "̀̈"
            b"\xcd\xb8",         // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x80 ", // "ऀ "
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88 ", // "ऀ̈ "
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x80\r", // "ऀ\r"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\r", // "ऀ̈\r"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x80\n", // "ऀ\n"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\n", // "ऀ̈\n"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x80\x01", // "ऀ\x01"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\x01", // "ऀ̈\x01"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe2\x80\x8c", // "ऀ\u200c"
        expected: &[
            b"\xe0\xa4\x80\xe2\x80\x8c", // "ऀ\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe2\x80\x8c", // "ऀ̈\u200c"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xe2\x80\x8c", // "ऀ̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xf0\x9f\x87\xa6", // "ऀ🇦"
        expected: &[
            b"\xe0\xa4\x80",     // "ऀ"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xf0\x9f\x87\xa6", // "ऀ̈🇦"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xd8\x80", // "ऀ\u0600"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xd8\x80", // "ऀ̈\u0600"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xa8\x83", // "ऀਃ"
        expected: &[
            b"\xe0\xa4\x80\xe0\xa8\x83", // "ऀਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xa8\x83", // "ऀ̈ਃ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xe0\xa8\x83", // "ऀ̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe1\x84\x80", // "ऀᄀ"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe1\x84\x80", // "ऀ̈ᄀ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe1\x85\xa0", // "ऀᅠ"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe1\x85\xa0", // "ऀ̈ᅠ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe1\x86\xa8", // "ऀᆨ"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe1\x86\xa8", // "ऀ̈ᆨ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xea\xb0\x80", // "ऀ가"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xea\xb0\x80", // "ऀ̈가"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xea\xb0\x81", // "ऀ각"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xea\xb0\x81", // "ऀ̈각"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xa4\x83", // "ऀः"
        expected: &[
            b"\xe0\xa4\x80\xe0\xa4\x83", // "ऀः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xa4\x83", // "ऀ̈ः"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xe0\xa4\x83", // "ऀ̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xa4\x84", // "ऀऄ"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xa4\x84", // "ऀ̈ऄ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xb5\x8e", // "ऀൎ"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xb5\x8e", // "ऀ̈ൎ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xa4\x95", // "ऀक"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xa4\x95", // "ऀ̈क"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe2\x8c\x9a", // "ऀ⌚"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe2\x8c\x9a", // "ऀ̈⌚"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x80", // "ऀ̀"
        expected: &[
            b"\xe0\xa4\x80\xcc\x80", // "ऀ̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xcc\x80", // "ऀ̈̀"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xcc\x80", // "ऀ̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xa4\x80", // "ऀऀ"
        expected: &[
            b"\xe0\xa4\x80\xe0\xa4\x80", // "ऀऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xa4\x80", // "ऀ̈ऀ"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xe0\xa4\x80", // "ऀ̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe0\xa5\x8d", // "ऀ्"
        expected: &[
            b"\xe0\xa4\x80\xe0\xa5\x8d", // "ऀ्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe0\xa5\x8d", // "ऀ्̈"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xe0\xa5\x8d", // "ऀ्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xe2\x80\x8d", // "ऀ\u200d"
        expected: &[
            b"\xe0\xa4\x80\xe2\x80\x8d", // "ऀ\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xe2\x80\x8d", // "ऀ̈\u200d"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88\xe2\x80\x8d", // "ऀ̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcd\xb8", // "ऀ\u0378"
        expected: &[
            b"\xe0\xa4\x80", // "ऀ"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x80\xcc\x88\xcd\xb8", // "ऀ̈\u0378"
        expected: &[
            b"\xe0\xa4\x80\xcc\x88", // "ऀ̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa5\x8d ", // "् "
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88 ", // "्̈ "
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\r", // "्\r"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\r", // "्̈\r"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\n", // "्\n"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\n", // "्̈\n"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\x01", // "्\x01"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\x01", // "्̈\x01"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe2\x80\x8c", // "्\u200c"
        expected: &[
            b"\xe0\xa5\x8d\xe2\x80\x8c", // "्\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe2\x80\x8c", // "्̈\u200c"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xe2\x80\x8c", // "्̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xf0\x9f\x87\xa6", // "्🇦"
        expected: &[
            b"\xe0\xa5\x8d",     // "्"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xf0\x9f\x87\xa6", // "्̈🇦"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xd8\x80", // "्\u0600"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xd8\x80", // "्̈\u0600"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xa8\x83", // "्ਃ"
        expected: &[
            b"\xe0\xa5\x8d\xe0\xa8\x83", // "्ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xa8\x83", // "्̈ਃ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xe0\xa8\x83", // "्̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe1\x84\x80", // "्ᄀ"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe1\x84\x80", // "्̈ᄀ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe1\x85\xa0", // "्ᅠ"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe1\x85\xa0", // "्̈ᅠ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe1\x86\xa8", // "्ᆨ"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe1\x86\xa8", // "्̈ᆨ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xea\xb0\x80", // "्가"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xea\xb0\x80", // "्̈가"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xea\xb0\x81", // "्각"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xea\xb0\x81", // "्̈각"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xa4\x83", // "्ः"
        expected: &[
            b"\xe0\xa5\x8d\xe0\xa4\x83", // "्ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xa4\x83", // "्̈ः"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xe0\xa4\x83", // "्̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xa4\x84", // "्ऄ"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xa4\x84", // "्̈ऄ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xb5\x8e", // "्ൎ"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xb5\x8e", // "्̈ൎ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xa4\x95", // "्क"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xa4\x95", // "्̈क"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe2\x8c\x9a", // "्⌚"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe2\x8c\x9a", // "्̈⌚"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x80", // "्̀"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x80", // "्̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xcc\x80", // "्̈̀"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xcc\x80", // "्̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xa4\x80", // "्ऀ"
        expected: &[
            b"\xe0\xa5\x8d\xe0\xa4\x80", // "्ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xa4\x80", // "्̈ऀ"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xe0\xa4\x80", // "्̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe0\xa5\x8d", // "््"
        expected: &[
            b"\xe0\xa5\x8d\xe0\xa5\x8d", // "््"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe0\xa5\x8d", // "््̈"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xe0\xa5\x8d", // "््̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xe2\x80\x8d", // "्\u200d"
        expected: &[
            b"\xe0\xa5\x8d\xe2\x80\x8d", // "्\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xe2\x80\x8d", // "्̈\u200d"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88\xe2\x80\x8d", // "्̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcd\xb8", // "्\u0378"
        expected: &[
            b"\xe0\xa5\x8d", // "्"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe0\xa5\x8d\xcc\x88\xcd\xb8", // "्̈\u0378"
        expected: &[
            b"\xe0\xa5\x8d\xcc\x88", // "्̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8d ", // "\u200d "
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b" ",            // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88 ", // "\u200d̈ "
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe2\x80\x8d\r", // "\u200d\r"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\r",           // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\r", // "\u200d̈\r"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\r",                   // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe2\x80\x8d\n", // "\u200d\n"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\n",           // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\n", // "\u200d̈\n"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\n",                   // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe2\x80\x8d\x01", // "\u200d\x01"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\x01",         // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\x01", // "\u200d̈\x01"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\x01",                 // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe2\x80\x8c", // "\u200d\u200c"
        expected: &[
            b"\xe2\x80\x8d\xe2\x80\x8c", // "\u200d\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe2\x80\x8c", // "\u200d̈\u200c"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xe2\x80\x8c", // "\u200d̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xf0\x9f\x87\xa6", // "\u200d🇦"
        expected: &[
            b"\xe2\x80\x8d",     // "\u200d"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xf0\x9f\x87\xa6", // "\u200d̈🇦"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xf0\x9f\x87\xa6",     // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xd8\x80", // "\u200d\u0600"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xd8\x80",     // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xd8\x80", // "\u200d̈\u0600"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xd8\x80",             // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xa8\x83", // "\u200dਃ"
        expected: &[
            b"\xe2\x80\x8d\xe0\xa8\x83", // "\u200dਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xa8\x83", // "\u200d̈ਃ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xe0\xa8\x83", // "\u200d̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe1\x84\x80", // "\u200dᄀ"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe1\x84\x80", // "\u200d̈ᄀ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe1\x84\x80",         // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe1\x85\xa0", // "\u200dᅠ"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe1\x85\xa0", // "\u200d̈ᅠ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe1\x85\xa0",         // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe1\x86\xa8", // "\u200dᆨ"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe1\x86\xa8", // "\u200d̈ᆨ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe1\x86\xa8",         // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xea\xb0\x80", // "\u200d가"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xea\xb0\x80", // "\u200d̈가"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xea\xb0\x80",         // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xea\xb0\x81", // "\u200d각"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xea\xb0\x81", // "\u200d̈각"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xea\xb0\x81",         // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xa4\x83", // "\u200dः"
        expected: &[
            b"\xe2\x80\x8d\xe0\xa4\x83", // "\u200dः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xa4\x83", // "\u200d̈ः"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xe0\xa4\x83", // "\u200d̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xa4\x84", // "\u200dऄ"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xa4\x84", // "\u200d̈ऄ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe0\xa4\x84",         // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xb5\x8e", // "\u200dൎ"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xb5\x8e", // "\u200d̈ൎ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe0\xb5\x8e",         // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xa4\x95", // "\u200dक"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xa4\x95", // "\u200d̈क"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe0\xa4\x95",         // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe2\x8c\x9a", // "\u200d⌚"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe2\x8c\x9a", // "\u200d̈⌚"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xe2\x8c\x9a",         // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x80", // "\u200d̀"
        expected: &[
            b"\xe2\x80\x8d\xcc\x80", // "\u200d̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xcc\x80", // "\u200d̈̀"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xcc\x80", // "\u200d̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xa4\x80", // "\u200dऀ"
        expected: &[
            b"\xe2\x80\x8d\xe0\xa4\x80", // "\u200dऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xa4\x80", // "\u200d̈ऀ"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xe0\xa4\x80", // "\u200d̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe0\xa5\x8d", // "\u200d्"
        expected: &[
            b"\xe2\x80\x8d\xe0\xa5\x8d", // "\u200d्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe0\xa5\x8d", // "\u200d्̈"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xe0\xa5\x8d", // "\u200d्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xe2\x80\x8d", // "\u200d\u200d"
        expected: &[
            b"\xe2\x80\x8d\xe2\x80\x8d", // "\u200d\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xe2\x80\x8d", // "\u200d̈\u200d"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88\xe2\x80\x8d", // "\u200d̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcd\xb8", // "\u200d\u0378"
        expected: &[
            b"\xe2\x80\x8d", // "\u200d"
            b"\xcd\xb8",     // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xe2\x80\x8d\xcc\x88\xcd\xb8", // "\u200d̈\u0378"
        expected: &[
            b"\xe2\x80\x8d\xcc\x88", // "\u200d̈"
            b"\xcd\xb8",             // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xcd\xb8 ", // "\u0378 "
        expected: &[
            b"\xcd\xb8", // "\u0378"
            b" ",        // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88 ", // "\u0378̈ "
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b" ",                // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xcd\xb8\r", // "\u0378\r"
        expected: &[
            b"\xcd\xb8", // "\u0378"
            b"\r",       // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <CARRIAGE RETURN (CR)> (CR) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\r", // "\u0378̈\r"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\r",               // "\r"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xcd\xb8\n", // "\u0378\n"
        expected: &[
            b"\xcd\xb8", // "\u0378"
            b"\n",       // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\n", // "\u0378̈\n"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\n",               // "\n"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xcd\xb8\x01", // "\u0378\x01"
        expected: &[
            b"\xcd\xb8", // "\u0378"
            b"\x01",     // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [5.0] <START OF HEADING> (Control) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\x01", // "\u0378̈\x01"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\x01",             // "\x01"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xcd\xb8\xe2\x80\x8c", // "\u0378\u200c"
        expected: &[
            b"\xcd\xb8\xe2\x80\x8c", // "\u0378\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH NON-JOINER (Extend) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe2\x80\x8c", // "\u0378̈\u200c"
        expected: &[
            b"\xcd\xb8\xcc\x88\xe2\x80\x8c", // "\u0378̈\u200c"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xcd\xb8\xf0\x9f\x87\xa6", // "\u0378🇦"
        expected: &[
            b"\xcd\xb8",         // "\u0378"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xf0\x9f\x87\xa6", // "\u0378̈🇦"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xf0\x9f\x87\xa6", // "🇦"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xcd\xb8\xd8\x80", // "\u0378\u0600"
        expected: &[
            b"\xcd\xb8", // "\u0378"
            b"\xd8\x80", // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xd8\x80", // "\u0378̈\u0600"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xd8\x80",         // "\u0600"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xa8\x83", // "\u0378ਃ"
        expected: &[
            b"\xcd\xb8\xe0\xa8\x83", // "\u0378ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] GURMUKHI SIGN VISARGA (SpacingMark) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xa8\x83", // "\u0378̈ਃ"
        expected: &[
            b"\xcd\xb8\xcc\x88\xe0\xa8\x83", // "\u0378̈ਃ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xcd\xb8\xe1\x84\x80", // "\u0378ᄀ"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe1\x84\x80", // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe1\x84\x80", // "\u0378̈ᄀ"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe1\x84\x80",     // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xcd\xb8\xe1\x85\xa0", // "\u0378ᅠ"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe1\x85\xa0", // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JUNGSEONG FILLER (V) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe1\x85\xa0", // "\u0378̈ᅠ"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe1\x85\xa0",     // "ᅠ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xcd\xb8\xe1\x86\xa8", // "\u0378ᆨ"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe1\x86\xa8", // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL JONGSEONG KIYEOK (T) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe1\x86\xa8", // "\u0378̈ᆨ"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe1\x86\xa8",     // "ᆨ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xcd\xb8\xea\xb0\x80", // "\u0378가"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xea\xb0\x80", // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GA (LV) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xea\xb0\x80", // "\u0378̈가"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xea\xb0\x80",     // "가"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xcd\xb8\xea\xb0\x81", // "\u0378각"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xea\xb0\x81", // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] HANGUL SYLLABLE GAG (LVT) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xea\xb0\x81", // "\u0378̈각"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xea\xb0\x81",     // "각"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xa4\x83", // "\u0378ः"
        expected: &[
            b"\xcd\xb8\xe0\xa4\x83", // "\u0378ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xa4\x83", // "\u0378̈ः"
        expected: &[
            b"\xcd\xb8\xcc\x88\xe0\xa4\x83", // "\u0378̈ः"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xa4\x84", // "\u0378ऄ"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe0\xa4\x84", // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER SHORT A (ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xa4\x84", // "\u0378̈ऄ"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe0\xa4\x84",     // "ऄ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xb5\x8e", // "\u0378ൎ"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe0\xb5\x8e", // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] MALAYALAM LETTER DOT REPH (Prepend_ConjunctLinkingScripts) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xb5\x8e", // "\u0378̈ൎ"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe0\xb5\x8e",     // "ൎ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xa4\x95", // "\u0378क"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe0\xa4\x95", // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xa4\x95", // "\u0378̈क"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe0\xa4\x95",     // "क"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xcd\xb8\xe2\x8c\x9a", // "\u0378⌚"
        expected: &[
            b"\xcd\xb8",     // "\u0378"
            b"\xe2\x8c\x9a", // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] WATCH (ExtPict) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe2\x8c\x9a", // "\u0378̈⌚"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xe2\x8c\x9a",     // "⌚"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x80", // "\u0378̀"
        expected: &[
            b"\xcd\xb8\xcc\x80", // "\u0378̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] COMBINING GRAVE ACCENT (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xcc\x80", // "\u0378̈̀"
        expected: &[
            b"\xcd\xb8\xcc\x88\xcc\x80", // "\u0378̈̀"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xa4\x80", // "\u0378ऀ"
        expected: &[
            b"\xcd\xb8\xe0\xa4\x80", // "\u0378ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN INVERTED CANDRABINDU (Extend_ConjunctLinkingScripts_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xa4\x80", // "\u0378̈ऀ"
        expected: &[
            b"\xcd\xb8\xcc\x88\xe0\xa4\x80", // "\u0378̈ऀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xe0\xa5\x8d", // "\u0378्"
        expected: &[
            b"\xcd\xb8\xe0\xa5\x8d", // "\u0378्"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe0\xa5\x8d", // "\u0378्̈"
        expected: &[
            b"\xcd\xb8\xcc\x88\xe0\xa5\x8d", // "\u0378्̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xe2\x80\x8d", // "\u0378\u200d"
        expected: &[
            b"\xcd\xb8\xe2\x80\x8d", // "\u0378\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xe2\x80\x8d", // "\u0378̈\u200d"
        expected: &[
            b"\xcd\xb8\xcc\x88\xe2\x80\x8d", // "\u0378̈\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xcd\xb8\xcd\xb8", // "\u0378\u0378"
        expected: &[
            b"\xcd\xb8", // "\u0378"
            b"\xcd\xb8", // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <reserved-0378> (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] <reserved-0378> (Other) ÷ [0.3]",
        input: b"\xcd\xb8\xcc\x88\xcd\xb8", // "\u0378̈\u0378"
        expected: &[
            b"\xcd\xb8\xcc\x88", // "\u0378̈"
            b"\xcd\xb8",         // "\u0378"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] <CARRIAGE RETURN (CR)> (CR) × [3.0] <LINE FEED (LF)> (LF) ÷ [4.0] LATIN SMALL LETTER A (Other) ÷ [5.0] <LINE FEED (LF)> (LF) ÷ [4.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"\r\na\n\xcc\x88", // "\r\na\n̈"
        expected: &[
            b"\r\n",     // "\r\n"
            b"a",        // "a"
            b"\n",       // "\n"
            b"\xcc\x88", // "̈"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [0.3]",
        input: b"a\xcc\x88", // "ä"
        expected: &[
            b"a\xcc\x88", // "ä"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] SPACE (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] ARABIC LETTER NOON (Other) ÷ [0.3]",
        input: b" \xe2\x80\x8d\xd9\x86", // " \u200dن"
        expected: &[
            b" \xe2\x80\x8d", // " \u200d"
            b"\xd9\x86",      // "ن"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] ARABIC LETTER NOON (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] SPACE (Other) ÷ [0.3]",
        input: b"\xd9\x86\xe2\x80\x8d ", // "ن\u200d "
        expected: &[
            b"\xd9\x86\xe2\x80\x8d", // "ن\u200d"
            b" ",                    // " "
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL CHOSEONG KIYEOK (L) × [6.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xe1\x84\x80\xe1\x84\x80", // "ᄀᄀ"
        expected: &[
            b"\xe1\x84\x80\xe1\x84\x80", // "ᄀᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GA (LV) × [7.0] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xea\xb0\x80\xe1\x86\xa8\xe1\x84\x80", // "각ᄀ"
        expected: &[
            b"\xea\xb0\x80\xe1\x86\xa8", // "각"
            b"\xe1\x84\x80",             // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] HANGUL SYLLABLE GAG (LVT) × [8.0] HANGUL JONGSEONG KIYEOK (T) ÷ [999.0] HANGUL CHOSEONG KIYEOK (L) ÷ [0.3]",
        input: b"\xea\xb0\x81\xe1\x86\xa8\xe1\x84\x80", // "각ᆨᄀ"
        expected: &[
            b"\xea\xb0\x81\xe1\x86\xa8", // "각ᆨ"
            b"\xe1\x84\x80",             // "ᄀ"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [12.0] REGIONAL INDICATOR SYMBOL LETTER B (RI) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER C (RI) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7\xf0\x9f\x87\xa8b", // "🇦🇧🇨b"
        expected: &[
            b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7", // "🇦🇧"
            b"\xf0\x9f\x87\xa8",                 // "🇨"
            b"b",                                // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [13.0] REGIONAL INDICATOR SYMBOL LETTER B (RI) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER C (RI) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7\xf0\x9f\x87\xa8b", // "a🇦🇧🇨b"
        expected: &[
            b"a",                                // "a"
            b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7", // "🇦🇧"
            b"\xf0\x9f\x87\xa8",                 // "🇨"
            b"b",                                // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [13.0] REGIONAL INDICATOR SYMBOL LETTER B (RI) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER C (RI) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7\xe2\x80\x8d\xf0\x9f\x87\xa8b", // "a🇦🇧\u200d🇨b"
        expected: &[
            b"a",                                            // "a"
            b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7\xe2\x80\x8d", // "🇦🇧\u200d"
            b"\xf0\x9f\x87\xa8",                             // "🇨"
            b"b",                                            // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER B (RI) × [13.0] REGIONAL INDICATOR SYMBOL LETTER C (RI) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xf0\x9f\x87\xa6\xe2\x80\x8d\xf0\x9f\x87\xa7\xf0\x9f\x87\xa8b", // "a🇦\u200d🇧🇨b"
        expected: &[
            b"a",                                // "a"
            b"\xf0\x9f\x87\xa6\xe2\x80\x8d",     // "🇦\u200d"
            b"\xf0\x9f\x87\xa7\xf0\x9f\x87\xa8", // "🇧🇨"
            b"b",                                // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER A (RI) × [13.0] REGIONAL INDICATOR SYMBOL LETTER B (RI) ÷ [999.0] REGIONAL INDICATOR SYMBOL LETTER C (RI) × [13.0] REGIONAL INDICATOR SYMBOL LETTER D (RI) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7\xf0\x9f\x87\xa8\xf0\x9f\x87\xa9b", // "a🇦🇧🇨🇩b"
        expected: &[
            b"a",                                // "a"
            b"\xf0\x9f\x87\xa6\xf0\x9f\x87\xa7", // "🇦🇧"
            b"\xf0\x9f\x87\xa8\xf0\x9f\x87\xa9", // "🇨🇩"
            b"b",                                // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [0.3]",
        input: b"a\xe2\x80\x8d", // "a\u200d"
        expected: &[
            b"a\xe2\x80\x8d", // "a\u200d"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xcc\x88b", // "äb"
        expected: &[
            b"a\xcc\x88", // "ä"
            b"b",         // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.1] DEVANAGARI SIGN VISARGA (SpacingMark_ConjunctLinkingScripts) ÷ [999.0] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xe0\xa4\x83b", // "aःb"
        expected: &[
            b"a\xe0\xa4\x83", // "aः"
            b"b",             // "b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) ÷ [999.0] ARABIC NUMBER SIGN (Prepend) × [9.2] LATIN SMALL LETTER B (Other) ÷ [0.3]",
        input: b"a\xd8\x80b", // "a\u0600b"
        expected: &[
            b"a",         // "a"
            b"\xd8\x80b", // "\u0600b"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] BABY (ExtPict) × [9.0] EMOJI MODIFIER FITZPATRICK TYPE-6 (Extend_ExtCccZwj) ÷ [999.0] BABY (ExtPict) ÷ [0.3]",
        input: b"\xf0\x9f\x91\xb6\xf0\x9f\x8f\xbf\xf0\x9f\x91\xb6", // "👶🏿👶"
        expected: &[
            b"\xf0\x9f\x91\xb6\xf0\x9f\x8f\xbf", // "👶🏿"
            b"\xf0\x9f\x91\xb6",                 // "👶"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] EMOJI MODIFIER FITZPATRICK TYPE-6 (Extend_ExtCccZwj) ÷ [999.0] BABY (ExtPict) ÷ [0.3]",
        input: b"a\xf0\x9f\x8f\xbf\xf0\x9f\x91\xb6", // "a🏿👶"
        expected: &[
            b"a\xf0\x9f\x8f\xbf", // "a🏿"
            b"\xf0\x9f\x91\xb6",  // "👶"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] EMOJI MODIFIER FITZPATRICK TYPE-6 (Extend_ExtCccZwj) ÷ [999.0] BABY (ExtPict) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [11.0] OCTAGONAL SIGN (ExtPict) ÷ [0.3]",
        input: b"a\xf0\x9f\x8f\xbf\xf0\x9f\x91\xb6\xe2\x80\x8d\xf0\x9f\x9b\x91", // "a🏿👶\u200d🛑"
        expected: &[
            b"a\xf0\x9f\x8f\xbf",                            // "a🏿"
            b"\xf0\x9f\x91\xb6\xe2\x80\x8d\xf0\x9f\x9b\x91", // "👶\u200d🛑"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] BABY (ExtPict) × [9.0] EMOJI MODIFIER FITZPATRICK TYPE-6 (Extend_ExtCccZwj) × [9.0] COMBINING DIAERESIS (Extend_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [11.0] BABY (ExtPict) × [9.0] EMOJI MODIFIER FITZPATRICK TYPE-6 (Extend_ExtCccZwj) ÷ [0.3]",
        input:
            b"\xf0\x9f\x91\xb6\xf0\x9f\x8f\xbf\xcc\x88\xe2\x80\x8d\xf0\x9f\x91\xb6\xf0\x9f\x8f\xbf", // "👶🏿̈\u200d👶🏿"
        expected: &[
            b"\xf0\x9f\x91\xb6\xf0\x9f\x8f\xbf\xcc\x88\xe2\x80\x8d\xf0\x9f\x91\xb6\xf0\x9f\x8f\xbf", // "👶🏿̈\u200d👶🏿"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] OCTAGONAL SIGN (ExtPict) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [11.0] OCTAGONAL SIGN (ExtPict) ÷ [0.3]",
        input: b"\xf0\x9f\x9b\x91\xe2\x80\x8d\xf0\x9f\x9b\x91", // "🛑\u200d🛑"
        expected: &[
            b"\xf0\x9f\x9b\x91\xe2\x80\x8d\xf0\x9f\x9b\x91", // "🛑\u200d🛑"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] OCTAGONAL SIGN (ExtPict) ÷ [0.3]",
        input: b"a\xe2\x80\x8d\xf0\x9f\x9b\x91", // "a\u200d🛑"
        expected: &[
            b"a\xe2\x80\x8d",    // "a\u200d"
            b"\xf0\x9f\x9b\x91", // "🛑"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] UPPER BLADE SCISSORS (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [11.0] UPPER BLADE SCISSORS (Other) ÷ [0.3]",
        input: b"\xe2\x9c\x81\xe2\x80\x8d\xe2\x9c\x81", // "✁\u200d✁"
        expected: &[
            b"\xe2\x9c\x81\xe2\x80\x8d\xe2\x9c\x81", // "✁\u200d✁"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) ÷ [999.0] UPPER BLADE SCISSORS (Other) ÷ [0.3]",
        input: b"a\xe2\x80\x8d\xe2\x9c\x81", // "a\u200d✁"
        expected: &[
            b"a\xe2\x80\x8d", // "a\u200d"
            b"\xe2\x9c\x81",  // "✁"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) ÷ [999.0] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\xa4", // "कत"
        expected: &[
            b"\xe0\xa4\x95", // "क"
            b"\xe0\xa4\xa4", // "त"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa4\xa4", // "क्त"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa4\xa4", // "क्त"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa5\x8d\xe0\xa4\xa4", // "क््त"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa5\x8d\xe0\xa4\xa4", // "क््त"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8d\xe2\x80\x8d\xe0\xa4\xa4", // "क्\u200dत"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d\xe2\x80\x8d\xe0\xa4\xa4", // "क्\u200dत"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN NUKTA (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\xbc\xe2\x80\x8d\xe0\xa5\x8d\xe0\xa4\xa4", // "क़\u200d्त"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa4\xbc\xe2\x80\x8d\xe0\xa5\x8d\xe0\xa4\xa4", // "क़\u200d्त"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN NUKTA (Extend_ConjunctLinkingScripts_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] ZERO WIDTH JOINER (ZWJ_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa4\xbc\xe0\xa5\x8d\xe2\x80\x8d\xe0\xa4\xa4", // "क़्\u200dत"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa4\xbc\xe0\xa5\x8d\xe2\x80\x8d\xe0\xa4\xa4", // "क़्\u200dत"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.3] DEVANAGARI LETTER YA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa4\xa4\xe0\xa5\x8d\xe0\xa4\xaf", // "क्त्य"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa4\xa4\xe0\xa5\x8d\xe0\xa4\xaf", // "क्त्य"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] LATIN SMALL LETTER A (Other) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8da", // "क्a"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d", // "क्"
            b"a",                        // "a"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] LATIN SMALL LETTER A (Other) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"a\xe0\xa5\x8d\xe0\xa4\xa4", // "a्त"
        expected: &[
            b"a\xe0\xa5\x8d", // "a्"
            b"\xe0\xa4\xa4",  // "त"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] QUESTION MARK (Other) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) ÷ [999.0] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"?\xe0\xa5\x8d\xe0\xa4\xa4", // "?्त"
        expected: &[
            b"?\xe0\xa5\x8d", // "?्"
            b"\xe0\xa4\xa4",  // "त"
        ],
    },
    SegmentationTest {
        desc: "÷ [0.2] DEVANAGARI LETTER KA (ConjunctLinkingScripts_LinkingConsonant) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.0] DEVANAGARI SIGN VIRAMA (Extend_ConjunctLinkingScripts_ConjunctLinker_ExtCccZwj) × [9.3] DEVANAGARI LETTER TA (ConjunctLinkingScripts_LinkingConsonant) ÷ [0.3]",
        input: b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa5\x8d\xe0\xa4\xa4", // "क््त"
        expected: &[
            b"\xe0\xa4\x95\xe0\xa5\x8d\xe0\xa5\x8d\xe0\xa4\xa4", // "क््त"
        ],
    },
];
