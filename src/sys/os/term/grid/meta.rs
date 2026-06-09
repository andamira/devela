// devela::sys::os::term::grid::meta
//
//! Defines [`TermelMeta`] and [`TermelOccupancy`].
//

use TermelOccupancy::{Continuation, Origin, Reserved, Unoccupied};

#[doc = crate::_tags!(term text layout bit)]
/// The cell-space occupancy represented by terminal-element metadata.
#[doc = crate::_doc_meta!{location("sys/os/term/grid")}]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TermelOccupancy {
    /// No terminal element occupies this position.
    #[default]
    Unoccupied = 0,
    /// The position begins a terminal element.
    Origin = 1,
    /// The position continues an earlier terminal element.
    Continuation = 2,
    /// Reserved for future extension.
    Reserved = 3,
}
impl TermelOccupancy {
    /// Creates an occupancy kind from its packed representation.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Unoccupied,
            1 => Self::Origin,
            2 => Self::Continuation,
            _ => Self::Reserved,
        }
    }
}

crate::bitfield! {
    #[doc = crate::_tags!(term text layout bit)]
    /// Compact terminal-cell occupancy metadata for a [`Termel`].
    #[doc = crate::_doc_meta!{
        location("sys/os/term/grid"),
        test_size_of(TermelMeta = 1),
    }]
    ///
    /// An origin stores its occupied column width.
    /// A continuation stores its backward offset to the origin.
    ///
    /// Widths and offsets range from 1 to 15 columns.
    #[must_use]
    pub struct TermelMeta(u8) {
        /// The [`TermelOccupancy`] discriminant.
        KIND = 0..=1;
        /// The origin width or continuation offset.
        VALUE = 2..=5;
    }
    impl {
        /// The maximum representable width or backward offset.
        pub const MAX_SPAN: u8 = 15;

        /// Creates metadata for an element origin.
        ///
        /// Returns `None` when `width` is zero or exceeds [`Self::MAX_SPAN`].
        #[must_use]
        pub const fn origin(width: u8) -> Option<Self> {
            if width == 0 || width > Self::MAX_SPAN { None }
            else { Some( Self::new().with_kind(Origin as u8).with_value(width)) }
        }
        /// Creates metadata for an element continuation.
        ///
        /// `origin_offset` is the number of columns back to the origin.
        ///
        /// Returns `None` when it is zero or exceeds [`Self::MAX_SPAN`].
        #[must_use]
        pub const fn continuation(origin_offset: u8) -> Option<Self> {
            if origin_offset == 0 || origin_offset > Self::MAX_SPAN { None }
            else { Some( Self::new().with_kind(Continuation as u8).with_value(origin_offset)) }
        }
        /// Returns the occupancy kind.
        #[must_use]
        pub const fn occupancy(self) -> TermelOccupancy {
            TermelOccupancy::from_u8(self.get_kind())
        }

        /// Returns whether this position is occupied.
        pub const fn is_occupied(self) -> bool {
            matches!( self.occupancy(), Origin | Continuation)
        }
        /// Returns whether this position is empty.
        #[must_use]
        pub const fn is_unoccupied(self) -> bool { matches!(self.occupancy(), Unoccupied) }

        /// Returns whether this position begins an element.
        #[must_use]
        pub const fn is_origin(self) -> bool { matches!(self.occupancy(), Origin) }
        /// Returns whether this position continues an earlier element.
        #[must_use]
        pub const fn is_continuation(self) -> bool { matches!(self.occupancy(), Continuation) }

        /// Returns the occupied width when this is an origin.
        #[must_use]
        pub const fn width(self) -> Option<u8> {
            if self.is_origin() { Some(self.get_value()) } else { None }
        }
        /// Returns the backward distance to the origin when this is a continuation.
        #[must_use]
        pub const fn origin_offset(self) -> Option<u8> {
            if self.is_continuation() { Some(self.get_value()) } else { None }
        }
        /// Returns whether this value has a valid canonical representation.
        #[must_use]
        pub const fn is_canonical(self) -> bool {
            if self.bits() & !Self::mask() != 0 { return false; }
            match self.occupancy() {
                Unoccupied => self.get_value() == 0,
                Origin | Continuation => { self.get_value() != 0 }
                Reserved => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(size_of::<TermelMeta>(), 1);
    }
    #[test]
    fn empty() {
        let meta = TermelMeta::new();
        assert!(meta.is_empty());
        assert_eq!(meta.width(), None);
        assert_eq!(meta.origin_offset(), None);
        assert!(meta.is_canonical());
    }
    #[test]
    fn origin() {
        let meta = TermelMeta::origin(4).unwrap();
        assert!(meta.is_origin());
        assert_eq!(meta.width(), Some(4));
        assert_eq!(meta.origin_offset(), None);
        assert!(meta.is_canonical());
    }
    #[test]
    fn continuation() {
        let meta = TermelMeta::continuation(3).unwrap();
        assert!(meta.is_continuation());
        assert_eq!(meta.width(), None);
        assert_eq!(meta.origin_offset(), Some(3));
        assert!(meta.is_canonical());
    }
    #[test]
    fn span_limits() {
        assert_eq!(TermelMeta::origin(0), None);
        assert_eq!(TermelMeta::origin(16), None);
        assert_eq!(TermelMeta::continuation(0), None);
        assert_eq!(TermelMeta::continuation(16), None);
        assert_eq!(TermelMeta::origin(TermelMeta::MAX_SPAN).unwrap().width(), Some(15),);
    }
    #[test]
    fn occupied_sequence() {
        let cells = [
            TermelMeta::origin(4).unwrap(),
            TermelMeta::continuation(1).unwrap(),
            TermelMeta::continuation(2).unwrap(),
            TermelMeta::continuation(3).unwrap(),
        ];
        assert_eq!(cells[0].width(), Some(4));
        assert_eq!(cells[1].origin_offset(), Some(1));
        assert_eq!(cells[2].origin_offset(), Some(2));
        assert_eq!(cells[3].origin_offset(), Some(3));
    }
}
