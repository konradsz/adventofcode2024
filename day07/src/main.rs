fn is_valid_part_1(target: u64, current: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        target == current
    } else if current > target {
        false
    } else {
        is_valid_part_1(target, current + numbers[0], &numbers[1..])
            || is_valid_part_1(target, current * numbers[0], &numbers[1..])
    }
}

fn is_valid_part_2(target: u64, current: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        target == current
    } else if current > target {
        false
    } else {
        is_valid_part_2(target, current + numbers[0], &numbers[1..])
            || is_valid_part_2(target, current * numbers[0], &numbers[1..])
            || is_valid_part_2(
                target,
                format!("{}{}", current, numbers[0]).parse().unwrap(),
                &numbers[1..],
            )
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input
        .lines()
        .map(|l| {
            let (result, numbers) = l.split_once(": ").unwrap();
            (
                result.parse::<u64>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let part_1 = input
        .iter()
        .filter(|inp| is_valid_part_1(inp.0, 0, &inp.1))
        .map(|inp| inp.0)
        .sum::<u64>();
    assert_eq!(part_1, 7579994664753);

    let part_2 = input
        .iter()
        .filter(|inp| is_valid_part_2(inp.0, 0, &inp.1))
        .map(|inp| inp.0)
        .sum::<u64>();
    assert_eq!(part_2, 438027111276610);
}
