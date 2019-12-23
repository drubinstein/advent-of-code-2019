extern crate permute;

use permute::permutations_of;
use std::fs;
use std::sync::mpsc;
use std::thread;

// TODO: Would be neat to rewrite this entirely with async/await
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(
        part2(&vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5
        ]),
        139629729,
        "test 1"
    );
    assert_eq!(
        part2(&vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ]),
        18216,
        "test 2"
    );
    let intcodes: Vec<i32> = fs::read_to_string("input.csv")
        .expect("Unable to read input")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("Solution for part 2: {:?}", part2(&intcodes.clone()));

    Ok(())
}

#[derive(Debug)]
struct Amplifier {
    intcode: Vec<i32>,
    input_signal: mpsc::Receiver<i32>,
    output_signal: mpsc::Sender<i32>,
    last_output: i32,
}

fn part2(intcodes: &Vec<i32>) -> i32 {
    permutations_of(&[5, 6, 7, 8, 9])
        .map(|seq| seq.map(|x| *x as i32).collect())
        .map(|seq: Vec<i32>| {
            let mut senders = Vec::new();
            let mut receivers = Vec::new();
            for _ in 0..5 {
                let (tx, rx) = mpsc::channel();
                senders.push(tx);
                receivers.push(rx);
            }
            receivers.rotate_right(1);
            for x in 0..5 {
                senders[(x + 4) % 5].send(seq[x]);
            }
            senders[4].send(0);

            let mut amplifiers: Vec<Amplifier> = (0..5)
                .map(|_| {
                    let rx = receivers.remove(0);
                    let tx = senders.remove(0);
                    Amplifier {
                        intcode: intcodes.clone(),
                        input_signal: rx,  // the input is the sender of the last amp
                        output_signal: tx, // the output is the sender of the current amp
                        last_output: 0,
                    }
                })
                .collect();

            let mut children = Vec::new();
            for _ in 0..5 {
                let amplifier = amplifiers.remove(0);
                children.push(thread::spawn(move || run(amplifier)));
            }
            let result = children.remove(4);
            match result.join() {
                Ok(v) => v.clone(),
                Err(e) => panic!("{:?}", e),
            }
        })
        .max()
        .unwrap()
}

fn run(mut amplifier: Amplifier) -> i32 {
    let mut program_counter: usize = 0;
    loop {
        let intcodes = &mut amplifier.intcode;
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
                let src_1 = amplifier.input_signal.recv().unwrap();
                let dest = intcodes[program_counter + 1] as usize;
                intcodes[dest] = src_1;
                program_counter += 2;
            }
            4 => {
                let src_1 = get(&intcodes, parameter_mode, program_counter, 1);
                amplifier.last_output = src_1;
                amplifier.output_signal.send(src_1);
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
    amplifier.last_output
}

fn get(intcodes: &Vec<i32>, parameter_mode: i32, program_counter: usize, position: usize) -> i32 {
    // position is relative to the program counter
    match (parameter_mode / 10_i32.pow(position as u32 - 1_u32)) % 10 {
        0 => intcodes[intcodes[program_counter + position] as usize],
        1 => intcodes[program_counter + position],
        _ => panic!("Invalid parameter mode detected, {}", parameter_mode),
    }
}
