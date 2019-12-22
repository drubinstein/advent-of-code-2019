use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    //assert_eq!(run(vec![1,0,0,0,99].as_mut_slice()), 2);
    //assert_eq!(run(vec![1,9,10,3,2,3,11,0,99,30,40,50].as_mut_slice()), 3500);

    let intcodes: Vec<i32> = fs::read_to_string("input.csv")
        .expect("Unable to read input")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!(
        "Solution for part 1: {}",
        part1(intcodes.clone().as_mut_slice())
    );
    println!(
        "Solution for part 2: {}",
        part2(intcodes.clone().as_mut_slice())
    );
    Ok(())
}

fn part1(intcodes: &mut [i32]) -> i32 {
    let outputs = run(1, intcodes);
    println!("{:?}", outputs);
    *outputs.last().unwrap()
}

fn part2(intcodes: &mut [i32]) -> i32 {
    let outputs = run(5, intcodes);
    println!("{:?}", outputs);
    *outputs.last().unwrap()
}

// Would be cool to change this match to be an enum
fn run(input: i32, intcodes: &mut [i32]) -> Vec<i32> {
    let mut program_counter: usize = 0;
    let mut io: Vec<i32> = vec![input];
    loop {
        let intcode = intcodes[program_counter];
        let opcode = intcode % 100;
        let parameter_mode = intcode / 100;
        match opcode {
            1 => {
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(intcodes, parameter_mode, program_counter, 2);
                let dest = intcodes[program_counter + 3] as usize;
                intcodes[dest] = src_1 + src_2;
                program_counter += 4;
            }
            2 => {
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(intcodes, parameter_mode, program_counter, 2);
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
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                io.push(src_1);
                println!("Diagnostic finished! Output: {}", src_1);
                program_counter += 2;
            }
            5 => {
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(intcodes, parameter_mode, program_counter, 2);
                if src_1 != 0 {
                    program_counter = src_2 as usize;
                } else {
                    program_counter += 3;
                }
            }
            6 => {
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(intcodes, parameter_mode, program_counter, 2);
                if src_1 == 0 {
                    program_counter = src_2 as usize;
                } else {
                    program_counter += 3;
                }
            }
            7 => {
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(intcodes, parameter_mode, program_counter, 2);
                let dest = intcodes[program_counter + 3] as usize;
                intcodes[dest] = (src_1 < src_2) as i32;
                program_counter += 4;
            }
            8 => {
                let src_1 = get(intcodes, parameter_mode, program_counter, 1);
                let src_2 = get(intcodes, parameter_mode, program_counter, 2);
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
    io
}

fn get(intcodes: &mut [i32], parameter_mode: i32, program_counter: usize, position: usize) -> i32 {
    // position is relative to the program counter
    match (parameter_mode / 10_i32.pow(position as u32 - 1_u32)) % 10 {
        0 => intcodes[intcodes[program_counter + position] as usize],
        1 => intcodes[program_counter + position],
        _ => panic!("Invalid parameter mode detected, {}", parameter_mode),
    }
}
