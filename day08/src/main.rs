use std::collections::{HashMap, HashSet};

fn part_1(antennas: &HashMap<char, Vec<(usize, usize)>>, height: i32, width: i32) -> usize {
    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        for (i, p1) in positions.iter().enumerate() {
            for p2 in positions.iter().skip(i + 1) {
                let v = (p1.0 as i32 - p2.0 as i32, p1.1 as i32 - p2.1 as i32);

                if p1.0 as i32 + v.0 >= 0
                    && p1.1 as i32 + v.1 >= 0
                    && p1.0 as i32 + v.0 < width
                    && p1.1 as i32 + v.1 < height
                {
                    antinodes.insert((p1.0 as i32 + v.0, p1.1 as i32 + v.1));
                }

                if p2.0 as i32 - v.0 >= 0
                    && p2.1 as i32 - v.1 >= 0
                    && p2.0 as i32 - v.0 < width
                    && p2.1 as i32 - v.1 < height
                {
                    antinodes.insert((p2.0 as i32 - v.0, p2.1 as i32 - v.1));
                }
            }
        }
    }

    antinodes.len()
}

fn part_2(antennas: &HashMap<char, Vec<(usize, usize)>>, height: i32, width: i32) -> usize {
    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        for (i, p1) in positions.iter().enumerate() {
            for p2 in positions.iter().skip(i + 1) {
                let v = (p1.0 as i32 - p2.0 as i32, p1.1 as i32 - p2.1 as i32);

                let mut multiply = 0;
                while p1.0 as i32 + (multiply * v.0) >= 0
                    && p1.1 as i32 + (multiply * v.1) >= 0
                    && p1.0 as i32 + (multiply * v.0) < width
                    && p1.1 as i32 + (multiply * v.1) < height
                {
                    antinodes.insert((
                        p1.0 as i32 + (multiply * v.0),
                        p1.1 as i32 + (multiply * v.1),
                    ));
                    multiply += 1;
                }

                let mut multiply = 0;
                while p2.0 as i32 - (multiply * v.0) >= 0
                    && p2.1 as i32 - (multiply * v.1) >= 0
                    && p2.0 as i32 - (multiply * v.0) < width
                    && p2.1 as i32 - (multiply * v.1) < height
                {
                    antinodes.insert((
                        p2.0 as i32 - (multiply * v.0),
                        p2.1 as i32 - (multiply * v.1),
                    ));
                    multiply += 1;
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;
    for (y, line) in input.lines().enumerate() {
        for (x, field) in line.chars().enumerate() {
            if field == '.' {
                continue;
            }

            antennas
                .entry(field)
                .and_modify(|positions| positions.push((x, y)))
                .or_insert(vec![(x, y)]);
        }
    }

    assert_eq!(part_1(&antennas, height, width), 291);
    assert_eq!(part_2(&antennas, height, width), 1015);
}
