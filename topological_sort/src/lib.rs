use std::collections::HashSet;

type EdgesSet = HashSet<(usize, usize)>;
type Edges = Vec<Vec<usize>>;

pub struct Grpah {
    size: usize,
    edges_set: EdgesSet,
    edges: Edges,
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

impl Grpah {
    pub fn new(size: usize, edges: &EdgesSet) -> Self {
        Grpah {
            size,
            edges: Grpah::build_edges(size, edges),
            edges_set: edges.clone(),
        }
    }

    fn build_edges(size: usize, edges: &EdgesSet) -> Edges {
        let mut v = Vec::with_capacity(size);
        for _i in 0..size {
            v.push(Vec::new());
        }

        for edge in edges.iter() {
            let from = edge.0;
            let to = edge.1;
            v[from].push(to)
        }

        v
    }

    pub fn edges(&self) -> &EdgesSet {
        &self.edges_set
    }

    pub fn sort(&mut self) -> bool {
        let mut colors_vec = vec![Color::White; self.size];
        let mut result = vec![];

        for i in 0..self.size {
            let color = colors_vec[i];
            if color == Color::White {
                if !self.dfs_sort(i, &mut colors_vec, &mut result) {
                    return false;
                }
                result.push(i)
            } else if color == Color::Gray {
                return false;
            }
        }

        result.reverse();

        let mut new_set = HashSet::new();

        for edge in self.edges_set.iter() {
            new_set.insert((result[edge.0], result[edge.1]));
        }

        self.edges = Grpah::build_edges(self.size, &new_set);
        self.edges_set = new_set;

        true
    }

    fn dfs_sort(&self, edge_idx: usize, colors: &mut Vec<Color>, result: &mut Vec<usize>) -> bool {
        colors[edge_idx] = Color::Gray;

        let edge = &self.edges[edge_idx];

        for from in 0..edge.len() {
            let to = edge[from];
            let color = colors[to];
            if color == Color::White {
                if !self.dfs_sort(to, colors, result) {
                    return false;
                }
                result.push(to);
            } else if color == Color::Gray {
                return false;
            }
        }

        colors[edge_idx] = Color::Black;

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_show_edges() {
        let e = HashSet::from([(0, 3), (3, 2), (3, 1), (2, 1)]);
        let g = Grpah::new(4, &e);
        let result = g.edges().clone();
        assert_eq!(result, e);
    }

    #[test]
    fn should_sort_edges() {
        let e = HashSet::from([(0, 3), (3, 2), (3, 1), (2, 1)]);
        let mut g = Grpah::new(4, &e);
        assert!(g.sort());
        let result = g.edges().clone();
        assert_eq!(result, HashSet::from([(1, 3), (2, 3), (0, 1), (1, 2)]));
    }

    #[test]
    fn should_not_sort_cicle_graph() {
        let e = HashSet::from([(0, 3), (3, 2), (3, 1), (2, 1), (1, 3)]);
        let mut g = Grpah::new(4, &e);
        assert!(!g.sort());
        let result = g.edges().clone();
        assert_eq!(result, e);
    }

    #[test]
    fn should_sort_edges_twice() {
        let e = HashSet::from([(0, 3), (3, 2), (3, 1), (2, 1)]);
        let mut g = Grpah::new(4, &e);
        assert!(g.sort());
        let result1 = g.edges().clone();
        assert!(g.sort());
        let result2 = g.edges().clone();
        assert_eq!(result1, result2);
    }
}
