// devela_base_core::vita::play::game::outcome

/// A resolved result recognized by the rules of play.
///
/// Represents what the system acknowledges as having resulted from an action, turn, cycle, or session.
///
/// Outcomes are produced by resolution and may modify session or legacy continuity.
///
/// ## Uses
/// Success / failure, win / loss / draw, damage dealt, resource gained,
/// unit defeated, objective completed, route ended…
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameOutcome<K> {
    /// The outcome kind or payload.
    pub kind: K,
}
