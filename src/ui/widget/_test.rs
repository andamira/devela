// devela/src/ui/widget/_test.rs

use super::*;
use crate::{UiAction, UiFrame, UiKey, UiRole};

#[test]
fn response_new_is_empty() {
    let id = crate::UiId::new(7);
    let response = UiResponse::new(id);

    assert_eq!(response.id(), id);
    assert!(response.is_empty());
    assert!(!response.is_activated());
}

#[test]
fn response_flags_mark_activation() {
    let response = UiResponse::new(crate::UiId::new(8)).focused().activate();

    assert!(response.is_focused());
    assert!(response.is_activated());
    assert!(!response.is_changed());
}

#[test]
fn button_emits_semantic_entry_and_text() {
    let frame = UiFrame::new();
    let button = UiButton::new(UiKey::new(9), "Save").with_description("Save the current document");
    let id = frame.id(button.key());

    let entry = button.entry(id);
    let text = button.text(id);

    assert_eq!(entry.id(), id);
    assert_eq!(entry.role(), UiRole::Button);
    assert!(entry.actions().has(UiAction::Activate.to_set()));

    assert_eq!(text.id(), id);
    assert_eq!(text.label(), Some("Save"));
    assert_eq!(text.description(), Some("Save the current document"));
}
