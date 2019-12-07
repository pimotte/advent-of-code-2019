use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str::FromStr;

use std::collections::HashSet;
use std::collections::VecDeque;

use Direction::*;

#[derive(Debug)]
pub enum Direction {
    Up, Right, Left, Down
}

impl FromStr for Direction {
    type Err = std::num::ParseIntError;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        match &code[0..1] {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    dir: Direction,
    steps: i64
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let dir = Direction::from_str(code)?;
        let steps = code[1..code.len()].parse::<i64>().unwrap();
        Ok(Instruction {
            dir, steps
        })
    }
}

pub fn day3() {
    let f = File::open("day3.txt").unwrap();
    let file = BufReader::new(&f);
    let mut sets: Vec<HashSet<(i64, i64, i64)>> = vec![];
    for line in file.lines() {
        let line_str = line.unwrap();
        let input = line_str.split(",").map(|i| Instruction::from_str(i).unwrap()).collect::<VecDeque<Instruction>>();
        let tmp = compute_squares(input);
        println!("{:?}", tmp);
        sets.push(tmp);
    }
    let proj1 = sets[0].iter().map(|(x, y, _z)| (*x, *y)).collect::<HashSet<(i64, i64)>>();
    let proj2 = sets[1].iter().map(|(x, y, _z)| (*x, *y)).collect::<HashSet<(i64, i64)>>();
    let intersection = proj1.intersection(&proj2);
    println!("{:?}", intersection);

    let mut minimum = std::i64::MAX;

    for (x, y) in intersection {
        let steps1 = sets[0].iter().filter(|(x1, y1, _z1)| x1 == x && y1 == y).map(|(_x, _y, z)| z).min().unwrap();
        let steps2 = sets[1].iter().filter(|(x1, y1, _z1)| x1 == x && y1 == y).map(|(_x, _y, z)| z).min().unwrap();

        if steps1 + steps2 < minimum {
            minimum = steps1 + steps2
        }
    }



    println!("{:?}", minimum)
}

pub fn compute_squares(mut input: VecDeque<Instruction>) -> HashSet<(i64, i64, i64)> {
    let mut current_pos = (0, 0);
    let mut signal = 0;
    let mut result = HashSet::new();
    while input.len() > 0 {
        match input.pop_front() {
            Some(i) => {
                println!("{:?}", i);
                let mutation = match i.dir {
                    Up => (1, 0),
                    Down => (-1, 0),
                    Right => (0, 1),
                    Left => (0, -1)
                };

                for _i in 0..i.steps {
                    current_pos.0 += mutation.0;
                    current_pos.1 += mutation.1;
                    signal += 1;

                    println!("{:?}", current_pos);
                    result.insert((current_pos.0, current_pos.1, signal));
                }
                
            },
            None => unreachable!()
        }
    }
    result
}