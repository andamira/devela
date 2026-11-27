use crate::{NotEnoughSpace, Search, SearchNode};

// A cyclic graph with a duplicate edge, two shortest routes, and a longer route.
fn successors(state: &u8) -> impl Iterator<Item = (u8, char)> + use<> {
    let edges: &'static [(u8, char)] = match *state {
        0 => &[(0, 'x'), (1, 'a'), (1, 'z'), (2, 'b')],
        1 => &[(0, 'y'), (4, 'd'), (3, 'c')],
        2 => &[(3, 'e')],
        4 => &[(3, 'f')],
        _ => &[],
    };
    edges.iter().copied()
}

#[test]
fn shortest_path_and_tie_order() {
    let mut nodes = [None; 5];
    assert_eq!(Search::bfs(0, |&s| s == 3, successors, &mut nodes), Ok(Some(4)));
    assert_eq!(
        nodes,
        [
            Some(SearchNode { state: 0, via: None }),
            Some(SearchNode { state: 1, via: Some((0, 'a')) }),
            Some(SearchNode { state: 2, via: Some((0, 'b')) }),
            Some(SearchNode { state: 4, via: Some((1, 'd')) }),
            Some(SearchNode { state: 3, via: Some((1, 'c')) }),
        ]
    );
}
#[test]
fn exhausted_and_reused() {
    let mut nodes = [None; 6];
    assert_eq!(Search::bfs(0, |_| false, successors, &mut nodes), Ok(None));
    assert_eq!(nodes.iter().flatten().count(), 5);
    assert_eq!(Search::bfs(3, |_| true, successors, &mut nodes), Ok(Some(0)));
    assert_eq!(nodes[0], Some(SearchNode { state: 3, via: None }));
    assert!(nodes[1..].iter().all(Option::is_none));
}
#[test]
fn insufficient_space() {
    let mut nodes = [None; 2];
    assert_eq!(Search::bfs(0, |&s| s == 3, successors, &mut nodes), Err(NotEnoughSpace(Some(1))));
    assert_eq!(nodes[1], Some(SearchNode { state: 1, via: Some((0, 'a')) }));
    assert_eq!(Search::bfs(0, |_| true, successors, &mut []), Err(NotEnoughSpace(Some(1))));
}
#[test]
fn full_storage_can_still_exhaust_or_find_goal() {
    let mut nodes = [None; 1];
    assert_eq!(Search::bfs(3, |_| false, successors, &mut nodes), Ok(None));
    assert_eq!(Search::bfs(0, |_| true, successors, &mut nodes), Ok(Some(0)));
    let mut nodes = [None; 5];
    assert_eq!(Search::bfs(0, |_| false, successors, &mut nodes), Ok(None));
}
