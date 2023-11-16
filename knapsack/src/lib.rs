use std::{cmp::{max, min}, collections::HashMap};

pub struct KanpsackItem {
    pub val: usize,
    pub weight: usize
}

impl KanpsackItem {
    pub fn new(val: usize, weight: usize) -> Self {
        Self { val, weight }
    }
}

pub fn knapsack(items: &[KanpsackItem], copasity: usize, size: usize) -> usize {
    let mut results = init_results(copasity);
    knapsack_step(items, copasity, size, size, &mut results)
}

fn knapsack_step(items: &[KanpsackItem], copasity: usize, size: usize, initial_size: usize, results: &mut HashMap<usize, Vec<usize>>) -> usize {
    if copasity == 0 {
        return 0
    }

    let copasity_idx = copasity - 1;

    let calc_deep = {
        let current_copasity_memory = results.entry(copasity_idx).or_insert(Vec::with_capacity(initial_size));
        let mut calc_deep = current_copasity_memory.len();

        if calc_deep == 0 {
          let item = &items[0];
          current_copasity_memory.push(if copasity >= item.weight {
            item.val
          } else {
            0
          });
          calc_deep += 1;
        }
        calc_deep
    };


    let mut last_res = results.get(&copasity_idx).unwrap()[min(calc_deep - 1, size - 1)];

    for idx in calc_deep..size {
        let item = &items[idx];
        let first = last_res;
        let seocnd = if copasity < item.weight {
            0
        } else {
            knapsack_step(items, copasity - item.weight, idx, initial_size, results) + item.val
        };

        last_res = max(first, seocnd);

        (*results.get_mut(&copasity_idx).unwrap()).push(last_res)
    }

    results.get(&copasity_idx).unwrap()[size - 1]
}

fn init_results(copasity: usize) -> HashMap<usize, Vec<usize>> {
    HashMap::with_capacity(copasity)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::{BufReader, Lines};
    use std::io::prelude::*;

    #[test]
    fn minimal_case() {
        let copasity = 6;
        let size = 4;
        let items = [
            KanpsackItem::new(3, 4),
            KanpsackItem::new(2, 3),
            KanpsackItem::new(4, 2),
            KanpsackItem::new(4, 3)
        ];
        assert_eq!(knapsack(&items, copasity, size), 8);
    }

    #[test]
    fn it_works() {
        let file =
          File::open("priv/small_knapsack.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();

        let head_line: Vec<usize> = lines.next().unwrap().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();

        let copasity = head_line[0];
        let size = head_line[1];

        let mut items: Vec<KanpsackItem> = Vec::with_capacity(size);

        for line_res in lines {
            if let Ok(line) = line_res {
                let parsed_line: Vec<usize> = line.split(" ").map(|x| x.parse().unwrap()).collect();
                items.push(KanpsackItem::new(parsed_line[0], parsed_line[1]))
            }
        }

        assert_eq!(knapsack(&items, copasity, size), 2_493_893);
    }

    #[test]
    fn it_works_big() {
        let file =
          File::open("priv/knapsack_big.txt").expect("Something went wrong reading the file");

        let reader = BufReader::new(file);
        let mut lines: Lines<BufReader<File>> = reader.lines();

        let head_line: Vec<usize> = lines.next().unwrap().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();

        let copasity = head_line[0];
        let size = head_line[1];

        let mut items: Vec<KanpsackItem> = Vec::with_capacity(size);

        for line_res in lines {
            if let Ok(line) = line_res {
                let parsed_line: Vec<usize> = line.split(" ").map(|x| x.parse().unwrap()).collect();
                items.push(KanpsackItem::new(parsed_line[0], parsed_line[1]))
            }
        }

        assert_eq!(knapsack(&items, copasity, size), 4_243_395);
    }
}
