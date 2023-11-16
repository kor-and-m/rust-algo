use graph_representations::AdjacencyListGraphOutcome;
use updatable_heap::UpdateableHeap;

pub fn dijkstra(graph: &AdjacencyListGraphOutcome, source: usize) -> Vec<isize> {
    shortest_path(graph, source)
}

pub fn dijkstra_with_reweighting(graph: &AdjacencyListGraphOutcome, source: usize, reweight_vec: &Vec<isize>) -> Vec<isize> {
    let mut vertex_heap: UpdateableHeap<(usize, usize, isize)> = UpdateableHeap::new(graph.size);
    vertex_heap.fill();
    vertex_heap.decrease_by_idx(source, 0, (source, source, 0));
    let mut scores = vec![isize::MAX; graph.size];
    let mut real_scores = vec![isize::MAX; graph.size];
    real_scores[source] = 0;
    scores[source] = 0;

    let mut max_scores = 0;

    for _i in 0..graph.size {
        let elem = vertex_heap.get_and_remove_min();
        let active_vertex = elem.idx;
        let active_scores = elem.ordering_key;
        let (from_vertex, _to_vertex, prev_len) = elem.payload;

        if active_scores == isize::MAX {
            break;
        }

        if active_scores < max_scores {
            panic!("not correct algo {} {}", max_scores, active_scores);
        }

        max_scores = active_scores;

        scores[active_vertex] = active_scores;

        if scores[active_vertex] != isize::MAX {
            real_scores[active_vertex] = real_scores[from_vertex] + prev_len + reweight_edge(reweight_vec, from_vertex, active_vertex);
        }
        let edges = &graph.edges[active_vertex];
        for edge in edges.iter() {
            vertex_heap.decrease_by_idx(edge.from_or_to, active_scores + edge.length, (active_vertex, edge.from_or_to, edge.length));
        }
    }

    real_scores
}


fn shortest_path(graph: &AdjacencyListGraphOutcome, src: usize) -> Vec<isize> {
    let mut vertex_heap: UpdateableHeap<usize> = UpdateableHeap::new(graph.size);
    vertex_heap.fill();
    vertex_heap.decrease_by_idx(src, 0, 0);
    let mut scores = vec![isize::MAX; graph.size];

    let mut max_scores = 0;

    for _i in 0..graph.size {
        let elem = vertex_heap.get_and_remove_min();
        let active_vertex = elem.idx;
        let active_scores = elem.ordering_key;

        if active_scores < max_scores {
            panic!("not correct algo {} {}", max_scores, active_scores);
        }

        max_scores = active_scores;

        scores[active_vertex] = active_scores;
        let edges = &graph.edges[active_vertex];
        for edge in edges.iter() {
            vertex_heap.decrease_by_idx(edge.from_or_to, active_scores + edge.length, active_vertex);
        }
    }

    scores
}

fn reweight_edge(reweight_vec: &Vec<isize>, from: usize, to: usize) -> isize {
    reweight_vec[to] - reweight_vec[from]
}

#[cfg(test)]
mod tests {
    use graph_representations::{Edges, Edge, SimpleGraph};

    use super::*;

    #[test]
    fn it_works() {
        let edges: Edges = vec![
            Edge::new(0, 1, 5),
            Edge::new(1, 2, 6),
            Edge::new(2, 3, 2),
            Edge::new(0, 2, 15)
        ];
        let graph = SimpleGraph::new(5, edges, false);
        let outcome = AdjacencyListGraphOutcome::from_simple(&graph);
        let result = dijkstra(&outcome, 0);
        assert_eq!(result, vec![0, 5, 11, 13, isize::MAX]);
    }

    #[test]
    fn it_works2() {
        let edges: Edges = vec![
            Edge { from: 0, to: 1, length: 0 },
            Edge { from: 1, to: 2, length: 0 },
            Edge { from: 2, to: 0, length: 1 },
            Edge { from: 2, to: 3, length: 0 },
            Edge { from: 4, to: 3, length: 2 },
            Edge { from: 4, to: 5, length: 2 },
            Edge { from: 2, to: 5, length: 0 }
        ];

        let graph = SimpleGraph::new(6, edges, true);
        let outcome = AdjacencyListGraphOutcome::from_simple(&graph);
        let result = dijkstra(&outcome, 0);
        assert_eq!(result, vec![0, 0, 0, 0, isize::MAX, 0]);
    }

    #[test]
    fn it_works3() {
        let edges: Edges = vec![
            Edge { from: 0, to: 1, length: 0 },
            Edge { from: 1, to: 2, length: 0 },
            Edge { from: 2, to: 0, length: 1 },
            Edge { from: 2, to: 3, length: 0 },
            Edge { from: 4, to: 3, length: 2 },
            Edge { from: 4, to: 5, length: 2 },
            Edge { from: 2, to: 5, length: 0 }
        ];

        let graph = SimpleGraph::new(6, edges, true);
        let outcome = AdjacencyListGraphOutcome::from_simple(&graph);
        let result = dijkstra(&outcome, 4);
        assert_eq!(result, vec![isize::MAX, isize::MAX, isize::MAX, 2, 0, 2]);
    }

    #[test]
    fn it_works4() {
        let edges: Edges = vec![
            Edge { from: 0, to: 1, length: 0 },
            Edge { from: 1, to: 2, length: 0 },
            Edge { from: 2, to: 0, length: 1 },
            Edge { from: 2, to: 3, length: 0 },
            Edge { from: 4, to: 3, length: 2 },
            Edge { from: 4, to: 5, length: 2 },
            Edge { from: 2, to: 5, length: 0 }
        ];

        let graph = SimpleGraph::new(6, edges, true);
        let v = vec![0, -2, -3, -6, 0, -1, 0];
        let outcome = AdjacencyListGraphOutcome::from_simple(&graph);
        let result = dijkstra_with_reweighting(&outcome, 4, &v);
        assert_eq!(result, vec![isize::MAX, isize::MAX, isize::MAX, -4, 0, 1]);
    }

    #[test]
    fn it_works5() {
        let edges: Edges = vec![
            Edge { from: 0, to: 1, length: 0 },
            Edge { from: 1, to: 2, length: 0 },
            Edge { from: 2, to: 0, length: 1 },
            Edge { from: 2, to: 3, length: 0 },
            Edge { from: 4, to: 3, length: 2 },
            Edge { from: 4, to: 5, length: 2 },
            Edge { from: 2, to: 5, length: 0 }
        ];

        let graph = SimpleGraph::new(6, edges, true);
        let v = vec![0, -2, -3, -6, 0, -1, 0];
        let outcome = AdjacencyListGraphOutcome::from_simple(&graph);
        let result = dijkstra_with_reweighting(&outcome, 0, &v);
        assert_eq!(result, vec![0, -2, -3, -6, isize::MAX, -1]);
    }
}

