use std::fs::read_to_string;

pub fn main() {
    let mut int_code: Vec<usize> = read_to_string("./src/data/day_two.txt")
        .expect("couldn't read input file")
        .split(",")
        .map(|l| l.parse().unwrap())
        .collect();
    int_code[1] = 12;
    int_code[2] = 2;
    let int_code = process(int_code);
    println!("position 0: {}", int_code[0]);
}

// This could take &mut Vec but tests would be a bit more awkward
fn process(mut int_code: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        let curr = int_code[i];
        match curr {
            1 | 2 => {
                let a = int_code[int_code[i + 1]];
                let b = int_code[int_code[i + 2]];
                let res = int_code[i + 3];
                int_code[res] = if curr == 1 { a + b } else { a * b };
            }
            99 => break,
            _ => panic!("Bad input"),
        }
        i += 4;
    }
    int_code
}

#[cfg(test)]
mod test {
    use super::process;
    #[test]
    fn test_process() {
        assert_eq!(process(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(process(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(process(vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            process(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
