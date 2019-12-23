use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// TODO: I would really love to use &str instead of String, but need to look up
// lifetimes of strings and rust. Especially when reading from a file.
// Ideally what I want to do is move ownership when the reading the file, but I couldnt
// figure out how to do it.
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    assert_eq!(part1(&parse_input_file("input-2.txt")), 42);
    println!(
        "Solution for part 1: {}",
        part1(&parse_input_file("input.txt"))
    );

    assert_eq!(part2(&parse_input_file("input-3.txt")), 4);
    println!(
        "Solution for part 2: {}",
        part2(&parse_input_file("input.txt"))
    );
    Ok(())
}

fn parse_input_file(filename: &str) -> HashMap<String, Vec<String>> {
    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    for line in f.lines() {
        let splitted = line
            .unwrap()
            .split(")")
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let orbiter = &splitted[0];
        let orbitee = &splitted[1];
        if orbits.contains_key(orbiter) {
            orbits.get_mut(orbiter).unwrap().push(orbitee.to_string());
        } else {
            orbits.insert(orbiter.to_string(), vec![orbitee.to_string()]);
        }
    }
    orbits
}

fn part1(orbits: &HashMap<String, Vec<String>>) -> u32 {
    let mut counts: HashMap<String, u32> = [("COM".to_string(), 0)].iter().cloned().collect();
    count_orbits("COM".to_string(), orbits, &mut counts, 0);
    counts.values().sum()
}

fn part2(orbits: &HashMap<String, Vec<String>>) -> i32 {
    let (lco, count, foundl, foundr) = find_lco(
        orbits,
        &"YOU".to_string(),
        &"SAN".to_string(),
        &"COM".to_string(),
    );
    count - 2
}

fn find_lco(
    orbits: &HashMap<String, Vec<String>>,
    left: &String,
    right: &String,
    head: &String,
) -> (String, i32, bool, bool) {
    // Lowest common orbit. Basically the same idea as doing it for a binary tree.
    // could also just use a variant of dijkstra's but that's too generic imo.
    let mut found_left = false;
    let mut found_right = false;
    let mut count = 0;

    if head == left {
        found_left = true;
    }
    if head == right {
        found_right = true;
    }

    if orbits.contains_key(&head.to_string()) {
        for body in orbits.get(&head.to_string()).unwrap() {
            let (lco, cnt, foundl, foundr) = find_lco(orbits, left, right, &body.to_string());
            if foundl && foundr {
                return (lco, cnt, foundl, foundr);
            } else if foundl {
                found_left = true;
                count += 1 + cnt;
            } else if foundr {
                found_right = true;
                count += 1 + cnt;
            }
        }
    }
    (head.to_string(), count, found_left, found_right)
}

fn count_orbits(
    key: String,
    orbits: &HashMap<String, Vec<String>>,
    counts: &mut HashMap<String, u32>,
    orbited: u32,
) {
    if !orbits.contains_key(&key.to_string()) {
        counts.insert(key.to_string(), orbited);
    } else {
        // orbit contains the key!
        for body in orbits.get(&key.to_string()).unwrap() {
            count_orbits(body.to_string(), orbits, counts, orbited + 1);
        }
        counts.insert(key.to_string(), orbited);
    }
}
