// devela::vita::play::game::turn

/// A bounded instance of enacted play.
/// Represents one occurrence of play as a whole.
///
/// A session contains the active structure of play and may contribute to a legacy.
///
/// ## Uses
/// One sitting, one match, one run, one playthrough, one active game instance…
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameSession<Rs, As, Os> {
    /// The participating roles in the session.
    pub roles: Rs,

    /// The actions declared in the session.
    pub actions: As,

    /// The outcomes resolved in the session.
    pub outcomes: Os,
}
