fn main() {
    assert_eq!(part1(111111, 111111), 1);
    assert_eq!(part1(223450, 223450), 0);
    assert_eq!(part1(123789, 123789), 0);
    println!("Solution to part 1: {}", part1(109165, 576723));

    assert_eq!(part2(112233, 112233), 1);
    assert_eq!(part2(123444, 123444), 0);
    assert_eq!(part2(111122, 111122), 1);
    println!("Solution to part 2: {}", part2(109165, 576723));
}

fn part1(lower_bound: u32, upper_bound: u32) -> u32 {
    (lower_bound..=upper_bound)
        .map(|num| {
            let bytes = num.to_string().into_bytes();
            (adjacent(&bytes) && increasing(&bytes)) as u32
        })
        .sum()
}

fn part2(lower_bound: u32, upper_bound: u32) -> u32 {
    (lower_bound..=upper_bound)
        .map(|num| {
            let bytes = num.to_string().into_bytes();
            (increasing(&bytes) && adjacent_larger(&bytes)) as u32
        })
        .sum()
}

fn adjacent(bytes: &Vec<u8>) -> bool {
    bytes
        .iter()
        .zip(bytes.iter().skip(1))
        .filter(|x| x.0 == x.1)
        .next()
        != None
}

fn adjacent_larger(bytes: &Vec<u8>) -> bool {
    // TODO: I dont like this looping method. I would rather make it look more functional.
    let mut last_char = bytes[0];
    let mut count = 1;
    for ch in &bytes[1..bytes.len() - 1] {
        if last_char == *ch {
            count += 1;
        } else if last_char != *ch && count == 2 {
            return true;
        } else {
            // last_char != ch
            last_char = *ch;
            count = 1;
        }
    }
    let final_byte = *bytes.last().unwrap();
    if (last_char == final_byte && count == 1) || (final_byte != last_char && count == 2) {
        return true;
    }
    false
}

fn increasing(bytes: &Vec<u8>) -> bool {
    bytes
        .iter()
        .zip(bytes.iter().skip(1))
        .fold(true, |acc, x| acc && x.0 <= x.1)
}
