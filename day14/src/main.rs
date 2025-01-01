use std::io::Write;

#[allow(unused)]
fn print_map(width: i32, height: i32, robots: &[Robot], file: &mut std::fs::File) {
    for y in 0..height {
        for x in 0..width {
            if robots.iter().any(|r| r.x == x && r.y == y) {
                write!(file, "*").unwrap();
            } else {
                write!(file, ".").unwrap();
            }
        }
        writeln!(file).unwrap();
    }
}

#[derive(Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut robots = input
        .lines()
        .map(|line| {
            let mut parts = line.split(&['=', ',', ' ']);
            let (_, x, y, _, vx, vy) = (
                parts.next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );
            Robot { x, y, vx, vy }
        })
        .collect::<Vec<_>>();

    let width = 101;
    let height = 103;
    let quadrant_width = width / 2;
    let quadrant_height = height / 2;

    // For part 2
    // let mut f = std::fs::File::create("maps.txt").unwrap();

    let seconds = 100;
    for _s in 1..=seconds {
        for robot in robots.iter_mut() {
            robot.x += robot.vx;
            robot.y += robot.vy;

            robot.x %= width;
            robot.y %= height;
            if robot.x < 0 {
                robot.x += width;
            }
            if robot.y < 0 {
                robot.y += height;
            }
        }

        /* Part 2 machinery
        if s >= 70 {
            if (s - 70) % 101 == 0 {
                let ss = format!("after {s}s");
                writeln!(&mut f, "{}", ss).unwrap();
                print_map(width, height, &robots, &mut f);
            }
        }
        */
    }

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    for robot in robots {
        if robot.x < quadrant_width && robot.y < quadrant_height {
            top_left += 1;
        } else if robot.x > quadrant_width && robot.y < quadrant_height {
            top_right += 1;
        } else if robot.x < quadrant_width && robot.y > quadrant_height {
            bottom_left += 1;
        } else if robot.x > quadrant_width && robot.y > quadrant_height {
            bottom_right += 1;
        }
    }

    let safety_factor = top_left * top_right * bottom_left * bottom_right;
    assert_eq!(safety_factor, 215987200);
}
