// devela/src/sys/os/term/grid/style.rs
//
//! Defines [`TermStyle`] and [`TermStyleExt`].
//

crate::set! {
    #[doc = crate::_tags!(term text set)]
    /// A compact set of broadly supported terminal text styles.
    #[doc = crate::_doc_meta!{
        location("sys/os/term"),
        test_size_of(TermStyle = 1|8),
    }]
    ///
    /// Its bits occupy the low byte of [`TermStyleExt`] with the same meanings.
    #[must_use]
    pub struct TermStyle(u8) {
        /// Increased intensity.
        BOLD = 0;
        /// Italic text.
        ITALIC = 1;
        /// Underlined text.
        UNDERLINE = 2;
        /// Reduced intensity.
        DIM = 3;
        /// Blinking text.
        BLINK = 4;
        /// Swapped foreground and background.
        INVERSE = 5;
        /// Concealed text.
        HIDDEN = 6;
        /// Crossed-out text.
        CROSSED = 7;
    }
    impl {
        /// Returns the corresponding extended style set.
        pub const fn extended(self) -> TermStyleExt {
            TermStyleExt::from_bits(self.bits as u16)
        }
    }
}

crate::set! {
    #[doc = crate::_tags!(term text set)]
    /// A complete set of terminal text styles.
    #[doc = crate::_doc_meta!{
        location("sys/os/term"),
        test_size_of(TermStyleExt = 2|16),
    }]
    ///
    /// The low byte has the same representation as [`TermStyle`].
    /// Styles in the high byte generally have more limited terminal support.
    #[must_use]
    pub struct TermStyleExt(u16) {

        /* common low byte */

        /// Increased intensity.
        BOLD = 0;
        /// Italic text.
        ITALIC = 1;
        /// Underlined text.
        UNDERLINE = 2;
        /// Reduced intensity.
        DIM = 3;
        /// Blinking text.
        BLINK = 4;
        /// Swapped foreground and background.
        INVERSE = 5;
        /// Concealed text.
        HIDDEN = 6;
        /// Crossed-out text.
        CROSSED = 7;

        /* extended high byte */

        /// Rapidly blinking text.
        BLINK_FAST = 8;
        /// Fraktur text.
        FRAKTUR = 9;
        /// Doubly underlined text.
        UNDERLINE_DOUBLE = 10;
        /// Framed text.
        FRAMED = 11;
        /// Encircled text.
        ENCIRCLED = 12;
        /// Overlined text.
        OVERLINE = 13;
        /// Superscript text.
        SUPERSCRIPT = 14;
        /// Subscript text.
        SUBSCRIPT = 15;

        /// All basic styles.
        BASIC = 0..=7;
        /// All extended styles.
        EXTENDED = 8..=15;
    }
    impl {
        /// Returns the basic styles.
        pub const fn basic(self) -> TermStyle { TermStyle::from_bits(self.bits as u8) }

        /// Returns whether no extended styles are enabled.
        #[must_use]
        pub const fn is_basic(self) -> bool { !self.intersects(Self::EXTENDED) }

        /// Returns the basic style set when no extended styles are enabled.
        #[must_use]
        pub const fn try_basic(self) -> Option<TermStyle> {
            if self.is_basic() { Some(self.basic()) } else { None }
        }

        /// Returns the enabled extended-only styles.
        pub const fn extended_only(self) -> Self { self.intersection(Self::EXTENDED) }
    }
}
impl From<TermStyle> for TermStyleExt {
    fn from(style: TermStyle) -> Self {
        style.extended()
    }
}

#[cfg(test)]
mod tests {
    use super::{TermStyle as S, TermStyleExt as X};

    #[test]
    fn sizes() {
        assert_eq!(size_of::<S>(), 1);
        assert_eq!(size_of::<X>(), 2);
        assert_eq!(size_of::<Option<S>>(), 2);
        assert_eq!(size_of::<Option<X>>(), 4);
    }
    #[test]
    fn basic_layout_matches() {
        assert_eq!(S::all().extended(), X::BASIC);
        assert_eq!(S::BOLD.extended(), X::BOLD);
        assert_eq!(S::CROSSED.extended(), X::CROSSED);
    }
    #[test]
    fn set_operations() {
        let style = S::BOLD | S::ITALIC | S::UNDERLINE;
        assert!(style.contains(S::BOLD | S::ITALIC));
        assert!(style.intersects(S::UNDERLINE));
        assert!(!style.intersects(S::BLINK));
        assert_eq!(style.without(S::ITALIC), S::BOLD | S::UNDERLINE);
    }
    #[test]
    fn extended_partition() {
        let style = X::BOLD | X::FRAMED | X::OVERLINE;
        assert_eq!(style.basic(), S::BOLD);
        assert_eq!(style.extended_only(), X::FRAMED | X::OVERLINE);
        assert!(!style.is_basic());
        assert_eq!(style.try_basic(), None);
        let basic = X::BOLD | X::UNDERLINE;
        assert!(basic.is_basic());
        assert_eq!(basic.try_basic(), Some(S::BOLD | S::UNDERLINE));
    }
    #[test]
    fn style_ranges() {
        assert_eq!(X::BASIC.bits(), 0x00FF);
        assert_eq!(X::EXTENDED.bits(), 0xFF00);
        assert_eq!((X::BASIC | X::EXTENDED).bits(), u16::MAX);
        assert!(!X::BASIC.intersects(X::EXTENDED));
    }
}
