use std::collections::HashMap;

fn part_1(left_list: &[i64], right_list: &[i64]) -> u64 {
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum()
}

fn part_2(left_list: &[i64], right_list: &[i64]) -> i64 {
    let mut counts = HashMap::new();
    right_list.iter().for_each(|el| {
        counts.entry(*el).and_modify(|c| *c += 1).or_insert(1);
    });
    left_list
        .iter()
        .map(|el| el * counts.get(el).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (mut left_list, mut right_list) = input
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace();
            (
                numbers.next().unwrap().parse::<i64>().unwrap(),
                numbers.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<(Vec<i64>, Vec<i64>)>();

    left_list.sort_unstable();
    right_list.sort_unstable();

    assert_eq!(part_1(&left_list, &right_list), 2164381);
    assert_eq!(part_2(&left_list, &right_list), 20719933);
}
