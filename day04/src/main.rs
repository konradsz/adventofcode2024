fn part_1(input: &[Vec<char>]) -> usize {
    let mut total = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'X' {
                total += count_xmases(input, x, y);
            }
        }
    }

    total
}

fn part_2(input: &[Vec<char>]) -> usize {
    let mut total = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            if input[y][x] == 'A' {
                total += count_mases(input, x, y);
            }
        }
    }

    total
}

fn count_xmases(input: &[Vec<char>], x: usize, y: usize) -> usize {
    let dirs = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let letters = ['X', 'M', 'A', 'S'];
    let mut count = 0;
    'd: for (dx, dy) in dirs {
        let (mut pos_x, mut pos_y) = (x as i32, y as i32);
        for letter in letters.into_iter().skip(1) {
            pos_x += dx;
            pos_y += dy;
            if pos_x < 0
                || pos_x >= input[0].len() as i32
                || pos_y < 0
                || pos_y >= input.len() as i32
            {
                continue 'd;
            }
            if input[pos_y as usize][pos_x as usize] != letter {
                continue 'd;
            }
        }
        count += 1;
    }
    count
}

fn count_mases(input: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut total = 0;
    if ((input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S')
        || (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M'))
        && ((input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S')
            || (input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M'))
    {
        total += 1;
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    assert_eq!(part_1(&input), 2654);
    assert_eq!(part_2(&input), 1990);
}
