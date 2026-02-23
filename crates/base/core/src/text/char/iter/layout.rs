// devela_base_core::text::char::iter::layout

use crate::{
    CharIter,
    TextCohesion::{Atomic, Breakable},
    TextSymbol, is,
};

macro_rules! impl_text_layout {
    ($($T:ty),+) => { $( impl_text_layout![%$T]; )+ };
    (%$T:ty) => {
        /// Methods related to text layout
        impl<'a> CharIter<'a, $T> {
            /// Fills `out` with layout symbols derived from Unicode scalars.
            /// Returns the number of symbols written.
            pub const fn fill_text_symbols(mut self, out: &mut [TextSymbol]) -> usize {
                let mut len = 0;
                while let Some(ch) = self.next_char() {
                    is![len == out.len(), break];
                    out[len] = TextSymbol {
                        units: 1,
                        cohesion: is![ch.is_whitespace(), Breakable, Atomic],
                    };
                    len += 1;
                }
                len
            }
        }
    };
}
impl_text_layout![&[u8], &str];
