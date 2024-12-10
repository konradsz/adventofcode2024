fn part_1(reports: &[Vec<u64>]) -> usize {
    reports.iter().filter(is_safe).count()
}

fn part_2(reports: &[Vec<u64>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut fixed_report = (**report).clone();
                fixed_report.remove(i);
                is_safe(&&fixed_report)
            })
        })
        .count()
}

fn is_safe(report: &&Vec<u64>) -> bool {
    enum Direction {
        Increasing,
        Decreasing,
    }

    let mut direction = None;

    report.iter().zip(report.iter().skip(1)).all(|(lhs, rhs)| {
        let diff = *lhs as i64 - *rhs as i64;
        if diff.abs() < 1 || diff.abs() > 3 {
            false
        } else if diff > 0 {
            if let Some(dir) = &direction {
                matches!(dir, Direction::Decreasing)
            } else {
                direction = Some(Direction::Decreasing);
                true
            }
        } else if let Some(dir) = &direction {
            matches!(dir, Direction::Increasing)
        } else {
            direction = Some(Direction::Increasing);
            true
        }
    })
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reports = input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&reports), 686);
    assert_eq!(part_2(&reports), 717);
}
