// devela/src/vita/play/game/action.rs

#[doc = crate::_tags!(game)]
/// A declared move or operation recognized by the rules of play.
#[doc = crate::_doc_meta!{location("vita/play/game")}]
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
