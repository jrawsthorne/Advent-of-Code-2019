mod day_one;
mod day_two;

fn main() {
    let day = match std::env::args().nth(1) {
        Some(day) => day.parse::<usize>().unwrap_or(1),
        None => 1,
    };
    match day {
        1 => day_one::main(),
        2 => day_two::main(),
        _ => panic!("Day not found"),
    };
}
