use std::fs;

#[macro_use] extern crate itertools;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(run(vec![1,0,0,0,99].as_mut_slice()), 2);
    assert_eq!(run(vec![1,9,10,3,2,3,11,0,99,30,40,50].as_mut_slice()), 3500);

    let intcodes: Vec<usize> = fs::read_to_string("input.csv")
        .expect("Unable to read input")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    println!("Solution for part 1: {}", part1(intcodes.clone().as_mut_slice()));
    println!("Solution for part 2: {}", part2(&intcodes));
    Ok(())
}

fn part1(intcodes: &mut [usize]) -> usize {
    intcodes[1] = 12;
    intcodes[2] = 2;
    run(intcodes)
}

fn part2(intcodes: &Vec<usize>) -> usize {
    let mut result: usize = 0;
    for (noun, verb) in iproduct!(0..100, 0..100) {
        let mut program = intcodes.clone();
        program[1] = noun;
        program[2] = verb;
        let local_result = run(program.as_mut_slice());
        if local_result == 19690720 {
            println!("Success with ({}, {})", noun, verb);
            result = 100 * noun + verb;
            break;
        }
    }
    result
}

fn run(intcodes: &mut [usize]) -> usize {
    let mut program_counter = 0;
    loop {
        match intcodes[program_counter] {
            1 => {
                let operand_1 = intcodes[program_counter+1];
                let operand_2 = intcodes[program_counter+2];
                let dest = intcodes[program_counter+3];
                intcodes[dest] = intcodes[operand_1] + intcodes[operand_2];
            },
            2 => { 
                let operand_1 = intcodes[program_counter+1];
                let operand_2 = intcodes[program_counter+2];
                let dest = intcodes[program_counter+3];
                intcodes[dest] = intcodes[operand_1] * intcodes[operand_2];            
            },
            99 => break,
            _ => panic!("Invalid opcode detected, {}", intcodes[program_counter])
        };
        program_counter += 4;
    };
    intcodes[0]
}