use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn day2() {
    let f = File::open("day2.txt").unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let program = line.unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        println!("{:?}", compute_program(program)[0]);
    }
}

pub fn day2_part2() {
    for noun in 0..100 {
        for verb in 0..100 {
            let f = File::open("day2.txt").unwrap();
            let file = BufReader::new(&f);
            for line in file.lines() {
                let mut program = line.unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                program[1] = noun;
                program[2] = verb;
                let res = compute_program(program)[0];

                if res == 19690720 {
                    println!("{:?}", 100 * noun + verb)
                }
    }
        }
    }
    
}

fn compute_program(mut program: Vec<usize>) -> Vec<usize> {
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
        assert_eq!(compute_program(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    }
}