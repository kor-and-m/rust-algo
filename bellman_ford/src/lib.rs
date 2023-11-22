use core::isize::MAX;
use graph_representations::AdjacencyListGraphIncome;

pub fn bellman_ford(graph: &AdjacencyListGraphIncome, source: usize) -> Option<Vec<isize>> {
    let size = graph.size;
    let mut bellman_ford_state = [vec![MAX; size], vec![MAX; size]];
    bellman_ford_state[0][source] = 0;
    let mut state_changed: bool;

    for i in 0..size {
        state_changed = false;
        for s in 0..size {
            let prev_state = &bellman_ford_state[i % 2];
            let mut min_val = MAX;
            for e in graph.edges[s].iter() {
                let c = if e.length > 0 {
                    min_val - e.length < prev_state[e.from_or_to]
                } else {
                    min_val < prev_state[e.from_or_to] + e.length
                };
                min_val = if c {
                    min_val
                } else {
                    prev_state[e.from_or_to] + e.length
                };
            }

            bellman_ford_state[(i + 1) % 2][s] = if prev_state[s] <= min_val {
                prev_state[s]
            } else {
                state_changed = true;
                min_val
            };
        }

        if !state_changed {
            return Some(bellman_ford_state[(i + 1) % 2].clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use graph_representations::{Edge, SimpleGraph};
    use std::fs::File;

    use std::io::prelude::*;
    use std::io::{BufReader, Lines};

    use super::*;

    #[test]
    fn it_works() {
        let edges = vec![
            Edge {
                from: 0,
                to: 1,
                length: 2,
            },
            Edge {
                from: 1,
                to: 2,
                length: 2,
            },
            Edge {
                from: 2,
                to: 3,
                length: 2,
            },
            Edge {
                from: 4,
                to: 3,
                length: 4,
            },
            Edge {
                from: 1,
                to: 4,
                length: 1,
            },
            Edge {
                from: 0,
                to: 4,
                length: 4,
            },
        ];

        let simple_graph = SimpleGraph::new(5, edges, true);

        let graph = AdjacencyListGraphIncome::from_simple(&simple_graph);
        let result = bellman_ford(&graph, 0).unwrap();
        assert_eq!(result, vec![0, 2, 4, 6, 3]);
    }

    #[test]
    fn it_works_negative() {
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
            Edge {
                from: 6,
                to: 0,
                length: 0,
            },
            Edge {
                from: 6,
                to: 1,
                length: 0,
            },
            Edge {
                from: 6,
                to: 2,
                length: 0,
            },
            Edge {
                from: 6,
                to: 3,
                length: 0,
            },
            Edge {
                from: 6,
                to: 4,
                length: 0,
            },
            Edge {
                from: 6,
                to: 5,
                length: 0,
            },
        ];

        let simple_graph = SimpleGraph::new(7, edges, true);

        let graph = AdjacencyListGraphIncome::from_simple(&simple_graph);
        let result = bellman_ford(&graph, 6).unwrap();
        assert_eq!(result, vec![0, -2, -3, -6, 0, -1, 0]);
    }

    #[test]
    fn it_works_first() {
        let file =
            File::open("priv/first_graph.txt").expect("Something went wrong reading the file");

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

        let mut graph = SimpleGraph::new(headers[0], edges, true);
        graph.increase_size(1);
        for i in 0..graph.size {
            graph.add_edge(Edge {
                from: headers[0],
                to: i,
                length: 0,
            })
        }

        let mut adjacency_graph = AdjacencyListGraphIncome::from_simple(&graph);

        let result = bellman_ford(&mut adjacency_graph, headers[0]);
        assert_eq!(result, None);
    }
}
