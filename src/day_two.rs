use std::fs::read_to_string;

fn import_program() -> Vec<usize> {
    read_to_string("./src/data/day_two.txt")
        .expect("couldn't read input file")
        .split(",")
        .map(|l| l.parse().unwrap())
        .collect()
}

pub fn part_one() {
    let mut int_code = import_program();
    int_code[1] = 12;
    int_code[2] = 2;
    let int_code = process(int_code);
    println!("position 0: {}", int_code[0]);
}

pub fn part_two() {
    let int_code = import_program();
    let mut noun = 0;
    let mut verb = 0;
    'outer: loop {
        if noun > 99 {
            panic!("Not found");
        }
        loop {
            if verb > 99 {
                break;
            }
            let mut memory = int_code.clone();
            memory[1] = noun;
            memory[2] = verb;
            let res = process(memory);
            if res[0] == 19_690_720 {
                break 'outer;
            }
            verb += 1;
        }
        noun += 1;
        verb = 0;
    }
    println!("noun={}, verb={}, answer={}", noun, verb, 100 * noun + verb);
}

// This could take &mut Vec but tests would be a bit more awkward
fn process(mut int_code: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        match int_code[i] {
            curr @ 1 | curr @ 2 => {
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
