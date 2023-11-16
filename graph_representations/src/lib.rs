use std::cmp::Ordering;
use std::collections::LinkedList;

#[derive(Debug, Eq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub length: isize
}

impl Edge {
    pub fn new(from: usize, to: usize, length: isize) -> Self {
        Edge { from, to, length }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.cmp(&other.length)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
    }
}

pub type Edges = Vec<Edge>;

#[derive(Debug)]
pub struct SimpleGraph {
    pub size: usize,
    pub is_directed: bool,
    pub edges: Edges
}

impl SimpleGraph {
    pub fn new(size: usize, edges: Edges, is_directed: bool) -> Self {
        SimpleGraph {size, edges, is_directed}
    }

    pub fn increase_size(&mut self, diff: usize) {
        self.size += diff;
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn decrease_size(&mut self, diff: usize) {
        self.size -= diff;

        let mut pointer = 0;
        let mut eges_len = self.edges.len();

        while pointer < eges_len {
            if self.edges[pointer].from >= self.size || self.edges[pointer].to >= self.size {
                self.edges.remove(pointer);
                eges_len -= 1;
            } else {
                pointer += 1;
            }
        }
    }
}


#[derive(Debug, Clone, Copy, Eq)]
pub struct AdjacencyListEdge {
    pub length: isize,
    pub from_or_to: usize
}

impl Ord for AdjacencyListEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length.cmp(&other.length)
    }
}

impl PartialOrd for AdjacencyListEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AdjacencyListEdge {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length
    }
}

#[derive(Debug)]
pub struct AdjacencyListGraphIncome {
    pub size: usize,
    pub is_directed: bool,
    pub edges: Vec<LinkedList<AdjacencyListEdge>>
}

impl AdjacencyListGraphIncome {
    // O(m) where m is edges count
    pub fn from_simple(simple_graph: &SimpleGraph) -> Self {
        let mut edges_list: Vec<LinkedList<AdjacencyListEdge>> = Vec::with_capacity(simple_graph.size);

        for _i in 0..simple_graph.size {
            edges_list.push(LinkedList::new())
        }

        let mut graph = AdjacencyListGraphIncome { size: simple_graph.size, is_directed: simple_graph.is_directed, edges: edges_list };

        for i in &simple_graph.edges {
            graph.add_edge(i)
        }

        graph
    }

    pub fn inspect_edges(&self) -> Edges {
        let mut edges = Vec::new();
        for i in 0..self.size {
            let j_edges = &self.edges[i];

            for j in j_edges {
                edges.push(Edge {to: i, from: j.from_or_to, length: j.length});
            }
        }

        edges
    }

    pub fn add_edge(&mut self, i: &Edge) {
        if !self.is_directed {
            self.edges[i.from].push_back(AdjacencyListEdge { length: i.length, from_or_to: i.to });
            self.edges[i.to].push_back(AdjacencyListEdge { length: i.length, from_or_to: i.from });
        } else {
            self.edges[i.to].push_back(AdjacencyListEdge { length: i.length, from_or_to: i.from });
        }
    }
}


#[derive(Debug)]
pub struct AdjacencyListGraphOutcome {
    pub size: usize,
    pub is_directed: bool,
    pub edges: Vec<LinkedList<AdjacencyListEdge>>
}

impl AdjacencyListGraphOutcome {
    // O(m) where m is edges count
    pub fn from_simple(simple_graph: &SimpleGraph) -> Self {
        let mut edges_list: Vec<LinkedList<AdjacencyListEdge>> = Vec::with_capacity(simple_graph.size);

        for _i in 0..simple_graph.size {
            edges_list.push(LinkedList::new())
        }

        let mut graph = AdjacencyListGraphOutcome { size: simple_graph.size, is_directed: simple_graph.is_directed, edges: edges_list };

        for i in &simple_graph.edges {
            graph.add_edge(i)
        }

        graph
    }

    pub fn inspect_edges(&self) -> Edges {
        let mut edges = Vec::new();
        for i in 0..self.size {
            let j_edges = &self.edges[i];

            for j in j_edges {
                edges.push(Edge {to: j.from_or_to, from: i, length: j.length});
            }
        }

        edges
    }

    pub fn add_edge(&mut self, i: &Edge) {
        if !self.is_directed {
            self.edges[i.from].push_back(AdjacencyListEdge { length: i.length, from_or_to: i.to });
            self.edges[i.to].push_back(AdjacencyListEdge { length: i.length, from_or_to: i.from });
        } else {
            self.edges[i.from].push_back(AdjacencyListEdge { length: i.length, from_or_to: i.to });
        }
    }
}
