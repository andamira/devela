// devela/src/ui/view/tests.rs

use super::*;
use crate::{UiId, UiRect};

#[test]
fn layer_base_is_zero() {
    assert_eq!(UiLayer::BASE.raw(), 0);
    assert_eq!(UiLayer::new(3).above().raw(), 4);
    assert_eq!(UiLayer::new(3).below().raw(), 2);
}
#[test]
fn view_new_uses_base_layer_and_empty_flags() {
    let id = UiId::new(7);
    let rect = UiRect::default();
    let view = UiView::new(id, rect);
    assert_eq!(view.id(), id);
    assert_eq!(view.rect(), rect);
    assert_eq!(view.layer(), UiLayer::BASE);
    assert!(view.flags().is_empty());
    assert!(view.is_visible());
    assert!(!view.is_hidden());
}
#[test]
fn view_flags_control_visibility() {
    let view = UiView::new(UiId::new(8), UiRect::default()).with_flags(UiViewFlags::HIDDEN);
    assert!(view.is_hidden());
    assert!(!view.is_visible());
    let view = view.without_flags(UiViewFlags::HIDDEN);
    assert!(!view.is_hidden());
    assert!(view.is_visible());
}
#[test]
fn view_layer_can_be_changed() {
    let view = UiView::new(UiId::new(9), UiRect::default()).with_layer(UiLayer::new(4));
    assert_eq!(view.layer().raw(), 4);
}
