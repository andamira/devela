// devela/src/geom/space/topol/turn.rs

#[doc = crate::_tags!(geom topol)]
/// The planar turn determined by three ordered points.
#[doc = crate::_doc_meta!{
    location("geom/space", enum Turn),
    test_size_of(Turn = 1|8; niche Option)
}]
/// Given the ordered points `a`, `b`, and `c`, this classifies the sign of:
///
/// ```text
/// (b - a) × (c - a)
/// ```
///
/// - [`Left`](Self::Left): `c` lies to the left of the directed line `a → b`.
/// - [`Right`](Self::Right): `c` lies to the right of the directed line `a → b`.
/// - [`Collinear`](Self::Collinear): the three points lie on one line.
///
/// Equivalently, this describes whether the ordered points form a
/// counterclockwise turn, a clockwise turn, or no turn.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Turn {
    /// A right, or clockwise, turn.
    Right = -1,

    /// No turn; the three points are collinear.
    #[default]
    Collinear = 0,

    /// A left, or counterclockwise, turn.
    Left = 1,
}
#[allow(missing_docs, non_upper_case_globals)]
impl Turn {
    /// Alias of a left turn.
    pub const CounterClockwise: Self = Self::Left;
    pub const CCW: Self = Self::Left;

    /// Alias of a right turn.
    pub const Clockwise: Self = Self::Right;
    pub const CW: Self = Self::Right;
}
impl Turn {
    /// Returns the opposite turn.
    pub const fn reversed(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Collinear => Self::Collinear,
        }
    }

    /// Whether this is a left turn.
    pub const fn is_left(self) -> bool {
        matches![self, Self::Left]
    }
    /// Whether this is a right turn.
    pub const fn is_right(self) -> bool {
        matches![self, Self::Right]
    }
    /// Whether this represents a collinear relation.
    pub const fn is_collinear(self) -> bool {
        matches![self, Self::Collinear]
    }
}

#[cfg(test)]
mod _test {
    use super::Turn;

    const _: () = {
        assert!(Turn::Left.reversed().is_right());
        assert!(Turn::Right.reversed().is_left());
        assert!(Turn::Collinear.reversed().is_collinear());
        assert!(Turn::Left.is_left());
        assert!(!Turn::Left.is_right());
        assert!(!Turn::Left.is_collinear());
        assert!(Turn::Right.is_right());
        assert!(!Turn::Right.is_left());
        assert!(!Turn::Right.is_collinear());
        assert!(Turn::Collinear.is_collinear());
        assert!(!Turn::Collinear.is_left());
        assert!(!Turn::Collinear.is_right());
    };
    #[test]
    fn turn_contract() {
        assert_eq!(Turn::default(), Turn::Collinear);
        assert_eq!(Turn::Right as i8, -1);
        assert_eq!(Turn::Collinear as i8, 0);
        assert_eq!(Turn::Left as i8, 1);
        assert_eq!(Turn::CounterClockwise, Turn::Left);
        assert_eq!(Turn::CCW, Turn::Left);
        assert_eq!(Turn::Clockwise, Turn::Right);
        assert_eq!(Turn::CW, Turn::Right);
        for turn in [Turn::Right, Turn::Collinear, Turn::Left] {
            assert_eq!(turn.reversed().reversed(), turn);
        }
    }
}
