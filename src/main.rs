mod day_one;
mod day_three;
mod day_two;

enum Part {
    One,
    Two,
}
fn main() {
    let day = match std::env::args().nth(1) {
        Some(day) => day.parse::<usize>().unwrap_or(1),
        None => 1,
    };
    let part = match std::env::args().nth(2) {
        Some(part) => {
            let parsed = part.parse::<usize>().unwrap_or(1);
            match parsed {
                1 => Part::One,
                2 => Part::Two,
                _ => panic!("Each day only has 2 parts"),
            }
        }
        None => Part::One,
    };
    match day {
        1 => match part {
            Part::One => day_one::part_one(),
            Part::Two => day_one::part_two(),
        },
        2 => match part {
            Part::One => day_two::part_one(),
            Part::Two => day_two::part_two(),
        },
        3 => match part {
            Part::One => day_three::part_one(),
            Part::Two => day_three::part_two(),
        },
        _ => panic!("Day not found"),
    };
}
