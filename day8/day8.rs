extern crate itertools;
use itertools::Itertools;
use std::fs;

// Could do this without the reshape and just count the 0s and
// # of 1's and # of 2's for each one and then take the solution for
// min 0s. Same goes for the second part. I dont need the Vec<Vec<>>.
// I originally wrote it this way expecting the second part to be more
// involved
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(part1("123456789012", 3, 2), 1);
    println!(
        "Solution to part 1: {}",
        part1(
            &fs::read_to_string("input.txt").expect("Unable to read input"),
            25,
            6
        )
    );

    println!(
        "Solution to part 2: {}",
        part2(
            &fs::read_to_string("input.txt").expect("Unable to read input"),
            25,
            6
        )
    );
    Ok(())
}

fn part1(input: &str, width: usize, height: usize) -> usize {
    let layers = map_to_layers(input, width, height)
        .iter()
        .map(|row| (row, row.iter().filter(|v| **v == '0').count()))
        .min_by(|a, b| (a.1).cmp(&b.1))
        .unwrap()
        .0
        .clone();
    let n_ones = layers.iter().filter(|v| **v == '1').count();
    let n_twos = layers.iter().filter(|v| **v == '2').count();
    n_ones * n_twos
}

fn part2(input: &str, width: usize, height: usize) -> String {
    let layers = map_to_layers(input, width, height);
    // 2 is transparent
    // Strategy is to just overwrite all the 2's and skip otherwise.
    // Could be a match with an enum.
    let mut message = vec!['2'; width * height];
    for layer in layers {
        for (index, digit) in layer.iter().enumerate() {
            if message[index] == '2' {
                message[index] = *digit;
            }
        }
    }
    // insert newlines
    message = message
        .iter()
        .map(|pixel| match pixel {
            '0' => ' ' as char,
            '1' => '#' as char,
            _ => panic!("We should know be here"),
        })
        .collect();
    for index in (0..(width * height)).step_by(width).rev() {
        message.insert(index, '\n');
    }
    message.into_iter().collect()
}

// 2D "image", but I'm too lazy to combine the digits
fn map_to_layers(input: &str, width: usize, height: usize) -> Vec<Vec<char>> {
    let mut reshaped = Vec::new();
    for chunk in input.chars().chunks(width * height).into_iter() {
        reshaped.push(chunk.collect());
    }
    reshaped
}
