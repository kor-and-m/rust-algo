use std::fs::File;
use std::io::{prelude::*, BufReader};

const BIGGEST_NUMBER: usize = 1_000_000; //usize::MAX;

pub struct Graph {
    pub edges: Vec<Vec<(usize, usize)>>,
    pub size: usize
}


#[derive(Debug)]
pub struct Heap{
    data: Vec<(usize, usize)>,
    idx_map: Vec<usize>,
    size: usize,
    capacity: usize
}

impl Heap {
    pub fn new(capacity: usize) -> Self {
        Heap{
            data: Vec::with_capacity(capacity),
            idx_map: vec![0; capacity],
            size: 0,
            capacity
        }
    }

    pub fn fill(&mut self) {
        let capacity = self.capacity;
        for i in 0..capacity {
            self.insert(i, BIGGEST_NUMBER);
        }
    }

    pub fn insert(&mut self, idx: usize, val: usize) -> usize {
        self.data.push((idx, val));
        self.size += 1;

        if self.size == 1 {
            return 0
        }

        self.idx_map[idx] = self.size - 1;

        self.up_fn(self.size - 1)
    }

    pub fn decrease_by_idx(&mut self, idx: usize, new_val: usize) -> usize {
        let data_idx = self.idx_map[idx];
        if data_idx >= self.capacity {
            return idx;
        }

        let v = &mut self.data[data_idx].1;
        let new_idx = if *v < new_val { idx } else { *v = new_val; self.up_fn(data_idx) };
        new_idx
    }

    pub fn get_and_remove_min(&mut self) -> (usize, usize) {
        self.data.swap(0, self.size - 1);
        let result = self.data.pop().unwrap();
        self.size -= 1;

        if self.size > 0 {
            self.idx_map[result.0] = self.capacity;
            let new_head = self.data[0].0;
            self.idx_map[new_head] = 0;
            self.down_fn(0);
        }

        result
    }

    fn down_fn(&mut self, mut new_elem_idx: usize) -> usize {
        loop {
            let children = self.find_children(new_elem_idx);

            if children.len() == 0 {
                break;
            }

            let min_children_id = if children.len() == 1 { children[0] } else {
                if self.data[children[0]].1 < self.data[children[1]].1 { children[0] } else { children[1] }
            };

            if self.data[min_children_id].1 >= self.data[new_elem_idx].1 {
                break;
            }

            self.idx_map[self.data[min_children_id].0] = new_elem_idx;
            self.idx_map[self.data[new_elem_idx].0] = min_children_id;

            self.data.swap(new_elem_idx, min_children_id);

            new_elem_idx = min_children_id;
        }

        new_elem_idx
    }

    fn up_fn(&mut self, mut new_elem_idx: usize) -> usize {
        loop {
            if new_elem_idx == 0 {
                return 0;
            }
            let parent_idx = Self::find_parent(new_elem_idx);
            if self.data[parent_idx].1 <= self.data[new_elem_idx].1 {
                break;
            }
            self.idx_map[self.data[parent_idx].0] = new_elem_idx;
            self.idx_map[self.data[new_elem_idx].0] = parent_idx;

            self.data.swap(new_elem_idx, parent_idx);

            new_elem_idx = parent_idx;
        }

        new_elem_idx
    }

    fn find_parent(idx: usize) -> usize {
        (idx + 1) / 2 - 1
    }

    fn find_children(&self, idx: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(2);
        let first = (idx + 1) * 2 - 1;
        for i in 0..2 {
            if self.size > first + i {
                result.push(first + i)
            }
        }
        result
    }
}

pub fn build_graph_from_file(path: &str, size: usize) -> Graph {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut v =  Vec::with_capacity(size);

    for _i in 0..size {
        v.push(Vec::new());
    }

    for line_res in reader.lines() {
        let line = line_res.unwrap().replace("\t", " ");
        let mut splitter = line.splitn(2, ' ');
        let idx: usize = splitter.next().unwrap().parse::<usize>().unwrap() - 1;

        v[idx] = splitter.next().unwrap().split(" ").filter(|x| *x != "").map(|x| {
            let v: Vec<&str> = x.split(",").collect();
            (v[0].parse::<usize>().unwrap() - 1, v[1].parse::<usize>().unwrap())
        }).collect();
    }

    Graph{edges: v, size}
}

pub fn shortest_path(graph: Graph, src: usize) -> Vec<usize> {
    let mut vertex_heap = Heap::new(graph.size);
    vertex_heap.fill();
    vertex_heap.decrease_by_idx(src, 0);
    let mut v = vec![0; graph.size];

    let mut max_scores = 0;

    for _i in 0..(graph.size - 1) {
        let (active_vertex, active_scores) = vertex_heap.get_and_remove_min();

        if active_scores < max_scores {
            panic!("not correct algo {} {}", max_scores, active_scores);
        }

        max_scores = active_scores;

        v[active_vertex] = active_scores;
        let edges = &graph.edges[active_vertex];
        for i in 0..edges.len() {
            let edge = edges[i];
            vertex_heap.decrease_by_idx(edge.0, active_scores + edge.1);
        }
    }

    let (active_vertex, active_scores) = vertex_heap.get_and_remove_min();

    v[active_vertex] = active_scores;

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
        let output_idx = [7,37,59,82,99,115,133,165,188,197];
        for i in 0..output_idx.len() {
            coursera_result[i] = result[output_idx[i] - 1];
        }
        assert_eq!(coursera_result, [2599,2610,2947,2052,2367,2399,2029,2442,2505,3068]);
        // [2599, 2610, 2803, 2052, 2367, 2186, 2029, 2229, 2505, 3068]
    }
}
