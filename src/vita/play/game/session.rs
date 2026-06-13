// devela/src/vita/play/game/session.rs

#[doc = crate::_tags!(game)]
/// A bounded instance of enacted play.
#[doc = crate::_doc_meta!{location("vita/play/game")}]
///
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
