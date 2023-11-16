use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use std::fs::File;

use std::io::{BufReader, Lines};

#[derive(Eq)]
struct Task {
    pub len: isize,
    pub weight: isize,
}

#[derive(Eq)]
struct OptimalTaks(Task);

impl Task {
    pub fn new(weight: isize, len: isize) -> Self {
        Task { len, weight }
    }

    fn diff(&self) -> isize {
        self.weight - self.len
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.diff() == other.diff() && self.weight == other.weight
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self.diff().cmp(&other.diff());
        if res == Ordering::Equal {
            self.weight.cmp(&other.weight)
        } else {
            res
        }
    }
}

impl OptimalTaks {
    pub fn new(weight: isize, len: isize) -> Self {
        OptimalTaks(Task::new(weight, len))
    }

    fn diff(&self) -> f32 {
        self.0.weight as f32 / self.0.len as f32
    }
}

impl PartialEq for OptimalTaks {
    fn eq(&self, other: &Self) -> bool {
        self.diff() == other.diff()
    }
}

impl PartialOrd for OptimalTaks {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptimalTaks {
    fn cmp(&self, other: &Self) -> Ordering {
        self.diff().partial_cmp(&other.diff()).unwrap()
    }
}

pub fn schedule(lines: &mut Lines<BufReader<File>>) -> isize {
    let count = lines.next().unwrap().unwrap().parse().unwrap();

    let mut heap: BinaryHeap<Task> = BinaryHeap::with_capacity(count);

    for line_res in lines {
        if let Ok(line) = line_res {
            let raw_data: Vec<&str> = line.split(" ").collect();
            let task = Task::new(raw_data[0].parse().unwrap(), raw_data[1].parse().unwrap());
            heap.push(task)
        }
    }

    let mut len = 0;
    let mut sum = 0;

    while let Some(i) = heap.pop() {
        len += i.len;
        sum += len * i.weight;
    }

    sum
}

pub fn schedule_optiomal(lines: &mut Lines<BufReader<File>>) -> isize {
    let count = lines.next().unwrap().unwrap().parse().unwrap();

    let mut heap: BinaryHeap<OptimalTaks> = BinaryHeap::with_capacity(count);

    for line_res in lines {
        if let Ok(line) = line_res {
            let raw_data: Vec<&str> = line.split(" ").collect();
            let task = OptimalTaks::new(raw_data[0].parse().unwrap(), raw_data[1].parse().unwrap());
            heap.push(task)
        }
    }

    let mut len = 0;
    let mut sum = 0;

    while let Some(i) = heap.pop() {
        len += i.0.len;
        sum += len * i.0.weight;
    }

    sum
}

#[derive(Eq, Clone, Copy, Debug)]
struct PrimObject {
    src: usize,
    min: isize,
}

impl PrimObject {
    pub fn new(src: usize, min: isize) -> Self {
        PrimObject { src, min }
    }
}

impl PartialEq for PrimObject {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
    }
}

impl PartialOrd for PrimObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrimObject {
    fn cmp(&self, other: &Self) -> Ordering {
        other.min.cmp(&self.min)
    }
}

type Graph = Vec<Vec<(usize, isize)>>;

fn build_graph(lines: &mut Lines<BufReader<File>>) -> Graph {
    let raw_headers = lines.next().unwrap().unwrap();
    let headers: Vec<&str> = raw_headers.split(" ").collect();

    let vertexes_count: usize = headers[0].parse().unwrap();
    let mut vertexes: Graph = Vec::with_capacity(vertexes_count);

    for _i in 0..vertexes_count {
        vertexes.push(Vec::new())
    }

    for line_res in lines {
        if let Ok(line) = line_res {
            let raw_data: Vec<&str> = line.split(" ").collect();
            let src: usize = raw_data[0].parse().unwrap();
            let target: usize = raw_data[1].parse().unwrap();
            let minimum: isize = raw_data[2].parse().unwrap();

            vertexes[src - 1].push((target - 1, minimum));
            vertexes[target - 1].push((src - 1, minimum));
        }
    }

    vertexes
}

pub fn run_prim(lines: &mut Lines<BufReader<File>>) -> isize {
    let graph = build_graph(lines);
    let mut heap: BinaryHeap<PrimObject> = BinaryHeap::new();

    let mut remove_idx = 0;
    let mut sum = 0;
    let mut removed = HashSet::new();
    removed.insert(remove_idx);

    for _i in 0..(graph.len() - 1) {
        for &j in graph[remove_idx].iter() {
            if !removed.contains(&j.0) {
                let obj = PrimObject::new(j.0, j.1);
                heap.push(obj);
            }
        }

        let mut min_obj = heap.pop().unwrap();

        while removed.contains(&min_obj.src) {
            min_obj = heap.pop().unwrap();
        }

        sum += min_obj.min;
        remove_idx = min_obj.src;
        removed.insert(remove_idx);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn non_optimal_works() {
        let file = File::open("priv/data.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();
        let result = schedule(&mut lines);
        assert_eq!(result, 69_119_377_652);
    }

    #[test]
    fn optimal_works() {
        let file = File::open("priv/data.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();
        let result = schedule_optiomal(&mut lines);
        assert_eq!(result, 67_311_454_237);
    }

    #[test]
    fn prim_works() {
        let file = File::open("priv/prim_data.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();
        let result = run_prim(&mut lines);
        assert_eq!(result, -3_612_829);
    }
}
