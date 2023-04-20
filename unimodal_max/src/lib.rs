use std::cmp::{max, Ordering};
use std::fmt::Debug;

pub fn unimodal_max<T: Copy + Ord + Debug>(n: &[T]) -> T {
    let l = n.len();

    match l {
        2 => return max(n[0], n[1]),
        1 => return n[0],
        _ => (),
    }

    let h = l / 2;

    match n[h - 1].cmp(&n[h]) {
        Ordering::Less => unimodal_max(&n[h..]),
        Ordering::Greater => unimodal_max(&n[..h - 1]),
        Ordering::Equal => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::unimodal_max;

    #[test]
    fn unimodal_max_test() {
        let arr = [1, 2, 4, 7, 6, 5, 3];
        let result = unimodal_max(&arr);
        assert_eq!(result, 7);
    }

    #[test]
    fn unimodal_max_increase_test() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 9, 8];
        let result = unimodal_max(&arr);
        assert_eq!(result, 9);
    }

    #[test]
    fn unimodal_max_last_test() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 12];
        let result = unimodal_max(&arr);
        assert_eq!(result, 12);
    }

    #[test]
    fn unimodal_max_first_test() {
        let arr = [1];
        let result = unimodal_max(&arr);
        assert_eq!(result, 1);
    }
}
