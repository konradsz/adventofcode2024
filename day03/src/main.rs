use regex::Regex;

fn part_1(input: &str) -> u64 {
    evaluate(input)
}

fn part_2(input: &str) -> u64 {
    let re = Regex::new(r"(?<part>.*?)(?<instruction>don't\(\)|do\(\))(?<rest>.*)").unwrap();

    let mut should_do = true;
    let mut res = 0;
    for line in input.lines() {
        let mut line = line.to_owned();
        while let Some(caps) = re.captures(&line) {
            if should_do {
                res += evaluate(&caps["part"]);
            }
            match &caps["instruction"] {
                "do()" => should_do = true,
                "don't()" => should_do = false,
                _ => panic!("invalid input"),
            };
            line = caps["rest"].to_owned();
        }

        if should_do {
            res += evaluate(&line);
        }
    }
    res
}

fn evaluate(input: &str) -> u64 {
    let re = Regex::new(r".*?mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let lhs = &caps["lhs"].parse::<u64>().unwrap();
            let rhs = &caps["rhs"].parse::<u64>().unwrap();
            lhs * rhs
        })
        .sum::<u64>()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    assert_eq!(part_1(&input), 180233229);
    assert_eq!(part_2(&input), 95411583);
}
