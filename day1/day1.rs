use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    println!("Answer to part 1: {}", part1().unwrap());
    println!("Answer to part 2: {}", part2().unwrap());
    println!("{}", calculate_fuel(14));
    println!("{}", calculate_fuel(1969));
}

fn part1() -> Result<i32, ()> {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    Ok(f.lines()
     .map(|x| x.unwrap())
     .map(|x| x.parse::<i32>())
     .map(|x| x.unwrap() / 3)
     .map(|x| x - 2)
     .sum())
}

fn part2() -> Result<i32, ()> {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    Ok(f.lines()
     .map(|x| x.unwrap())
     .map(|x| x.parse::<i32>())
     .map(|x| calculate_fuel(x.unwrap()))
     .sum())
}

fn calculate_fuel(mut fuel: i32) -> i32 {
    let mut total_fuel: i32 = 0; 
    fuel = fuel / 3 - 2;
    while fuel > 0 {
        total_fuel += fuel;
        fuel = fuel / 3 - 2;
    };
    total_fuel
}