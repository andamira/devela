// devela::vita::play::game::action

/// A declared move or operation recognized by the rules of play.
///
/// Represents what an actor attempts or selects within the formal game.
///
/// An action is attempted in play and may resolve into one or more outcomes.
///
/// ## Uses
/// Move, attack, defend, pass, bid, draw, play card, choose branch, end turn…
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameAction<K> {
    /// The action kind or payload.
    pub kind: K,
}
