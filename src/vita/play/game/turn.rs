// devela::vita::play::game::turn

/// A bounded opportunity for one role or control locus to act.
///
/// Provides a rule-defined acting window.
///
/// A turn may contain actions and phases, and may belong to a cycle.
///
/// ## Uses
/// One player's turn, one unit's turn, one side's turn, initiative turns,
/// structured alternating control…
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameTurn<R, As> {
    /// The acting role for this turn.
    pub role: R,

    /// The actions declared within this turn.
    pub actions: As,
}
