use std::collections::HashSet;

use regex::Regex;

fn calculate_min_price(
    target: (u32, u32),
    current: (u32, u32),
    button_a: (u32, u32),
    button_b: (u32, u32),
    spent: u32,
    cheapest: &mut Option<u32>,
    seen_states: &mut HashSet<((u32, u32), u32)>,
) {
    if spent >= cheapest.unwrap_or(u32::MAX) {
        return;
    }

    if current == target {
        *cheapest = Some(spent.min(cheapest.unwrap_or(u32::MAX)));
        return;
    }

    if current.0 >= target.0 || current.1 >= target.1 {
        return;
    }

    let moves = [
        ((current.0 + button_a.0, current.1 + button_a.1), spent + 3),
        ((current.0 + button_b.0, current.1 + button_b.1), spent + 1),
    ];

    for (new_position, new_spent) in moves {
        if seen_states.insert((new_position, new_spent)) {
            calculate_min_price(
                target,
                new_position,
                button_a,
                button_b,
                new_spent,
                cheapest,
                seen_states,
            );
        }
    }
}

fn main() {
    let re = Regex::new(
        "Button A: X\\+(?<ax>\\d+), Y\\+(?<ay>\\d+)\\n\
             Button B: X\\+(?<bx>\\d+), Y\\+(?<by>\\d+)\\n\
             Prize: X=(?<px>\\d+), Y=(?<py>\\d+)",
    )
    .unwrap();
    let input = std::fs::read_to_string("input").unwrap();
    let mut sum = 0;
    for machine in input.split("\n\n") {
        if let Some(c) = re.captures(machine) {
            let ax = c["ax"].parse::<u32>().unwrap();
            let ay = c["ay"].parse::<u32>().unwrap();
            let bx = c["bx"].parse::<u32>().unwrap();
            let by = c["by"].parse::<u32>().unwrap();
            let px = c["px"].parse::<u32>().unwrap();
            let py = c["py"].parse::<u32>().unwrap();

            let mut cheapest = None;
            let mut seen_states = HashSet::new();
            calculate_min_price(
                (px, py),
                (0, 0),
                (ax, ay),
                (bx, by),
                0,
                &mut cheapest,
                &mut seen_states,
            );

            sum += cheapest.unwrap_or(0);
        }
    }

    assert_eq!(sum, 35997);
}
