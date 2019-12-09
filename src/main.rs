mod day_one;

fn main() {
    let day = match std::env::args().nth(1) {
        Some(day) => day.parse::<usize>().unwrap_or(1),
        None => 1,
    };
    match day {
        1 => day_one::main(),
        _ => panic!("Day not found"),
    };
}
