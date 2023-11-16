use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
pub struct Tree {
    pub left: Option<Rc<RefCell<Tree>>>,
    pub right: Option<Rc<RefCell<Tree>>>,
    pub frequency: f32,
    vertex_sum: f32,
    vertex_avg: f32,
    pub val: usize
}

impl Tree {
    pub fn new_leaf(val: usize, frequency: f32) -> Self {
        Tree {
            left: None,
            right: None,
            frequency,
            val,
            vertex_sum: frequency,
            vertex_avg: frequency
        }
    }

    pub fn union_trees(root_val: usize, root_frequency: f32, first: Option<Rc<RefCell<Tree>>>, second: Option<Rc<RefCell<Tree>>>) -> Self {
        match (first, second) {
            (None, None) => unreachable!(),
            (Some(v1), Some(v2)) => {
                let v1_copy = Rc::clone(&v1);
                let v2_copy = Rc::clone(&v2);
                let v1l = v1.borrow();
                let v2l = v2.borrow();

                let (left, right) = match (v1l.val < root_val, v2l.val < root_val) {
                    (true, true) => unreachable!(),
                    (false, false) => unreachable!(),
                    (false, true) => (v2_copy, v1_copy),
                    (true, false) => (v1_copy, v2_copy)
                };
                let vertex_sum = v1l.vertex_sum + v2l.vertex_sum + root_frequency;
                let vertex_avg = vertex_sum + v1l.vertex_avg + v2l.vertex_avg;

                Tree {
                    left: Some(left),
                    right: Some(right),
                    frequency: root_frequency,
                    val: root_val,
                    vertex_sum,
                    vertex_avg
                }
            },
            (v1, v2) => {
                let v = if v1.is_none() {
                    v2
                } else {
                    v1
                }.unwrap();

                let (vertex_val, vertex_sum, vertex_avg) = {
                    let vl = v.borrow();
                    let vertex_sum = vl.vertex_sum + root_frequency;
                    (vl.val, vertex_sum, vertex_sum + vl.vertex_avg)
                };

                if vertex_val < root_val {
                    Tree {
                        left: Some(v),
                        right: None,
                        frequency: root_frequency,
                        val: root_val,
                        vertex_sum,
                        vertex_avg
                    }
                } else {
                    Tree {
                        left: None,
                        right: Some(v),
                        frequency: root_frequency,
                        val: root_val,
                        vertex_sum,
                        vertex_avg
                    }
                }
            }
        }
    }

    pub fn is_consistent(&self) -> bool {
        self.calculate_avg() == self.vertex_avg
    }

    pub fn calculate_avg(&self) -> f32 {
        self.calculate_avg_step(1)
    }

    pub fn calculate_avg_step(&self, step: usize) -> f32 {
        let l = if let Some(v) = self.left.as_ref() {
            v.borrow().calculate_avg_step(step + 1)
        } else {
            0.0
        };
        let r = if let Some(v) = self.right.as_ref() {
            v.borrow().calculate_avg_step(step + 1)
        } else {
            0.0
        }; 
        self.frequency * step as f32 + l + r
    }
}

pub fn build_tree(frequencies: &[f32]) -> Rc<RefCell<Tree>> {
    let l = frequencies.len();
    let mut forest = vec![vec![None; l]; l];

    for i in 0..l {
        forest[i][i] = Some(Rc::new(RefCell::new(Tree::new_leaf(i, frequencies[i]))))

    }

    for i in 1..l {
        let f = frequencies[i - 1];
        let s = frequencies[i];
        let tree = if f < s {
            Tree::union_trees(i, s, Some(Rc::new(RefCell::new(Tree::new_leaf(i - 1, f)))), None)
        } else {
            Tree::union_trees(i - 1, f, Some(Rc::new(RefCell::new(Tree::new_leaf(i, s)))), None)
        };

        forest[i-1][i] = Some(Rc::new(RefCell::new(tree)));
    }


    for issue_size in 2..l {
        for start_pointer in 0..l-issue_size {
            let mut m: Option<Tree> = None;
            let end_pointer = start_pointer + issue_size;
            for root_idx in (start_pointer + 1)..end_pointer {
                let first_half = forest[start_pointer][root_idx-1].as_ref().unwrap();
                let second_half = forest[root_idx+1][end_pointer].as_ref().unwrap();
                let tree = Tree::union_trees(
                    root_idx,
                    frequencies[root_idx],
                    Some(Rc::clone(first_half)),
                    Some(Rc::clone(second_half))
                );
                m = match m {
                    Some(mv) if mv.vertex_avg < tree.vertex_avg => Some(mv),
                    _ => Some(tree)
                };
            }

            forest[start_pointer][end_pointer] = Some(Rc::new(RefCell::new(m.unwrap())));
        }
    }

    Rc::clone(forest[0][l-1].as_ref().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = [0.05, 0.4, 0.08, 0.04, 0.1, 0.1, 0.23];
        let result = build_tree(&input);
        assert_eq!(result.borrow().vertex_avg, 2.19);
        assert_eq!(result.borrow().calculate_avg(), 2.19);
    }

    #[test]
    fn it_works2() {
        let input = [0.2, 0.05, 0.17, 0.1, 0.2, 0.03, 0.25];
        let result = build_tree(&input);
        assert_eq!(result.borrow().vertex_avg, 2.23);
        assert_eq!(result.borrow().calculate_avg(), 2.23);
    }
} 
