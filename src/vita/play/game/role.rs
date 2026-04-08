// devela::vita::play::game::role

/// A rule-bearing identity or seat in play.
///
/// Defines who or what occupies a structured position in the game system.
///
/// A role constrains or enables actions and responsibilities.
///
/// ## Uses
/// Player, side, team, faction, dealer, referee…
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GameRole<K> {
    /// The role kind or payload.
    pub kind: K,
}
