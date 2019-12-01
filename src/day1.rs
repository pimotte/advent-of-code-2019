use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use std::collections::HashMap;


pub fn day1() {
    let f = File::open("day1.txt").unwrap();
    let file = BufReader::new(&f);
    let mut total_fuel = 0;
    let mut memo = HashMap::new();
    for line in file.lines() {
        let mass = line.unwrap().parse::<i64>().unwrap();
        total_fuel += calc_total_fuel(mass, &mut memo);
    }

    println!("{:?}", total_fuel);
}


fn calc_total_fuel(mass: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if mass <= 0 {
        return 0
    }

    match memo.get(&mass) {
        Some(u) => *u,
        None => {
            let fuel = calc_fuel(mass);

            let result = fuel + calc_total_fuel(fuel, memo);

            memo.insert(mass, result);
            result
        }
    }
}

fn calc_fuel(mass: i64) -> i64 {
    let fuel = mass/3-2;
    if fuel < 0 {
        return 0
    }
    return fuel;
}


