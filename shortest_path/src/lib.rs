use std::fs::File;
use std::io::{prelude::*, BufReader};

use updatable_heap::UpdateableHeap;

pub struct Graph {
    pub edges: Vec<Vec<(usize, usize)>>,
    pub size: usize,
}

pub fn build_graph_from_file(path: &str, size: usize) -> Graph {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut v = Vec::with_capacity(size);

    for _i in 0..size {
        v.push(Vec::new());
    }

    for line_res in reader.lines() {
        let line = line_res.unwrap().replace("\t", " ");
        let mut splitter = line.splitn(2, ' ');
        let idx: usize = splitter.next().unwrap().parse::<usize>().unwrap() - 1;

        v[idx] = splitter
            .next()
            .unwrap()
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| {
                let v: Vec<&str> = x.split(",").collect();
                (
                    v[0].parse::<usize>().unwrap() - 1,
                    v[1].parse::<usize>().unwrap(),
                )
            })
            .collect();
    }

    Graph { edges: v, size }
}

pub fn shortest_path(graph: Graph, src: usize) -> Vec<isize> {
    let mut vertex_heap: UpdateableHeap<()> = UpdateableHeap::new(graph.size);
    vertex_heap.fill();
    vertex_heap.decrease_by_idx(src, 0, ());
    let mut v = vec![0; graph.size];

    let mut max_scores = 0;

    for _i in 0..graph.size {
        let elem = vertex_heap.get_and_remove_min();
        let active_vertex = elem.idx;
        let active_scores = elem.ordering_key;

        if active_scores < max_scores {
            panic!("not correct algo {} {}", max_scores, active_scores);
        }

        max_scores = active_scores;

        v[active_vertex] = active_scores;
        let edges = &graph.edges[active_vertex];
        for i in 0..edges.len() {
            let edge = edges[i];
            vertex_heap.decrease_by_idx(edge.0, active_scores + edge.1 as isize, ());
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_graph_should_work() {
        let result = build_graph_from_file("input.txt", 200);
        assert_eq!(result.edges[0][0].0, 79);
        assert_eq!(result.edges[50][2].1, 1096);
    }

    #[test]
    fn coursera_example_should_work() {
        let g = build_graph_from_file("input.txt", 200);
        let result = shortest_path(g, 0);
        let mut coursera_result = [0; 10];
        let output_idx = [7, 37, 59, 82, 99, 115, 133, 165, 188, 197];
        for i in 0..output_idx.len() {
            coursera_result[i] = result[output_idx[i] - 1];
        }
        assert_eq!(
            coursera_result,
            [2599, 2610, 2947, 2052, 2367, 2399, 2029, 2442, 2505, 3068]
        );
    }
}
