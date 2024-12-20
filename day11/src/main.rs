fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut stones = input
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    let blinks = 20;
    let mut previous = 0;
    for i in 0..blinks {
        let mut new_arragement = Vec::new();
        for stone in stones.iter() {
            if stone == "0" {
                new_arragement.push("1".to_string());
            } else if stone.len() % 2 == 0 {
                let (left, right) = stone.split_at(stone.len() / 2);
                let mut right = right.trim_start_matches('0');
                if right.is_empty() {
                    right = "0";
                }
                new_arragement.push(left.to_string());
                new_arragement.push(right.to_string());
            } else {
                let new_stone = (stone.parse::<u64>().unwrap() * 2024).to_string();
                new_arragement.push(new_stone);
            }
        }
        stones = new_arragement;
        // for s in stones.iter() {
        //     print!("{s} ");
        // }
        // println!();

        println!(
            "blink {} -> {} (+{})",
            i + 1,
            stones.len(),
            stones.len() - previous
        );
        previous = stones.len();
    }

    println!("{}", stones.len());
}
