// devela_base_core::text::grapheme::scanner::layout

use crate::{GraphemeMachine, GraphemeScanner, TextCohesion::Atomic, TextSymbol, charu, is};

macro_rules! impl_text_layout {
    ($($T:ty),+) => { $( impl_text_layout![%$T]; )+ };
    (%$T:ty) => {
        /// Methods related to text layout.
        impl<'a> GraphemeScanner<'a, $T> {
            /// Fills `out` with layout symbols derived from grapheme clusters.
            pub const fn fill_text_symbols<const CAP: usize>(
                machine: &mut GraphemeMachine,
                text: &str,
                out: &mut [TextSymbol],
            ) -> usize {
                let mut scan = GraphemeScanner::<charu>::new(machine, text);
                let mut len = 0;
                while let Some(_g) = scan.next_grapheme_u8::<CAP>() {
                    is![len == out.len(); break];
                    out[len] = TextSymbol {
                        units: 1,
                        cohesion: Atomic, // IMPROVE: TEMP naive impl
                    };
                    len += 1;
                }
                len
            }
        }
    };
}
impl_text_layout![char, charu];
