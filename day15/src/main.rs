#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Box,
}

fn try_to_move(map: &mut Vec<Vec<Tile>>, (pos_x, pos_y): (i32, i32), dir: (i32, i32)) -> bool {
    let (new_x, new_y) = ((pos_x + dir.0) as usize, (pos_y + dir.1) as usize);
    match map[new_y][new_x] {
        Tile::Empty => {
            map[new_y][new_x] = map[pos_y as usize][pos_x as usize];
            map[pos_y as usize][pos_x as usize] = Tile::Empty;
            true
        }
        Tile::Box => {
            if try_to_move(map, (new_x as i32, new_y as i32), dir) {
                map[new_y][new_x] = map[pos_y as usize][pos_x as usize];
                map[pos_y as usize][pos_x as usize] = Tile::Empty;
                true
            } else {
                false
            }
        }
        Tile::Wall => false,
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (map_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut map = Vec::new();
    let mut position = (0, 0);
    for (y, line) in map_input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'O' => Tile::Box,
                '@' => {
                    position = (x as i32, y as i32);
                    Tile::Empty
                }
                _ => panic!("invalid input"),
            });
        }
        map.push(row);
    }

    let moves_input = moves_input.trim();
    moves_input.chars().for_each(|c| {
        let dir = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        if try_to_move(&mut map, position, dir) {
            position.0 += dir.0;
            position.1 += dir.1;
        }
    });

    let sum = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| tile == &&Tile::Box)
                .map(|(x, _)| 100 * y + x)
                .sum::<usize>()
        })
        .sum::<usize>();

    assert_eq!(sum, 1430536);
}
