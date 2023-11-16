use std::{rc::Rc, cell::RefCell, cmp::{max, min}};

pub struct HuffmanTree {
    pub max_rank: usize,
    pub min_rank: usize,
    pub weight: usize,
    pub left: Option<Rc<RefCell<HuffmanTree>>>,
    pub right: Option<Rc<RefCell<HuffmanTree>>>
}

impl HuffmanTree {
    pub fn new(weight: usize) -> Self {
        Self { max_rank: 0, min_rank: 0, weight, left: None, right: None }
    }

    pub fn union(left: Rc<RefCell<HuffmanTree>>, right: Rc<RefCell<HuffmanTree>>) -> Self {
        let weight = left.borrow().weight + right.borrow().weight;
        let max_rank = max(left.borrow().max_rank, right.borrow().max_rank) + 1;
        let min_rank = min(left.borrow().min_rank, right.borrow().min_rank) + 1;
        Self { max_rank, min_rank, weight, left: Some(left), right: Some(right) }
    }
}

pub fn build_huffman_tree(weights: &Vec<usize>, weights_count: usize) -> Rc<RefCell<HuffmanTree>> {
    let mut forest = Vec::with_capacity(weights_count);

    for i in 0..weights_count {
        let leaf = Rc::new(RefCell::new(HuffmanTree::new(weights[i])));
        forest.push(leaf);
    }

    for i in 1..weights_count {
        let min_tree1 = Rc::clone(&forest[i - 1]);
        let min_tree2 = Rc::clone(&forest[i]);

        let union = Rc::new(RefCell::new(HuffmanTree::union(min_tree1, min_tree2)));

        forest[i] = union;

        for j in (i + 1)..weights_count {
            let min_tree1 = Rc::clone(&forest[j - 1]);
            let min_tree2 = Rc::clone(&forest[j]);

            if min_tree2.borrow().weight < min_tree1.borrow().weight {
                forest.swap(j - 1, j)
            } else {
                break;
            }
        }
    }

    Rc::clone(&forest[weights_count - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::{BufReader, Lines};
    use std::io::prelude::*;

    #[test]
    fn it_works() {
        let file =
          File::open("priv/huffman_input.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();

        let line_count: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let mut weights: Vec<usize> = Vec::with_capacity(line_count);

        for line_res in lines {
            if let Ok(line) = line_res {
                weights.push(line.parse().unwrap())
            }
        }

        weights.sort();

        let tree = build_huffman_tree(&weights, line_count);

        let sum: usize = weights.into_iter().sum();

        assert_eq!(sum, 4990911370);
        assert_eq!(tree.borrow().weight, 4990911370);
        assert_eq!(tree.borrow().max_rank, 19);
        assert_eq!(tree.borrow().min_rank, 9);
    }
}
