extern crate permute;

use permute::permutations_of;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(
        part1(&vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
        ]),
        43210,
        "test 1"
    );
    assert_eq!(
        part1(&vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0
        ]),
        54321,
        "test 2"
    );
    assert_eq!(
        part1(&vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ]),
        65210,
        "test 3"
    );
    let intcodes: Vec<i32> = fs::read_to_string("input.csv")
        .expect("Unable to read input")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("Solution for part 1: {:?}", part1(&intcodes.clone()));

    Ok(())
}

fn part1(intcodes: &Vec<i32>) -> i32 {
    permutations_of(&[0, 1, 2, 3, 4])
        .map(|seq| seq.map(|x| *x as i32).collect())
        .map(|seq: Vec<i32>| {
            seq.iter()
                .fold(0, |acc, x| run(&vec![acc, *x], intcodes.clone()))
        })
        .max()
        .unwrap()
}

fn part2(intcodes: &Vec<i32>) -> i32 {
    permutations_of(&[5, 6, 7, 8, 9])
        .map(|seq| seq.map(|x| *x as i32).collect())
        .map(|seq: Vec<i32>| {
            let mut init = 0;
            loop {
                init = seq
                    .iter()
                    .fold(init, |acc, x| run(&vec![acc, *x], intcodes.clone()));
            }
        })
        .max()
        .unwrap()
}

fn run(input: &Vec<i32>, mut intcodes: Vec<i32>) -> i32 {
    let mut program_counter: usize = 0;
    let mut io: Vec<i32> = input.clone();
    loop {
        let intcode = intcodes[program_counter];
        let opcode = intcode % 100;
        let parameter_mode = intcode / 100;
        match opcode {
            1 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(&intcodes, parameter_mode, program_counter, 2);
                let dest = intcodes[program_counter + 3] as usize;
                intcodes[dest] = src_1 + src_2;
                program_counter += 4;
            }
            2 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(&intcodes, parameter_mode, program_counter, 2);
                let dest = intcodes[program_counter + 3] as usize;
                intcodes[dest] = src_1 * src_2;
                program_counter += 4;
            }
            3 => {
                let src_1 = io.pop().unwrap();
                let dest = intcodes[program_counter + 1] as usize;
                intcodes[dest] = src_1;
                program_counter += 2;
            }
            4 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                io.push(src_1);
                program_counter += 2;
            }
            5 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(&intcodes, parameter_mode, program_counter, 2);
                if src_1 != 0 {
                    program_counter = src_2 as usize;
                } else {
                    program_counter += 3;
                }
            }
            6 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(&intcodes, parameter_mode, program_counter, 2);
                if src_1 == 0 {
                    program_counter = src_2 as usize;
                } else {
                    program_counter += 3;
                }
            }
            7 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(&intcodes, parameter_mode, program_counter, 2);
                let dest = intcodes[program_counter + 3] as usize;
                intcodes[dest] = (src_1 < src_2) as i32;
                program_counter += 4;
            }
            8 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(&intcodes, parameter_mode, program_counter, 2);
                let dest = intcodes[program_counter + 3] as usize;
                intcodes[dest] = (src_1 == src_2) as i32;
                program_counter += 4;
            }
            99 => break,
            _ => panic!(
                "Invalid opcode detected => (pc, intcode, opcode): ({}, {}, {})",
                program_counter, intcode, opcode
            ),
        };
    }
    *io.last().unwrap()
}

fn get(intcodes: &Vec<i32>, parameter_mode: i32, program_counter: usize, position: usize) -> i32 {
    // position is relative to the program counter
    match (parameter_mode / 10_i32.pow(position as u32 - 1_u32)) % 10 {
        0 => intcodes[intcodes[program_counter + position] as usize],
        1 => intcodes[program_counter + position],
        _ => panic!("Invalid parameter mode detected, {}", parameter_mode),
    }
}
