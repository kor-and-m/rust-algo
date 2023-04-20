fn merge(f: Vec<usize>, s: Vec<usize>) -> (Vec<usize>, usize) {
    let f_l = f.len();
    let s_l = s.len();
    let mut i = 0;
    let mut j = 0;
    let mut counter = 0;
    let mut sorted_arr = Vec::with_capacity(f_l + s_l);

    while f_l > i && s_l > j {
        if f[i] < s[j] {
            sorted_arr.push(f[i]);
            i += 1;
        } else {
            sorted_arr.push(s[j]);
            counter += f_l - i;
            j += 1;
        }
    }

    for k in i..f_l {
        sorted_arr.push(f[k]);
    }

    for k in j..s_l {
        sorted_arr.push(s[k]);
    }

    (sorted_arr, counter)
}

fn inversions_count_and_sort(n: &Vec<usize>) -> (Vec<usize>, usize) {
    let l = n.len();

    if l == 1 {
        return (n.to_vec(), 0);
    }

    let h = l/2;

    let f = &n[h..].to_vec();
    let s = &n[..h].to_vec();

    let (max_arr, r_count) = inversions_count_and_sort(f);
    let (min_arr, l_count) = inversions_count_and_sort(s);
    let (new_arr, m_count) = merge(min_arr, max_arr);
    (new_arr, m_count + l_count + r_count)
}

pub fn inversions_count(n: &Vec<usize>) -> usize {
    inversions_count_and_sort(n).1
}

#[cfg(test)]
mod tests {
    use crate::inversions_count;
    use std::fs;

    #[test]
    fn inversions_count_test() {
        let input = vec![1,3,5,2,4,6];
        let result = inversions_count(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn inversions_count_2_test() {
        let input = vec![1,3,5,4,2,6];
        let result = inversions_count(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn inversions_count_in_file_test() {
        let contents = fs::read_to_string("priv/data.txt")
          .expect("Something went wrong reading the file");
        let input: Vec<usize> = contents.split_whitespace().map(|x| x.parse::<i32>().unwrap() as usize).collect();
        let result = inversions_count(&input);
        assert_eq!(result, 2407905288);
    }
}