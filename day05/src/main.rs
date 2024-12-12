use std::collections::HashSet;

fn part_1(rules: &HashSet<(u32, u32)>, updates: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    'outer: for update in updates {
        for i in 0..update.len() {
            let numbers_after = &update[i + 1..];
            for n in numbers_after {
                if rules.contains(&(*n, update[i])) {
                    continue 'outer;
                }
            }
        }
        sum += update[update.len() / 2];
    }

    sum
}

fn part_2(rules: &HashSet<(u32, u32)>, updates: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for mut update in updates {
        let mut update_incorrect = false;
        'outer: loop {
            for i in 0..update.len() {
                let numbers_after = &update[i + 1..];
                for (j, n) in numbers_after.iter().enumerate() {
                    if rules.contains(&(*n, update[i])) {
                        update.swap(i, i + 1 + j);
                        update_incorrect = true;
                        continue 'outer;
                    }
                }
            }
            break;
        }
        if update_incorrect {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();
    let rules = rules_input
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once('|').unwrap();
            (lhs.parse::<u32>().unwrap(), rhs.parse::<u32>().unwrap())
        })
        .collect();
    let updates = updates_input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    assert_eq!(part_1(&rules, &updates), 5087);
    assert_eq!(part_2(&rules, updates), 4971);
}
