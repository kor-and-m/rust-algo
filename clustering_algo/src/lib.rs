use std::fs::File;

use std::io::{BufReader, Lines};
use std::collections::HashMap;
use std::rc::Rc;

use union_find::{UnionFind, Node};

#[derive(Debug)]
struct Edge {
    src: usize,
    target: usize,
    distance: usize
}

pub fn run_clustering_algo_with_huge_input(lines: &mut Lines<BufReader<File>>) -> usize {
    let raw_input = lines.next().unwrap().unwrap();
    let raw_headers: Vec<&str> = raw_input.split(" ").collect();

    let nodes_count: usize = raw_headers[0].parse().unwrap();
    let _size: usize = raw_headers[1].parse().unwrap();
    let mut edges = init_huge_edges(lines);
    run_clustering_algo(&mut edges, nodes_count, |t, _e| (nodes_count - t, false))
}

pub fn run_clustering_algo_with_edge_input(lines: &mut Lines<BufReader<File>>, components: usize) -> usize {
    let raw_headers = lines.next().unwrap().unwrap();

    let nodes_count: usize = raw_headers.parse().unwrap();
    let mut edges = init_edges(lines);
    run_clustering_algo(&mut edges, nodes_count, |t, e| {
        (e.distance, t > nodes_count - components)
    })
}

fn run_clustering_algo<T, F>(edges: &mut Vec<Edge>, nodes_count: usize, f: F) -> T where F: Fn(usize, &Edge) -> (T, bool) {
    let mut node_map = build_node_map(edges, nodes_count);
    
    let mut t = 0;

    let mut result: Option<T> = None;

    for i in edges.iter() {
        let src_node = fetch_node_by_id(&mut node_map, i.src);
        let target_node = fetch_node_by_id(&mut node_map, i.target);
        let src_v = *UnionFind::find_val(&src_node).borrow();
        let target_v = *UnionFind::find_val(&target_node).borrow();

        if src_v != target_v {
            t += 1;

            UnionFind::union(&src_node, &target_node);
            
            let r = f(t, i);
            result = Some(r.0);
            if r.1 {
                return result.unwrap()
            }
        }
    }

    result.unwrap()
}


fn build_node_map(edges: &mut Vec<Edge>, nodes_count: usize) -> HashMap<usize, Node<usize>> {
    let mut node_map = HashMap::with_capacity(nodes_count);

    edges.sort_by(|a, b| a.distance.cmp(&b.distance));

    for i in 0..nodes_count {
        create_root(&mut node_map, i + 1);
    }

    node_map
}

fn init_edges(lines: &mut Lines<BufReader<File>>) -> Vec<Edge> {
    let mut edges = Vec::new();

    for line_res in lines {
        if let Ok(line) = line_res {
            let raw_data: Vec<&str> = line.split(" ").collect();
            let src: usize = raw_data[0].parse().unwrap();
            let target: usize = raw_data[1].parse().unwrap();
            let distance: usize = raw_data[2].parse().unwrap();

            edges.push(Edge {src, target, distance})
        }
    }

    edges
}

fn init_huge_edges(lines: &mut Lines<BufReader<File>>) -> Vec<Edge> {
    let mut edges = Vec::new();
    let mut edges_map = HashMap::new();
    let mut vertexes = Vec::new();

    let mut line_pointer: usize = 0;
    for line_res in lines {
        if let Ok(line) = line_res {
            let mut values = [0u8; 24];
            let mut cursor = 0;
            for i in line.split(" ") {

                if let Ok(s) = i.parse::<u8>() {
                    values[cursor] = s;
                    cursor += 1;
                }
            }

            vertexes.push(values);
            edges_map.entry(values).or_insert(vec![]).push(line_pointer);
            line_pointer += 1;
        }
    }

    let mut id = 0;
    for v in &vertexes {
        let zero_distance_v = edges_map.get(v).unwrap();
        for i in zero_distance_v {
            if *i != id {
                edges.push(Edge { src: id + 1, target: i + 1, distance: 0 })
            }
        }
        for i in produce_neighbours(v) {
            if let Some(neighbours) = edges_map.get(&i.0) {
                for n in neighbours {
                    edges.push(Edge { src: id + 1, target: n + 1, distance: i.1 })
                }
            }
        }
        id += 1;
    }

    edges
}

fn produce_neighbours(origin: &[u8; 24]) -> [([u8; 24], usize); 300] {
    let mut neighbors = [(*origin, 1); 300];

    let mut pointer = 0;
    for i in 0..25 {
        for j in 0..i {
            if i < 24 {
                neighbors[pointer].0[i] = (neighbors[pointer].0[i] + 1) % 2;
                neighbors[pointer].1 = 2;
            }
            neighbors[pointer].0[j] = (neighbors[pointer].0[j] + 1) % 2;

            pointer += 1;
        }
    }
    neighbors
}

fn create_root(node_map: &mut HashMap<usize, Node<usize>>, id: usize) -> Node<usize> {
    let node = UnionFind::create_root(id);
    node_map.insert(id, Rc::clone(&node));
    node
}

fn fetch_node_by_id(node_map: &mut HashMap<usize, Node<usize>>, id: usize) -> Node<usize> {
    match node_map.get(&id) {
        Some(v) => Rc::clone(v),
        None => panic!("Error map initialization {}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn sould_work_with_small_input() {
        let file =
          File::open("priv/input.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();
        let result = run_clustering_algo_with_edge_input(&mut lines, 4);
        assert_eq!(result, 106);
    }

    #[test]
    fn sould_work_with_huge_input() {
        let file =
          File::open("priv/huge_input.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();
        let result = run_clustering_algo_with_huge_input(&mut lines);
        assert_eq!(result, 6118);
    }

    #[test]
    fn sould_produce_naighbours() {
        let origin = [0; 24];
        let result = produce_neighbours(&origin);
        result.into_iter().for_each(|x| {
            let sum: u8 = x.0.into_iter().sum();
            assert!(sum == 1 || sum == 2);
            assert_eq!(x.1, sum as usize)
        });
    }
}
