use std::cmp::PartialOrd;
use core::fmt::Debug;
use std::cmp::Ordering::{Less, Greater, Equal};

// fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
//     for i in 0..arr.len() {
//         let mut j = i;
//         while j > 0 && arr[j - 1] > arr[j] {
//             arr.swap(j - 1, j);
//             j -= 1;
//         }
//     }
// }

fn shafle_arr<T: Copy + Debug, Lt>(arr: &mut [T], lt: &Lt, pivot_idx: usize) -> usize
    where Lt: Fn(&T, &T) -> bool {
    arr.swap(0, pivot_idx);
    let pivot = arr[0];

    let mut j = 1;
    for i in 1..arr.len() {
        if lt(&arr[i], &pivot)  {
            arr.swap(i, j);
            j += 1;
        }
    }

    arr.swap(0, j - 1);
    j
}

pub fn sort_n_by<T: Copy + Debug, Lt, F2>(arr: &mut [T], n: usize, lt: &Lt, choice_pivot: &F2) -> T
    where Lt: Fn(&T, &T) -> bool,
          F2: Fn(&mut [T]) -> usize {

    let len = arr.len();
    if len == 1 && n == 0 {
        return arr[0];
    }

    let initial_pivot = choice_pivot(arr);
    let pivot_idx = shafle_arr(arr, lt, initial_pivot) - 1;

    match n.cmp(&(pivot_idx)) {
        Equal => arr[pivot_idx],
        Greater => sort_n_by(&mut arr[pivot_idx+1..len], n - pivot_idx - 1, lt, choice_pivot),
        Less => sort_n_by(&mut arr[0..pivot_idx], pivot_idx - n - 1, lt, choice_pivot)
    }
}

pub fn sort_n<T, F2>(arr: &mut [T], n: usize, choice_pivot: &F2) -> T
    where T: PartialOrd + Copy + Debug,
    F2: Fn(&mut [T]) -> usize {
    sort_n_by(arr, n, &|a: &T, b: &T| a.lt(b), choice_pivot)
}

pub fn first(_v: &mut [usize]) -> usize {
    0
}

// pub fn median_of_medians(v: &mut [usize], group_size: usize) -> usize {
//     let parts = v.len() / group_size;
//     let mut medians: Vec<usize> = Vec::new();
//     let m = v.len() % group_size;

//     for p in 0..parts {
//         insertion_sort(&mut v[p * group_size..(p + 1) * group_size]);
//         medians.push(v[group_size * p + group_size / 2])
//     }

//     if m > 0 {
//         insertion_sort(&mut v[parts * group_size..parts * group_size + m - 1]);
//         medians.push(v[parts * group_size + m / 2]);    
//     }
    
//     let median_elem = medians.len() / 2;
//     sort_n(&mut medians[..], median_elem, &first)
// }


#[cfg(test)]
mod tests {
    // use crate::median_of_medians;

    use super::{sort_n, first};

    #[test]
    fn it_works() {
        let mut v = [3, 2, 1, 4, 5];
        let r = sort_n(&mut v, 3, &first);
        assert!(r == 4);
        let r2 = sort_n(&mut v, 0, &first);
        assert!(r2 == 1);
    }

    // #[test]
    // fn it_works_median() {
    //     let mut v = [3, 2, 1, 4, 5];
    //     let r = sort_n(&mut v, 3, &|x| median_of_medians(x, 5));
    //     assert!(r == 4);
    // }

    // #[test]
    // fn it_works_median_2() {
    //     let mut v = [3, 6, 8, 4, 5, 7, 1];
    //     let r = sort_n(&mut v, 5, &|x| median_of_medians(x, 5));
    //     assert!(r == 7);
    // }
}
