// devela/ui/frame/tests.rs

use super::*;

#[test]
fn key_and_id_preserve_raw_values_except_max() {
    assert_eq!(UiKey::new(0).raw(), 0);
    assert_eq!(UiId::new(0).raw(), 0);
    assert_eq!(UiKey::new(u64::MAX).raw(), u64::MAX - 1);
    assert_eq!(UiId::new(u64::MAX).raw(), u64::MAX - 1);
}
#[test]
fn default_identity_values_are_root() {
    assert_eq!(UiKey::default(), UiKey::ROOT);
    assert_eq!(UiId::default(), UiId::ROOT);
    assert_eq!(UiScope::default(), UiScope::ROOT);
}
#[test]
fn resolving_key_is_deterministic() {
    let scope = UiScope::ROOT;
    let key = UiKey::new(42);
    assert_eq!(scope.resolve(key), scope.resolve(key));
}
#[test]
fn entering_scope_matches_resolved_id() {
    let scope = UiScope::ROOT;
    let key = UiKey::new(7);
    assert_eq!(scope.enter(key).id(), scope.resolve(key));
}
#[test]
fn resolving_root_key_inside_root_scope_derives_child_id() {
    assert_ne!(UiScope::ROOT.resolve(UiKey::ROOT), UiId::ROOT);
}
#[test]
fn frame_resolves_ids_through_its_scope() {
    let frame = UiFrame::new();
    let key = UiKey::new(13);
    assert_eq!(frame.id(key), frame.scope().resolve(key));
}
#[test]
fn frame_enter_updates_scope() {
    let frame = UiFrame::new();
    let key = UiKey::new(5);
    let entered = frame.enter(key);
    assert_eq!(entered.scope(), frame.scope().enter(key));
}
#[test]
fn frame_phase_can_be_changed() {
    let frame = UiFrame::new().with_phase(UiPhase::Layout);
    assert_eq!(frame.phase(), UiPhase::Layout);
}
