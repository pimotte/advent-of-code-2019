use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn day2() {
    let f = File::open("day2.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        println!("{:?}", compute_program(line.unwrap())[0]);
    }
}

fn compute_program(line: String) -> Vec<usize> {
    let mut program = line.split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut index = 0;
    let mut finished = false;

    while !finished {
        match program.get(index) {
            Some(1) => {
                let op1 = program[index + 1];
                let op2 = program[index + 2];
                let target = program[index + 3];

                // println!("Settings {:?} to {:?}", target, program[op1] + program[op2]);
                program[target] = program[op1] + program[op2];
            },
            Some(2) => {
                let op1 = program[index + 1];
                let op2 = program[index + 2];
                let target = program[index + 3];
                program[target] = program[op1] * program[op2];
            },
            Some(99) => {
                finished = true;
            }
            Some(_)| None => unreachable!()
        }

        index += 4;
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(compute_program("1,0,0,0,99".to_string()), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_first() {
        assert_eq!(compute_program("1,0,0,0,99".to_string()), vec![2, 0, 0, 0, 99]);
    }
}