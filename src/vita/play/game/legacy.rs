// devela::vita::play::game::legacy

/// Persistent continuity that carries across multiple sessions of play.
///
/// Represents what remains consequential beyond any single session.
///
/// A legacy preserves or accumulates consequences across sessions.
///
/// ## Uses
/// Campaign continuity, persistent characters, world progression,
/// metaprogression, unlocks, linked scenarios, dynasty or profile continuity…
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameLegacy<Ss> {
    /// The sessions linked by this legacy.
    pub sessions: Ss,
}
