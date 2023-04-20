use core::cmp::min;

use rand::Rng;
use rand::prelude::ThreadRng;

type Edges = Vec<(usize, usize)>;

struct VertixSet {
    parent: usize,
    rank: usize
}

fn build_sets(size: usize) -> Vec<VertixSet> {
    let mut s = Vec::with_capacity(size);
    for i in 0..size {
        s.push(VertixSet {parent: i, rank: 0})
    }
    s
}

fn find_in_set(v: &mut Vec<VertixSet>, i: usize) -> usize {
    if v[i].parent != i {
        v[i].parent = find_in_set(v, v[i].parent);
    }
 
    return v[i].parent;
}

fn union_set(v: &mut Vec<VertixSet>, x: usize, y: usize) {
    let xroot = find_in_set(v, x);
    let yroot = find_in_set(v, y);
 
    if v[xroot].rank < v[yroot].rank {
        v[xroot].parent = yroot;
    } else if v[xroot].rank > v[yroot].rank {
        v[yroot].parent = xroot;
    } else {
        v[yroot].parent = xroot;
        v[xroot].rank += 1;
    }
}

fn min_cut_iter(edges: &Edges, vertex_count: usize, rng: &mut ThreadRng, max_min: usize) -> usize {
    let mut s = build_sets(vertex_count);
    let mut v = vertex_count;

    while v > 2 {
        let rand_idx = rng.gen_range(0..edges.len());
        let x = find_in_set(&mut s, edges[rand_idx].0);
        let y = find_in_set(&mut s, edges[rand_idx].1);
        if x == y {
            continue;
        } else {
            v -= 1;
            union_set(&mut s, x, y)
        }
    }

    let mut cutedges = 0;

    for i in 0..edges.len() {
        let x = find_in_set(&mut s, edges[i].0);
        let y = find_in_set(&mut s, edges[i].1);
        if x != y {
            cutedges += 1;
        }

        if cutedges >= max_min {
            break;
        }
    }
 
    cutedges
}

pub fn min_cut(edges: Vec<(usize, usize)>, vertex_count: usize) -> usize {
    let iter_count = vertex_count.pow(2) * (vertex_count as f64).log2().floor() as usize;
    let mut minimum = usize::MAX;
    let mut rng = rand::thread_rng();
    for _i in 0..iter_count {
        let local_min = min_cut_iter(&edges, vertex_count, &mut rng, minimum);
        minimum = min(minimum, local_min);
    }
    return minimum;
}


#[cfg(test)]
mod tests {
    use std::fs;
    use std::collections::HashSet;

    use crate::{min_cut};

    #[test]
    fn it_works_small() {
        let g = min_cut(vec![
            (0, 4),
            (0, 7),
            (1,2),
            (1,3),
            (1,4),
            (2,3),
            (2,6),
            (2,10),
            (3,5),
            (4,5),
            (4,9),
            (4,10),
            (5,6),
            (5,9),
            (6,7),
            (6,8),
            (7,8),
            (8,11),
            (8,12),
            (8,9),
            (9,10),
            (11,12)
        ], 13);
        assert_eq!(2, g);
    }

    #[test]
    fn it_works_file() {
        let contents = fs::read_to_string("priv/graph.txt")
          .expect("Something went wrong reading the file");
        let mut edges_set = HashSet::new();
        for i in contents.split("\n") {
            let j: Vec<usize> = i.split_whitespace().map(|x| x.parse::<i32>().unwrap() as usize - 1).collect();
            if j.len() == 0 {
                continue
            }
            let v = j[0];
            for j_idx in 1..j.len() {
                if v > j[j_idx] {
                    edges_set.insert((v, j[j_idx]));
                } else {
                    edges_set.insert((j[j_idx], v));
                }
            }
        };
        assert_eq!(17, min_cut(edges_set.into_iter().collect(), 200));   
    }
}
