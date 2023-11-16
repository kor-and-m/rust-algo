use std::cmp::max;

pub fn find_wis(weights: &Vec<usize>, veights_count: usize) -> (usize, Vec<usize>) {
    let mut acc = (0, weights[0]);
    let mut results = [vec![], vec![0]];
    let mut idx = 1;

    for i in 1..veights_count {
        let current_weight = max(acc.1, acc.0 + weights[i]);
        if acc.1 < acc.0 + weights[i] {
            idx = (idx + 1) % 2;
            results[idx].push(i);
        } else {
            results[(idx + 1) % 2] = results[idx].clone();
            idx = (idx + 1) % 2;
        }
        acc = (acc.1, current_weight);
    }

    (acc.1, results[idx].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{BufReader, Lines};

    #[test]
    fn it_works() {
        let file =
            File::open("priv/graph_path.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();

        let line_count: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let mut weights: Vec<usize> = Vec::with_capacity(line_count);

        for line_res in lines {
            if let Ok(line) = line_res {
                weights.push(line.parse().unwrap())
            }
        }

        let res = find_wis(&weights, line_count);
        assert_eq!(res.0, 2955353732);

        let sum: usize = res.1.iter().map(|x| weights[*x]).sum();
        assert_eq!(sum, 2955353732);

        let task_input = [1, 2, 3, 4, 17, 117, 517, 997];
        let task_result: Vec<u8> = task_input
            .into_iter()
            .map(|x| x - 1)
            .map(|x| res.1.contains(&x) as u8)
            .collect();

        assert_eq!(task_result, vec![1, 0, 1, 0, 0, 1, 1, 0]);
    }
}
