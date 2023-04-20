use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub trait Medianable {
    fn update_state(&mut self, num: usize);
    fn calculate_median(&self) -> f64;
}

pub struct TwoHeaps {
    top: BinaryHeap<isize>,
    bottom: BinaryHeap<isize>,
}

impl TwoHeaps {
    pub fn new() -> Self {
        TwoHeaps {
            top: BinaryHeap::new(),
            bottom: BinaryHeap::new(),
        }
    }
}

impl Medianable for TwoHeaps {
    fn update_state(&mut self, num: usize) {
        match (self.bottom.peek(), self.bottom.len(), self.top.len()) {
            (_, 0, 0) => self.bottom.push(num as isize),
            (Some(v), b_len, t_len) if *v < num as isize && b_len > t_len => {
                self.top.push(-(num as isize))
            }
            (Some(_), b_len, t_len) if b_len > t_len => {
                self.bottom.push(num as isize);
                let v = self.bottom.pop().unwrap();
                self.top.push(-v)
            }
            (_, b_len, t) if b_len == t => {
                let top_elem = -self.top.peek().unwrap();
                if top_elem >= num as isize {
                    self.bottom.push(num as isize);
                } else {
                    self.top.push(-(num as isize));
                    self.bottom.push(-self.top.pop().unwrap());
                }
            }
            (_, _, _) => unimplemented!(),
        }
    }

    fn calculate_median(&self) -> f64 {
        match (
            self.bottom.len(),
            self.top.len(),
            self.bottom.peek(),
            self.top.peek(),
        ) {
            (l1, l2, Some(v), _) if l1 >= l2 => *v as f64,
            (l1, l2, _, _) => panic!("unreachable case sizes is {} {}", l1, l2),
        }
    }
}

pub fn median_stream_heap(path: &str, state: &mut impl Medianable) -> f64 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut sum: f64 = 0.0;

    for line_res in reader.lines() {
        let num: usize = line_res.unwrap().parse().unwrap();
        state.update_state(num);
        sum += state.calculate_median();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut state = TwoHeaps::new();
        let result = median_stream_heap("input.txt", &mut state);
        assert_eq!(result, 46831213.0);
    }
}
