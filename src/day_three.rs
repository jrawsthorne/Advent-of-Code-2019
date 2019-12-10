use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Node {
    direction: Direction,
    magnitude: usize,
}

impl FromStr for Node {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.chars().nth(0).expect("No direction") {
            'L' => Direction::L,
            'R' => Direction::R,
            'U' => Direction::U,
            'D' => Direction::D,
            _ => panic!("Invalid direction"),
        };
        let magnitude = s.get(1..).unwrap().parse().expect("Bad magnitude");
        Ok(Node {
            direction,
            magnitude,
        })
    }
}

#[derive(Debug)]
enum Direction {
    L,
    R,
    U,
    D,
}

fn import_paths() -> Vec<Vec<Node>> {
    read_to_string("./src/data/day_three.txt")
        .expect("couldn't read input file")
        .lines()
        .take(2)
        .map(|l| l.split(",").map(|l| l.parse().unwrap()).collect())
        .collect()
}

pub fn part_one() {
    let paths = import_paths();
    println!("{:?}", paths);
}

pub fn part_two() {}

#[cfg(test)]
mod test {
    // #[test]
}
