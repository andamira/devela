// devela/src/ui/route/_test.rs

use super::*;
use crate::UiId;

#[test]
fn hot_state_can_be_empty_or_set() {
    let id = UiId::new(7);
    let hot = RouteHot::NONE;
    assert!(hot.is_none());
    assert!(!hot.is(id));
    let hot = RouteHot::some(id);
    assert!(hot.is_some());
    assert!(hot.is(id));
}
#[test]
fn active_focus_and_capture_store_identity() {
    let id = UiId::new(9);
    assert_eq!(RouteActive::some(id).id(), Some(id));
    assert_eq!(RouteFocus::some(id).id(), Some(id));
    assert_eq!(RouteCapture::some(id).id(), Some(id));
}
