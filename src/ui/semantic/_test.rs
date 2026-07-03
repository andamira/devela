// devela/src/ui/_test.rs

use super::*;
use crate::UiId;

#[test]
fn actions_default_to_empty() {
    let actions = UiActions::default();
    assert!(actions.is_empty());
    assert!(!actions.has(UiActions::Activate));
}
#[test]
fn actions_can_include_and_remove_members() {
    let actions = UiActions::new().with(UiActions::Activate).with(UiActions::Focus);
    assert!(actions.has(UiActions::Activate));
    assert!(actions.has(UiActions::Focus));
    assert!(!actions.has(UiActions::Toggle));
    let actions = actions.without(UiActions::Focus);
    assert!(actions.has(UiActions::Activate));
    assert!(!actions.has(UiActions::Focus));
}
#[test]
fn flags_can_include_and_remove_states() {
    let flags = UiFlags::new().with(UiFlags::DISABLED).with(UiFlags::REQUIRED);
    assert!(flags.has(UiFlags::DISABLED));
    assert!(flags.has(UiFlags::REQUIRED));
    assert!(!flags.has(UiFlags::CHECKED));
    let flags = flags.without(UiFlags::DISABLED);
    assert!(!flags.has(UiFlags::DISABLED));
    assert!(flags.has(UiFlags::REQUIRED));
}
#[test]
fn entry_new_starts_with_empty_actions_and_flags() {
    let id = UiId::new(7);
    let entry = UiEntry::new(id, UiRole::Button);
    assert_eq!(entry.id(), id);
    assert_eq!(entry.role(), UiRole::Button);
    assert!(entry.actions().is_empty());
    assert!(entry.flags().is_empty());
}
#[test]
fn entry_builders_add_actions_and_flags() {
    let id = UiId::new(8);
    let entry = UiEntry::new(id, UiRole::Button)
        .with_action(UiAction::Activate)
        .with_flags(UiFlags::DISABLED);
    assert!(entry.actions().has(UiActions::Activate));
    assert!(entry.flags().has(UiFlags::DISABLED));
}
#[test]
fn text_can_store_label_and_description() {
    let id = UiId::new(9);
    let text = UiText::new(id).with_label("Save").with_description("Save the current document");
    assert_eq!(text.id(), id);
    assert_eq!(text.label(), Some("Save"));
    assert_eq!(text.description(), Some("Save the current document"));
}
