use bellman_ford::bellman_ford;
use dijkstra::dijkstra_with_reweighting;
use graph_representations::{
    AdjacencyListGraphIncome, AdjacencyListGraphOutcome, Edge, SimpleGraph,
};

pub fn jonsons(graph: &mut SimpleGraph) -> Option<Vec<Vec<isize>>> {
    let last_idx = graph.size;
    graph.increase_size(1);
    for i in 0..last_idx {
        graph.add_edge(Edge {
            from: last_idx,
            to: i,
            length: 0,
        })
    }

    let mut adjacency_graph = AdjacencyListGraphIncome::from_simple(graph);

    if let Some(v) = bellman_ford(&mut adjacency_graph, last_idx) {
        graph.decrease_size(1);
        reweight_graph(graph, &v);
        let mut res = Vec::with_capacity(graph.size);
        let outcome_graph = AdjacencyListGraphOutcome::from_simple(&graph);
        for i in 0..graph.size {
            res.push(dijkstra_with_reweighting(&outcome_graph, i, &v))
        }
        Some(res)
    } else {
        None
    }
}

fn reweight_graph(graph: &mut SimpleGraph, reweight_vec: &Vec<isize>) {
    for e in graph.edges.iter_mut() {
        e.length += reweight_vec[e.from] - reweight_vec[e.to];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::isize::MAX;

    use std::io::prelude::*;
    use std::io::{BufReader, Lines};

    #[test]
    fn it_works() {
        let edges = vec![
            Edge {
                from: 0,
                to: 1,
                length: -2,
            },
            Edge {
                from: 1,
                to: 2,
                length: -1,
            },
            Edge {
                from: 2,
                to: 0,
                length: 4,
            },
            Edge {
                from: 2,
                to: 3,
                length: -3,
            },
            Edge {
                from: 4,
                to: 3,
                length: -4,
            },
            Edge {
                from: 4,
                to: 5,
                length: 1,
            },
            Edge {
                from: 2,
                to: 5,
                length: 2,
            },
        ];

        let mut graph = SimpleGraph::new(6, edges, true);
        let result = jonsons(&mut graph).unwrap();
        assert_eq!(result[0], vec![0, -2, -3, -6, MAX, -1]);
        assert_eq!(
            result,
            vec![
                vec![0, -2, -3, -6, MAX, -1],
                vec![3, 0, -1, -4, MAX, 1],
                vec![4, 2, 0, -3, MAX, 2],
                vec![MAX, MAX, MAX, 0, MAX, MAX],
                vec![MAX, MAX, MAX, -4, 0, 1],
                vec![MAX, MAX, MAX, MAX, MAX, 0]
            ]
        );
    }

    #[test]
    fn it_works_first() {
        let mut graph = graph_from_file("priv/first_graph.txt");
        let result = jonsons(&mut graph);
        assert_eq!(result, None);
    }

    #[test]
    fn it_works_second() {
        let mut graph = graph_from_file("priv/second_graph.txt");
        let result = jonsons(&mut graph);
        assert_eq!(result, None);
    }

    #[test]
    fn it_works_third() {
        let mut graph = graph_from_file("priv/third_graph.txt");
        let result = jonsons(&mut graph).unwrap();
        let mut min = MAX;

        for i in 0..graph.size {
            for j in 0..graph.size {
                let v = result[i][j];
                min = if v < min { v } else { min }
            }
        }
        assert_eq!(min, -19);
    }

    #[test]
    fn it_works_huge() {
        let mut graph = graph_from_file("priv/huge_graph.txt");
        let result = jonsons(&mut graph).unwrap();
        let mut min = MAX;

        for i in 0..graph.size {
            for j in 0..graph.size {
                let v = result[i][j];
                min = if v < min { v } else { min }
            }
        }
        assert_eq!(min, -6);
    }

    fn graph_from_file(path: &str) -> SimpleGraph {
        let file = File::open(path).expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();

        let headers: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let mut edges = Vec::with_capacity(headers[1]);

        for i in lines {
            if let Ok(v) = i {
                let value: Vec<isize> = v.split(" ").map(|x| x.parse().unwrap()).collect();
                edges.push(Edge::new(
                    value[0] as usize - 1,
                    value[1] as usize - 1,
                    value[2],
                ))
            }
        }

        SimpleGraph::new(headers[0], edges, true)
    }
}
