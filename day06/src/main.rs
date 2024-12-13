use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part_1(map: &[Vec<char>], mut guard_postition: (i32, i32)) -> usize {
    let mut guard_direction = Direction::Up;
    let mut visited_positions = HashSet::new();
    loop {
        visited_positions.insert(guard_postition);
        let new_pos = match guard_direction {
            Direction::Up => (guard_postition.0, guard_postition.1 - 1),
            Direction::Right => (guard_postition.0 + 1, guard_postition.1),
            Direction::Down => (guard_postition.0, guard_postition.1 + 1),
            Direction::Left => (guard_postition.0 - 1, guard_postition.1),
        };
        if new_pos.0 < 0
            || new_pos.0 >= map[0].len() as i32
            || new_pos.1 < 0
            || new_pos.1 >= map.len() as i32
        {
            break;
        }

        if map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            guard_direction = match guard_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            guard_postition = new_pos;
        }
    }

    visited_positions.len()
}

fn part_2(map: &[Vec<char>], guard_position: (i32, i32)) -> usize {
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '.' {
                continue;
            }
            let mut guard_position = guard_position;
            let mut map = map.to_owned();
            map[y][x] = '#';

            let mut guard_direction = Direction::Up;
            let mut states = HashSet::new();
            loop {
                if !states.insert((guard_position, guard_direction)) {
                    result += 1;
                    break;
                }
                let new_pos = match guard_direction {
                    Direction::Up => (guard_position.0, guard_position.1 - 1),
                    Direction::Right => (guard_position.0 + 1, guard_position.1),
                    Direction::Down => (guard_position.0, guard_position.1 + 1),
                    Direction::Left => (guard_position.0 - 1, guard_position.1),
                };
                if new_pos.0 < 0
                    || new_pos.0 >= map[0].len() as i32
                    || new_pos.1 < 0
                    || new_pos.1 >= map.len() as i32
                {
                    break;
                }

                if map[new_pos.1 as usize][new_pos.0 as usize] == '#' {
                    guard_direction = match guard_direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    };
                } else {
                    guard_position = new_pos;
                }
            }
        }
    }

    result
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut map = Vec::new();

    let mut guard_postition = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, field) in line.chars().enumerate() {
            row.push(match field {
                '.' => '.',
                '^' => {
                    guard_postition = (x as i32, y as i32);
                    '.'
                }
                '#' => '#',
                _ => panic!("invalid input"),
            });
        }
        map.push(row);
    }

    assert_eq!(part_1(&map, guard_postition), 4647);
    assert_eq!(part_2(&map, guard_postition), 1723);
}
