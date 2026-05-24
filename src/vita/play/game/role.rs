// devela::vita::play::game::role

#[doc = crate::_tags!(game)]
/// A rule-bearing identity or seat in play.
#[doc = crate::_doc_location!("vita/play/game")]
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
