use std::collections::{HashSet, VecDeque};

fn calculate(
    map: &[Vec<u32>],
    map_width: i32,
    map_height: i32,
    trailheads: &[(i32, i32)],
    count_distincts: bool,
) -> usize {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut total_score = 0;
    for th in trailheads {
        let mut score = 0;
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        to_visit.push_back(*th);

        while !to_visit.is_empty() {
            let pos = to_visit.pop_front().unwrap();
            let height = map[pos.1 as usize][pos.0 as usize];

            if height == 9 && (count_distincts || !visited.contains(&pos)) {
                score += 1;
                visited.insert(pos);
                continue;
            }

            for dir in directions {
                let new_x = pos.0 + dir.0;
                let new_y = pos.1 + dir.1;
                if new_x < 0 || new_x >= map_width || new_y < 0 || new_y >= map_height {
                    continue;
                }
                if map[new_y as usize][new_x as usize] == height + 1 {
                    to_visit.push_back((new_x, new_y));
                }
            }
        }

        total_score += score;
    }

    total_score
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut map = Vec::new();
    let mut trailheads = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, d) in line.chars().enumerate() {
            let height = d.to_digit(10).unwrap();
            row.push(height);

            if height == 0 {
                trailheads.push((x as i32, y as i32));
            }
        }
        map.push(row);
    }
    let map_height = map.len() as i32;
    let map_width = map[0].len() as i32;

    let part_1 = calculate(&map, map_width, map_height, &trailheads, false);
    assert_eq!(part_1, 574);
    let part_2 = calculate(&map, map_width, map_height, &trailheads, true);
    assert_eq!(part_2, 1238);
}
