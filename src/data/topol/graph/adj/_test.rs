// devela/src/data/topol/graph/adj/_test.rs

use crate::{
    GraphAdjEdgeExample as Edge, GraphAdjExample as Graph, GraphAdjVertexExample as Vertex,
};
use crate::{const_assert, unwrap};

#[test]
fn empty_and_capacity() {
    let graph = Graph::<3, 4>::new();
    assert_eq!(graph.vertex_count(), 3);
    assert_eq!(graph.edge_count(), 0);
    assert_eq!(graph.edge_capacity(), 4);
    assert_eq!(graph.edge_remaining(), 4);
    assert!(graph.is_empty());
    assert!(!graph.is_full());
}
#[test]
fn add_and_query_edge() {
    let mut graph = Graph::<3, 4>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let edge = graph.add_edge(a, b).unwrap();
    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.edge_target(edge), Some(b));
    assert_eq!(graph.first_out_edge(a), Some(edge));
    assert!(graph.has_edge(a, b));
    assert!(!graph.has_edge(b, a));
}
#[test]
fn outgoing_edges_are_reverse_insertion_order() {
    let mut graph = Graph::<4, 4>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let c = graph.vertex(2).unwrap();
    let d = graph.vertex(3).unwrap();
    let ab = graph.add_edge(a, b).unwrap();
    let ac = graph.add_edge(a, c).unwrap();
    let ad = graph.add_edge(a, d).unwrap();
    assert_eq!(graph.first_out_edge(a), Some(ad));
    assert_eq!(graph.next_out_edge(ad), Some(ac));
    assert_eq!(graph.next_out_edge(ac), Some(ab));
    assert_eq!(graph.next_out_edge(ab), None);
}
#[test]
fn handle_bounds() {
    let mut graph = Graph::<3, 2>::new();
    assert!(graph.vertex(0).is_some());
    assert!(graph.vertex(2).is_some());
    assert!(graph.vertex(3).is_none());
    let outside = Vertex::try_from_usize(3).unwrap();
    let a = graph.vertex(0).unwrap();
    assert!(!graph.contains_vertex(outside));
    assert!(graph.add_edge(a, outside).is_none());
    assert!(graph.add_edge(outside, a).is_none());
    let future = Edge::try_from_usize(0).unwrap();
    assert!(!graph.contains_edge(future));
    assert_eq!(graph.edge(0), None);
    let edge = graph.add_edge(a, a).unwrap();
    assert_eq!(graph.edge(0), Some(edge));
    assert!(graph.contains_edge(edge));
    assert_eq!(graph.edge(1), None);
}
#[test]
fn edge_capacity_is_enforced() {
    let mut graph = Graph::<2, 1>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let ab = graph.add_edge(a, b).unwrap();
    assert!(graph.is_full());
    assert_eq!(graph.edge_remaining(), 0);
    assert_eq!(graph.add_edge(b, a), None);
    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.first_out_edge(a), Some(ab));
    assert_eq!(graph.first_out_edge(b), None);
}
#[test]
fn loops_parallel_edges_and_cycles_are_allowed() {
    let mut graph = Graph::<2, 4>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let aa = graph.add_edge(a, a).unwrap(); // self-loop
    let ab1 = graph.add_edge(a, b).unwrap();
    let ab2 = graph.add_edge(a, b).unwrap(); // parallel edge
    let ba = graph.add_edge(b, a).unwrap(); // cycle
    assert!(graph.has_edge(a, a));
    assert!(graph.has_edge(a, b));
    assert!(graph.has_edge(b, a));
    assert_eq!(graph.out_degree(a), Some(3));
    assert_eq!(graph.out_degree(b), Some(1));
    assert_eq!(graph.first_out_edge(a), Some(ab2));
    assert_eq!(graph.next_out_edge(ab2), Some(ab1));
    assert_eq!(graph.next_out_edge(ab1), Some(aa));
    assert_eq!(graph.next_out_edge(aa), None);
    assert_eq!(graph.first_out_edge(b), Some(ba));
}
#[test]
fn outgoing_iterators_follow_adjacency_order() {
    let mut graph = Graph::<4, 4>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let c = graph.vertex(2).unwrap();
    let d = graph.vertex(3).unwrap();
    let ab = graph.add_edge(a, b).unwrap();
    let ac = graph.add_edge(a, c).unwrap();
    let ad = graph.add_edge(a, d).unwrap();
    let mut edges = graph.out_edges(a);
    assert_eq!(edges.next(), Some((ad, d)));
    assert_eq!(edges.next(), Some((ac, c)));
    assert_eq!(edges.next(), Some((ab, b)));
    assert_eq!(edges.next(), None);
    let mut neighbors = graph.neighbors(a);
    assert_eq!(neighbors.next(), Some(d));
    assert_eq!(neighbors.next(), Some(c));
    assert_eq!(neighbors.next(), Some(b));
    assert_eq!(neighbors.next(), None);
}
#[test]
fn clear_preserves_domain_and_reuses_edge_indices() {
    let mut graph = Graph::<2, 2>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let old = graph.add_edge(a, b).unwrap();
    graph.clear();
    assert_eq!(graph.vertex_count(), 2);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.contains_vertex(a));
    assert!(graph.contains_vertex(b));
    assert!(!graph.contains_edge(old));
    assert_eq!(graph.edge_target(old), None);
    assert_eq!(graph.first_out_edge(a), None);
    let new = graph.add_edge(b, a).unwrap();
    // Handles are contextual; clearing does not permanently retire an index.
    assert_eq!(old, new);
    assert_eq!(graph.edge_target(old), Some(a));
}
#[test]
fn zero_sized_graph() {
    let graph = Graph::<0, 0>::new();
    assert_eq!(graph.vertex_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.is_empty());
    assert!(graph.is_full());
    assert_eq!(graph.vertex(0), None);
    assert_eq!(graph.edge(0), None);
}
#[test]
const fn static_core_is_const_capable() {
    const RESULT: (usize, bool, usize, u8) = {
        let mut graph = Graph::<3, 3>::new();
        let a = unwrap![some graph.vertex(0)];
        let b = unwrap![some graph.vertex(1)];
        let edge = unwrap![some graph.add_edge(a, b)];
        let target = unwrap![some_map_into graph.edge_target(edge), |v| v.index_prim()];
        let degree = unwrap![some graph.out_degree(a)];
        (graph.edge_count(), graph.has_edge(a, b), degree, target)
    };
    const_assert!(eq RESULT.0, 1);
    const_assert!(eq RESULT.1, true);
    const_assert!(eq RESULT.2, 1);
    const_assert!(eq RESULT.3, 1);
}
#[test]
fn omitted_representations_and_explicit_static_work() {
    crate::graph_adj! {
        [
            vertex: u8;
            edge: u8;
        ]
        GraphAdjPlainTest: static;
        GraphAdjPlainVertexTest;
        GraphAdjPlainEdgeTest;
    }
    let mut graph = GraphAdjPlainTest::<2, 2>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    assert!(graph.add_edge(a, b).is_some());
    assert!(graph.has_edge(a, b));
}
#[test]
fn incoming_reachability_and_acyclicity() {
    let mut graph = Graph::<5, 6>::new();
    let a = graph.vertex(0).unwrap();
    let b = graph.vertex(1).unwrap();
    let c = graph.vertex(2).unwrap();
    let d = graph.vertex(3).unwrap();
    let e = graph.vertex(4).unwrap();
    graph.add_edge(a, b).unwrap();
    graph.add_edge(a, c).unwrap();
    graph.add_edge(b, d).unwrap();
    graph.add_edge(c, d).unwrap();
    graph.add_edge(d, e).unwrap();
    assert_eq!(graph.in_degree(a), Some(0));
    assert_eq!(graph.in_degree(d), Some(2));
    assert_eq!(graph.in_degree(e), Some(1));
    assert!(graph.is_reachable(a, e));
    assert!(graph.is_reachable(a, a));
    assert!(!graph.is_reachable(e, a));
    let mut reach = [None; 5];
    assert_eq!(graph.is_reachable_in(a, e, &mut reach), Some(true));
    let mut too_small = [None; 4];
    assert_eq!(graph.is_reachable_in(a, e, &mut too_small), None);
    assert!(graph.is_acyclic());
    let mut degrees = [None; 5];
    assert_eq!(graph.is_acyclic_in(&mut degrees), Some(true));
    graph.add_edge(e, b).unwrap();
    assert!(!graph.is_acyclic());
}

/**
```compile_fail,E0080
# use devela::GraphAdjExample as Graph;
let _ = Graph::<256, 0>::new();
```
```compile_fail,E0080
# use devela::GraphAdjExample as Graph;
let _ = Graph::<0, 65536>::new();
```
**/
#[allow(dead_code)]
fn static_rejects_unrepresentable_domains() {}

#[cfg(feature = "alloc")]
mod alloc {
    use crate::{GraphAdjAllocExample as Graph, Vec};

    #[test]
    fn grows_edges() {
        let mut graph = Graph::new(3);
        let a = graph.vertex(0).unwrap();
        let b = graph.vertex(1).unwrap();
        for _ in 0..100 {
            assert!(graph.add_edge(a, b).is_some());
        }
        assert_eq!(graph.edge_count(), 100);
        assert!(graph.edge_capacity() >= 100);
        assert_eq!(graph.out_degree(a), Some(100));
        assert!(!graph.is_full());
    }
    #[test]
    fn with_capacity_reserves_edges() {
        let graph = Graph::with_capacity(3, 8);
        assert_eq!(graph.vertex_count(), 3);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.edge_capacity() >= 8);
        assert!(graph.edge_remaining() >= 8);
        assert!(!graph.is_full());
    }
    #[test]
    #[should_panic(expected = "vertex count exceeds")]
    fn rejects_unrepresentable_vertex_count() {
        let _ = Graph::new(256);
    }
    #[test]
    #[should_panic(expected = "edge capacity exceeds")]
    fn rejects_unrepresentable_edge_capacity() {
        let _ = Graph::with_capacity(0, 65_536);
    }
    #[test]
    fn basic_topology() {
        let mut graph = Graph::new(3);
        let a = graph.vertex(0).unwrap();
        let b = graph.vertex(1).unwrap();
        let c = graph.vertex(2).unwrap();
        let ab = graph.add_edge(a, b).unwrap();
        let ac = graph.add_edge(a, c).unwrap();
        assert_eq!(graph.first_out_edge(a), Some(ac));
        assert_eq!(graph.next_out_edge(ac), Some(ab));
        assert!(graph.has_edge(a, b));
        assert_eq!(graph.out_degree(a), Some(2));
        assert_eq!(graph.neighbors(a).collect::<Vec<_>>(), [c, b]);
        graph.clear();
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.first_out_edge(a).is_none());
    }
}
