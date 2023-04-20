use core::fmt::Debug;
use std::cmp::PartialOrd;

fn shafle_arr<T: Copy + Debug, Lt>(arr: &mut [T], lt: &Lt, pivot_idx: usize) -> usize
where
    Lt: Fn(&T, &T) -> bool,
{
    arr.swap(0, pivot_idx);
    let pivot = arr[0];

    let mut j = 1;
    for i in 1..arr.len() {
        if lt(&arr[i], &pivot) {
            arr.swap(i, j);
            j += 1;
        }
    }

    arr.swap(0, j - 1);
    j
}

pub fn quicksort_by<T: Copy + Debug, Lt, F2>(arr: &mut [T], lt: &Lt, choice_pivot: &F2) -> usize
where
    Lt: Fn(&T, &T) -> bool,
    F2: Fn(&[T]) -> usize,
{
    let len = arr.len();
    if len <= 1 {
        return 0;
    }

    let j = shafle_arr(arr, lt, choice_pivot(arr));

    let mut sum = len - 1;

    sum += quicksort_by(&mut arr[0..j - 1], lt, choice_pivot);
    sum += quicksort_by(&mut arr[j..len], lt, choice_pivot);
    sum
}

pub fn quicksort<T, F2>(arr: &mut [T], choice_pivot: &F2) -> usize
where
    T: PartialOrd + Copy + Debug,
    F2: Fn(&[T]) -> usize,
{
    quicksort_by(arr, &|a: &T, b: &T| a.lt(b), choice_pivot)
}

pub fn median(v: &[i32]) -> usize {
    let l = v[v.len() - 1];
    let f = v[0];
    let m = v[(v.len() - 1) / 2];
    match (l > f, l < m, f > m) {
        (true, true, _) => v.len() - 1,
        (false, false, _) => v.len() - 1,
        (true, false, true) => 0,
        (false, true, false) => 0,
        _ => (v.len() - 1) / 2,
    }
}

pub fn first(_v: &[i32]) -> usize {
    0
}

pub fn last(v: &[i32]) -> usize {
    v.len() - 1
}

#[cfg(test)]
mod tests {
    use super::{first, last, median, quicksort};
    use std::fs;

    fn check(v: &[i32], v2: &[i32]) {
        for i in 0..v2.len() {
            assert!(v[i] == v2[i], "{:?}", v);
        }
    }

    #[test]
    fn it_works() {
        let mut v = [3, 2, 1, 4, 5];
        quicksort(&mut v, &first);
        let v2 = [1, 2, 3, 4, 5];
        check(&v, &v2);
    }

    #[test]
    fn it_works_example() {
        let mut v = [3, 8, 2, 5, 1, 4, 7, 6];
        quicksort(&mut v, &first);
        let v2 = [1, 2, 3, 4, 5, 6, 7, 8];
        check(&v, &v2);
    }

    #[test]
    fn it_works_duble() {
        let mut v = [3, 7, 1, 3, 5];
        quicksort(&mut v, &first);
        let v2 = [1, 3, 3, 5, 7];
        check(&v, &v2)
    }

    #[test]
    fn it_works_1() {
        let mut v = [78, 27, 76, 61, 45, 86, 75, 38, 72, 59, 54, 71, 38, 4, 52];
        let mut v2 = v.clone();
        quicksort(&mut v, &first);
        v2.sort();
        check(&v, &v2)
    }

    #[test]
    fn it_works_2() {
        let mut v = [
            42, 80, 92, 19, 26, 23, 42, 62, 32, 23, 95, 1, 1, 88, 17, 37, 3, 24, 95, 53,
        ];
        let mut v2 = v.clone();
        quicksort(&mut v, &first);
        v2.sort();
        check(&v, &v2)
    }

    #[test]
    fn it_works_file_median() {
        let contents =
            fs::read_to_string("priv/data.txt").expect("Something went wrong reading the file");
        let mut v: Vec<i32> = contents
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut v2 = v.clone();
        let e = quicksort(&mut v, &median);
        assert!(e == 138382, "{}", e);
        v2.sort();
        check(&v, &v2)
    }

    #[test]
    fn it_works_file_first() {
        let contents =
            fs::read_to_string("priv/data.txt").expect("Something went wrong reading the file");
        let mut v: Vec<i32> = contents
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut v2 = v.clone();
        let e = quicksort(&mut v, &first);
        assert!(e == 162085, "{}", e);
        v2.sort();
        check(&v, &v2)
    }

    #[test]
    fn it_works_file_last() {
        let contents =
            fs::read_to_string("priv/data.txt").expect("Something went wrong reading the file");
        let mut v: Vec<i32> = contents
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut v2 = v.clone();
        let e = quicksort(&mut v, &last);
        assert!(e == 164123, "{}", e);
        v2.sort();
        check(&v, &v2)
    }
}
