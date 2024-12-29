use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut to_assign = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            to_assign.insert((x as i32, y as i32));
        }
    }
    let map_coordinates = to_assign.clone();

    let mut regions = Vec::new();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while !to_assign.is_empty() {
        let mut current_region = Vec::new();
        let el = to_assign.iter().next().cloned().unwrap();
        current_region.push(el);
        to_assign.remove(&el);
        let current_plant = map[el.1 as usize][el.0 as usize];

        let mut to_visit = VecDeque::new();
        to_visit.push_back(el);
        while let Some((x, y)) = to_visit.pop_front() {
            for (dx, dy) in directions {
                let new_pos = (x + dx, y + dy);
                if to_assign.contains(&new_pos)
                    && map[new_pos.1 as usize][new_pos.0 as usize] == current_plant
                {
                    current_region.push(new_pos);
                    to_assign.remove(&new_pos);
                    to_visit.push_back(new_pos);
                }
            }
        }
        regions.push(current_region);
    }

    let mut sum = 0;
    for region in regions {
        let area = region.len();
        let mut perimeter = 0;
        for field in region {
            let plant = map[field.1 as usize][field.0 as usize];
            for dir in directions {
                let new_pos = (field.0 + dir.0, field.1 + dir.1);
                if map_coordinates.contains(&new_pos) {
                    if map[new_pos.1 as usize][new_pos.0 as usize] != plant {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }

        sum += perimeter * area;
    }

    println!("{sum}");
}
