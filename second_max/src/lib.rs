#[macro_use]
extern crate more_asserts;
use core::hash::Hash;
use std::{cmp::max, collections::HashMap};

fn split_array<T: Copy + Ord + std::fmt::Debug + Hash>(
    n: Vec<T>,
    history: &mut HashMap<T, T>,
    counter: &mut usize,
) -> Vec<T> {
    let l = n.len();
    let mut max_arr = Vec::with_capacity(l / 2);

    for i in 0..(l / 2) {
        *counter += 1;
        let f = n[i * 2];
        let s = n[i * 2 + 1];
        let (max_elem, min_elem) = if f < s { (s, f) } else { (f, s) };
        max_arr.push(max_elem);
        history.insert(max_elem, min_elem);
    }

    max_arr
}

fn find_max_pair<T: Copy + Ord + std::fmt::Debug + Hash>(n: Vec<T>, counter: &mut usize) -> (T, T) {
    let l = n.len();
    let mut history = HashMap::new();

    if l == 2 {
        return if n[0] > n[1] {
            (n[0], n[1])
        } else {
            (n[1], n[0])
        };
    }

    let max_arr = split_array(n, &mut history, counter);

    let (the_laggest_val, second_laggest) = find_max_pair(max_arr, counter);
    let second = *history.get(&the_laggest_val).unwrap();

    *counter += 1;
    (the_laggest_val, max(second_laggest, second))
}

pub fn find_second_max_safe<T: Copy + Ord + std::fmt::Debug + Hash>(
    n: Vec<T>,
) -> (Option<T>, usize) {
    let mut counter: usize = 0;
    if n.len() < 2 {
        (None, counter)
    } else {
        (Some(find_max_pair(n, &mut counter).1), counter)
    }
}

#[cfg(test)]
mod tests {
    use crate::find_second_max_safe;

    #[test]
    fn find_second_max_regular() {
        let input = vec![1, 6, 9, 3, 8, 3, 12, 2];
        let (res, time) = find_second_max_safe(input);
        assert_eq!(res, Some(9));
        assert_le!(time, (8 + 3 - 2));
    }

    #[test]
    fn find_second_max_big_input() {
        let input = vec![
            1, 6, 9, 3, 8, 3, 12, 2, 1, 6, 9, 3, 8, 3, 12, 2, 1, 6, 9, 3, 8, 3, 12, 2, 1, 6, 9, 3,
            8, 3, 12, 2, 1, 6, 9, 3, 8, 3, 12, 2, 1, 6, 9, 3, 8, 3, 12, 2, 1, 6, 9, 3, 8, 3, 12, 2,
            1, 6, 9, 3, 8, 3, 12, 2,
        ];
        let (res, time) = find_second_max_safe(input);
        assert_eq!(res, Some(12));
        assert_le!(time, (64 + 6 - 2));
    }
}
