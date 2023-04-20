use std::fs::File;
use std::io::{prelude::*, BufReader};

type Graph = Vec<Vec<usize>>;

pub fn find_strong_components(size: usize, graph: Graph) -> Vec<usize> {
    let order = {
        let mut viseted_vec = vec![false; size];
        let mut new_order = vec![0; size];
        let mut current_idx: usize = 0;
        for i in 0..size {
            let vertex = size - 1 - i;
            if !viseted_vec[vertex] {
                dfs(
                    &graph,
                    &mut viseted_vec,
                    &mut new_order,
                    &mut current_idx,
                    vertex,
                )
            }
        }

        new_order
    };

    let new_graph = rebuild_graph_with_substitution(size, graph, order);

    let mut classes: Vec<usize> = vec![];
    let mut class_labels: Vec<usize> = vec![0; size];
    let mut current_class: usize = 1;

    for i in 0..size {
        let vertex = size - 1 - i;

        if class_labels[vertex] == 0 {
            let mut stack = Vec::with_capacity(size);
            let weight = dfs_classes(
                vertex,
                current_class,
                &mut stack,
                &new_graph,
                &mut class_labels,
            );
            classes.push(weight);
            current_class += 1;
        }
    }

    // let mut classes_with_deps = Vec::with_capacity(classes.len());

    // for i in 0..classes.len() {
    //     let mut s = vec![false; classes.len()];
    //     classes_with_deps.push(sum_deps(&classes, &mut classes_deps, &classes_with_deps, &mut s, i))
    // }

    // classes_deps[1].clone()
    // 1_379_666
    // 875_714

    classes.sort_by(|a, b| b.cmp(a));
    classes.into_iter().take(5).collect::<Vec<usize>>()
}

fn dfs_classes(
    vertex_idx_init: usize,
    current_class: usize,
    stack: &mut Vec<(usize, usize)>,
    graph: &Graph,
    class_labels: &mut Vec<usize>,
) -> usize {
    // let monitor = vertex_idx_init ==  600496;
    let mut vertex_idx = vertex_idx_init;
    let mut i = 0;
    let mut sum = 0;

    'outer: loop {
        if vertex_idx == 572230 {
            println!("!!")
        }
        class_labels[vertex_idx] = current_class;

        let r = graph[vertex_idx].clone();

        while i < r.len() {
            let next_vertex = r[i];
            let class = class_labels[next_vertex];
            if class == 0 {
                stack.push((i + 1, vertex_idx));
                vertex_idx = next_vertex;
                i = 0;
                continue 'outer;
            }

            i += 1;
        }

        sum += 1;

        if stack.len() == 0 {
            break;
        }

        (i, vertex_idx) = stack.pop().unwrap();
    }

    // if monitor {
    //     panic!("{}", k)
    // }

    sum
}

pub fn build_from_file(path: &str, size: usize) -> Graph {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut v = Vec::with_capacity(size);

    for _i in 0..size {
        v.push(Vec::new());
    }

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let pair = line_to_pair(&line);
        v[pair.0].push(pair.1)
    }

    v
}

fn rebuild_graph_with_substitution(size: usize, graph: Graph, order: Vec<usize>) -> Graph {
    let mut g = Vec::with_capacity(size);

    for _i in 0..size {
        g.push(Vec::new());
    }

    for i in 0..size {
        let r = &graph[i];
        for j in 0..r.len() {
            let f = order[r[j]];
            let s = order[i];
            g[f].push(s)
        }
    }

    g
}

fn dfs(
    graph: &Graph,
    viseted_vec: &mut Vec<bool>,
    new_order: &mut Vec<usize>,
    current_idx: &mut usize,
    init_vertex: usize,
) {
    let mut stack = Vec::new();
    let mut vertex = init_vertex;
    let mut iteration = 0;

    'outer: loop {
        viseted_vec[vertex] = true;
        let neighbords = graph[vertex].clone();
        while iteration < neighbords.len() {
            let neighbor = neighbords[iteration];
            if !viseted_vec[neighbor] {
                stack.push((iteration, vertex));
                vertex = neighbor;
                iteration = 0;
                continue 'outer;
                //dfs(graph, viseted_vec, new_order, current_idx, neighbor)
            }
            iteration += 1;
        }

        new_order[vertex] = current_idx.clone();
        *current_idx += 1;

        //panic!("{:?}", stack);

        if stack.len() == 0 {
            break;
        }

        (iteration, vertex) = stack.pop().unwrap()
    }
}

fn line_to_pair(line: &str) -> (usize, usize) {
    let pair: Vec<&str> = line.split(" ").collect();
    (
        pair[0].parse::<usize>().unwrap() - 1,
        pair[1].parse::<usize>().unwrap() - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRAPH_SIZE: usize = 875_714;

    #[test]
    fn coursera_example_should_work() {
        let graph = build_from_file("input.txt", GRAPH_SIZE);
        let result = find_strong_components(GRAPH_SIZE, graph);
        assert_eq!(result, vec![434821, 968, 459, 313, 211]);
    }

    #[test]
    fn coursera_lesson_should_work() {
        let graph = build_from_file("lesson.txt", 9);
        let result = find_strong_components(9, graph);
        assert_eq!(result, vec![3, 3, 3]);
    }
}
