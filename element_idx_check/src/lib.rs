use std::cmp::Ordering;

pub fn is_elem_idx_exists(n: &[i32], padding: i32) -> bool {
    let l = n.len();
    
    match l {
        1 => return n[0] == padding,
        _ => ()
    }

    let h = l/2;
    let current_padding = h as i32 + padding;
    println!("{} {}", &current_padding, &n[h]);

    match n[h].cmp(&current_padding) {
        Ordering::Less => is_elem_idx_exists(&n[h..], current_padding),
        Ordering::Greater => is_elem_idx_exists(&n[..h], padding),
        Ordering::Equal => true
    }
}

#[cfg(test)]
mod tests {
    use crate::is_elem_idx_exists;

    #[test]
    fn is_elem_idx_exists_test() {
        let arr = [-3, -1, 2, 7, 9];
        let result = is_elem_idx_exists(&arr, 0);
        assert!(result);
    }

    #[test]
    fn is_elem_idx_exists_false_test() {
        let arr = [-3, -1, 3, 7, 9];
        let result = is_elem_idx_exists(&arr, 0);
        assert!(!result);
    }

    #[test]
    fn is_elem_idx_exists_last_test() {
        let arr = [-5, -4, -2, -1, 4];
        let result = is_elem_idx_exists(&arr, 0);
        assert!(result);
    }

    #[test]
    fn is_elem_idx_exists_even_test() {
        let arr = [-5, -4, -2, -1, 1, 5];
        let result = is_elem_idx_exists(&arr, 0);
        assert!(result);
    }

    #[test]
    fn is_elem_idx_exists_small_test() {
        let arr = [-1, 1];
        let result = is_elem_idx_exists(&arr, 0);
        assert!(result);
    }

    #[test]
    fn is_elem_idx_exists_first_test() {
        let arr = [0, 2, 5, 25, 100, 1000, 1001];
        let result = is_elem_idx_exists(&arr, 0);
        assert!(result);
    }
}

