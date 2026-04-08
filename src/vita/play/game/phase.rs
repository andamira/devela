// devela::vita::play::game::phase

/// A named subdivision within a turn or cycle of play.
///
/// Represents an internal structural segment with its own timing, permissions, or procedures.
///
/// A phase structures when actions or resolutions may occur inside a turn or cycle.
///
/// ## Uses
/// Upkeep, draw, planning, movement, combat, resolution, cleanup, reveal…
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GamePhase<K> {
    /// The phase kind or payload.
    pub kind: K,
}
