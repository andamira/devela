// devela::vita::play::game::legacy

#[doc = crate::_tags!(game)]
/// Persistent continuity that carries across multiple sessions of play.
#[doc = crate::_doc_location!("vita/play/game")]
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
