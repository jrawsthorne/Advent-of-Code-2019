use std::fs::read_to_string;

pub fn main() {
    let sum = read_to_string("./src/data/day_one.txt")
        .expect("couldn't read input file")
        .lines()
        .map(|l| correct_fuel(l.parse().expect("couldn't parse line as int")))
        .fold(0, |total, x| total + x);
    println!("sum={}", sum);
}

fn fuel(module_mass: isize) -> isize {
    module_mass / 3 - 2
}

fn correct_fuel(module_mass: isize) -> usize {
    let module_fuel = module_mass / 3 - 2;
    if module_fuel < 0 {
        0
    } else {
        module_fuel as usize + correct_fuel(module_fuel)
    }
}

#[cfg(test)]
mod test {
    use super::{correct_fuel, fuel};
    #[test]
    fn test_fuel() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn correct_fuel_test() {
        assert_eq!(correct_fuel(14), 2);
        assert_eq!(correct_fuel(1969), 966);
        assert_eq!(correct_fuel(100756), 50346);
    }
}
