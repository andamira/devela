// devela/src/data/topol/link/_test.rs

use crate::LinkExample;

const CONST_LINKS: LinkExample = {
    let mut links = LinkExample::new();
    let _ = links.set_next_prim(7);
    let _ = links.set_parent_prim(300);
    links
};

#[test]
fn link_const_operations() {
    assert_eq!(CONST_LINKS.get_next_prim(), Some(7));
    assert_eq!(CONST_LINKS.get_prev_prim(), None);
    assert_eq!(CONST_LINKS.get_parent_prim(), Some(300));
}
#[test]
fn link_components() {
    let links = LinkExample::from_prim(Some(7), None, Some(300)).unwrap();
    assert_eq!(links.get_next_prim(), Some(7));
    assert_eq!(links.get_prev_prim(), None);
    assert_eq!(links.get_parent_prim(), Some(300));
    assert_eq!(links.into_prim(), (Some(7), None, Some(300)),);
}
#[test]
fn link_mutation() {
    let mut links = LinkExample::new();
    assert!(links.is_empty());
    assert_eq!(links.set_next_prim(7), Ok(None));
    assert_eq!(links.get_next_prim(), Some(7));
    assert!(links.has_next());
    assert_eq!(links.set_next_prim(9).unwrap().unwrap().get(), 7);
    assert_eq!(links.get_next_prim(), Some(9));
    let previous = links.clear_next().unwrap();
    assert_eq!(previous.get(), 9);
    assert!(!links.has_next());
    assert!(links.is_empty());
}
#[test]
fn link_rejects_invalid_niche_value_without_mutating() {
    let mut links = LinkExample::from_prim(Some(7), None, None).unwrap();
    assert!(links.set_next_prim(u8::MAX).is_err());
    assert_eq!(links.get_next_prim(), Some(7));
}
