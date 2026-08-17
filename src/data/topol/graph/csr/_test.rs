// devela/src/data/topol/graph/csr/_test.rs

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{
    GraphCsrEdgeExample as Edge, GraphCsrExample as Graph, GraphCsrVertexExample as Vertex,
};

fn vertex(index: usize) -> Vertex {
    Vertex::try_from_usize(index).unwrap()
}
fn edge(index: usize) -> Edge {
    Edge::try_from_usize(index).unwrap()
}
fn graph() -> Graph<4, 4> {
    // 0 → 1, 2
    // 1 → ∅
    // 2 → 2, 3
    // 3 → ∅
    Graph::from_parts(
        [
            Some(edge(0)),
            Some(edge(2)),
            Some(edge(2)),
            None, // offset EDGES == 4
        ],
        [vertex(1), vertex(2), vertex(2), vertex(3)],
    )
    .unwrap()
}

#[test]
fn size_and_handles() {
    let graph = graph();
    assert_eq!(graph.vertex_count(), 4);
    assert_eq!(graph.edge_count(), 4);
    assert!(!graph.is_empty());
    assert_eq!(graph.vertex(0), Some(vertex(0)));
    assert_eq!(graph.vertex(3), Some(vertex(3)));
    assert_eq!(graph.vertex(4), None);
    assert_eq!(graph.edge(0), Some(edge(0)));
    assert_eq!(graph.edge(3), Some(edge(3)));
    assert_eq!(graph.edge(4), None);
}
#[test]
fn row_bounds_include_empty_rows() {
    let graph = graph();
    assert_eq!(graph.out_edge_bounds(vertex(0)), Some((0, 2)));
    assert_eq!(graph.out_edge_bounds(vertex(1)), Some((2, 2)));
    assert_eq!(graph.out_edge_bounds(vertex(2)), Some((2, 4)));
    assert_eq!(graph.out_edge_bounds(vertex(3)), Some((4, 4)));
}
#[test]
fn edge_targets() {
    let graph = graph();
    assert_eq!(graph.edge_target(edge(0)), Some(vertex(1)));
    assert_eq!(graph.edge_target(edge(1)), Some(vertex(2)));
    assert_eq!(graph.edge_target(edge(2)), Some(vertex(2)));
    assert_eq!(graph.edge_target(edge(3)), Some(vertex(3)));
}
#[test]
fn queries() {
    let graph = graph();
    assert!(graph.has_edge(vertex(0), vertex(1)));
    assert!(graph.has_edge(vertex(0), vertex(2)));
    assert!(!graph.has_edge(vertex(1), vertex(2)));
    assert!(graph.has_edge(vertex(2), vertex(2))); // self-loop
    assert!(graph.has_edge(vertex(2), vertex(3)));
    assert_eq!(graph.out_degree(vertex(0)), Some(2));
    assert_eq!(graph.out_degree(vertex(1)), Some(0));
    assert_eq!(graph.out_degree(vertex(2)), Some(2));
    assert_eq!(graph.out_degree(vertex(3)), Some(0));
}
#[test]
#[cfg(feature = "alloc")]
fn iterators_follow_packed_order() {
    let graph = graph();
    assert_eq!(
        graph.out_edges(vertex(0)).collect::<Vec<_>>(),
        [(edge(0), vertex(1)), (edge(1), vertex(2))]
    );
    assert_eq!(graph.neighbors(vertex(2)).collect::<Vec<_>>(), [vertex(2), vertex(3)]);
    assert_eq!(graph.neighbors(vertex(1)).collect::<Vec<_>>(), []);
}
#[test]
fn parts_roundtrip() {
    let graph = graph();
    let (starts, targets) = graph.clone().into_parts();
    let rebuilt = Graph::from_parts(starts, targets).unwrap();
    assert_eq!(graph.as_parts(), rebuilt.as_parts());
}
#[test]
fn rejects_nonzero_first_start() {
    let graph = Graph::<3, 2>::from_parts([Some(edge(1)), None, None], [vertex(1), vertex(2)]);
    assert!(graph.is_none());
}
#[test]
fn rejects_decreasing_starts() {
    let graph = Graph::<3, 3>::from_parts(
        [Some(edge(0)), Some(edge(2)), Some(edge(1))],
        [vertex(0), vertex(1), vertex(2)],
    );
    assert!(graph.is_none());
}
#[test]
fn rejects_noncanonical_end_start() {
    // EDGES == 2, so its one-past offset must be represented by None,
    // not by an Edge handle with numerical index 2.
    let graph = Graph::<3, 2>::from_parts(
        [Some(edge(0)), Edge::try_from_usize(2).ok(), None],
        [vertex(1), vertex(2)],
    );
    assert!(graph.is_none());
}
#[test]
fn rejects_invalid_target() {
    let outside = Vertex::try_from_usize(3).unwrap();
    let graph = Graph::<3, 1>::from_parts([Some(edge(0)), None, None], [outside]);
    assert!(graph.is_none());
}
#[test]
fn zero_graph_is_valid() {
    let graph = Graph::<0, 0>::from_parts([], []).unwrap();
    assert_eq!(graph.vertex_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert!(graph.is_empty());
}
#[test]
fn zero_edges_with_vertices_is_valid() {
    let graph = Graph::<3, 0>::from_parts([None, None, None], []).unwrap();
    assert_eq!(graph.out_degree(vertex(0)), Some(0));
    assert_eq!(graph.out_degree(vertex(1)), Some(0));
    assert_eq!(graph.out_degree(vertex(2)), Some(0));
}
#[test]
fn parallel_edges_are_allowed() {
    let graph = Graph::<2, 2>::from_parts([Some(edge(0)), None], [vertex(1), vertex(1)]).unwrap();
    assert!(graph.has_edge(vertex(0), vertex(1)));
    assert_eq!(graph.out_degree(vertex(0)), Some(2));
}

/**
```compile_fail,E0080
# use devela::GraphCsrExample as Graph;
let _ = Graph::<256, 0>::from_parts([None; 256], []);
```
```compile_fail,E0080
# use devela::{GraphCsrExample as Graph, GraphCsrVertexExample as Vertex};
let v = Vertex::try_from_usize(0).unwrap();
let _ = Graph::<1, 65_536>::from_parts(
    [None],
    [v; 65_536],
);
```
**/
#[allow(dead_code)]
fn static_rejects_unrepresentable_domains() {}

#[cfg(feature = "alloc")]
mod alloc {
    use crate::{
        GraphCsrAllocEdgeExample as Edge, GraphCsrAllocExample as Graph,
        GraphCsrAllocVertexExample as Vertex, Vec,
    };

    fn vertex(index: usize) -> Vertex {
        Vertex::try_from_usize(index).unwrap()
    }
    fn edge(index: usize) -> Edge {
        Edge::try_from_usize(index).unwrap()
    }
    fn graph() -> Graph {
        Graph::from_parts(
            Vec::from([Some(edge(0)), Some(edge(2)), Some(edge(2)), None]),
            Vec::from([vertex(1), vertex(2), vertex(2), vertex(3)]),
        )
        .unwrap()
    }

    #[test]
    fn basic_topology() {
        let graph = graph();
        assert_eq!(graph.vertex_count(), 4);
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.out_edge_bounds(vertex(0)), Some((0, 2)));
        assert_eq!(graph.out_edge_bounds(vertex(1)), Some((2, 2)));
        assert_eq!(graph.out_edge_bounds(vertex(2)), Some((2, 4)));
        assert_eq!(graph.out_edge_bounds(vertex(3)), Some((4, 4)));
        assert!(graph.has_edge(vertex(0), vertex(1)));
        assert!(graph.has_edge(vertex(2), vertex(2)));
        assert_eq!(graph.out_degree(vertex(1)), Some(0));
        assert_eq!(graph.neighbors(vertex(0)).collect::<Vec<_>>(), [vertex(1), vertex(2)]);
    }
    #[test]
    fn parts_roundtrip() {
        let graph = graph();
        let (starts, targets) = graph.clone().into_parts();
        let rebuilt = Graph::from_parts(starts, targets).unwrap();
        assert_eq!(graph.as_parts(), rebuilt.as_parts());
    }
    #[test]
    fn dynamic_sizes_are_supported() {
        for vertices in 1..32 {
            let starts = Vec::from_iter((0..vertices).map(|_| None));
            let graph = Graph::from_parts(starts, Vec::new()).unwrap();
            assert_eq!(graph.vertex_count(), vertices);
            assert_eq!(graph.edge_count(), 0);
        }
    }
    #[test]
    fn rejects_unrepresentable_vertex_domain() {
        let starts = Vec::from_iter((0..256).map(|_| None));
        assert!(Graph::from_parts(starts, Vec::new()).is_none());
    }
    #[test]
    fn rejects_invalid_target() {
        let outside = Vertex::try_from_usize(3).unwrap();
        assert!(
            Graph::from_parts(Vec::from([Some(edge(0)), None, None]), Vec::from([outside]),)
                .is_none()
        );
    }
    #[test]
    fn rejects_invalid_starts() {
        assert!(
            Graph::from_parts(
                Vec::from([Some(edge(1)), None, None]),
                Vec::from([vertex(1), vertex(2)]),
            )
            .is_none()
        );
        assert!(
            Graph::from_parts(
                Vec::from([Some(edge(0)), Some(edge(2)), Some(edge(1))]),
                Vec::from([vertex(0), vertex(1), vertex(2)]),
            )
            .is_none()
        );
    }
}
