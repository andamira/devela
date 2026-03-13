// devela_base_core::vita::play::game::cycle

/// A repeated structural grouping of turns, phases, or recurrent play segments.
///
/// Represents one recurring loop in the formal organization of play.
///
/// A cycle groups repeated play structure within a session.
///
/// ## Uses
/// Combat rounds, initiative rounds, repeated day/night loops, trick cycles,
/// seasonal or stage loops, repeated resolution loops…
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameCycle<Ts> {
    /// The turns grouped by this cycle.
    pub turns: Ts,
}
