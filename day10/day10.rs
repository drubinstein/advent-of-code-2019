use std::collections::HashSet;
use std::fs;

extern crate num;
use num::complex::Complex;

fn main() {
    assert_eq!(
        part1(&create_asteroid_map(
            ".#..#
.....
#####
....#
...##"
        )),
        (Complex::new(3,4), 8)
    );

    println!(
        "Solution to part 1: {:?}",
        part1(&create_asteroid_map(
            &fs::read_to_string("input.txt").expect("Unable to read input")
        ))
    );
}

fn create_asteroid_map(map: &str) -> HashSet<Complex<i32>> {
    map.split('\n')
        .enumerate()
        .map(|(rindex, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, val)| *val == '#')
                .map(move |(cindex, _)| Complex::new(cindex as i32, rindex.clone() as i32))
        })
        .flatten()
        .collect()
}

fn part1(asteroid_map: &HashSet<Complex<i32>>) -> (Complex<i32>, i32) {
    asteroid_map
        .iter()
        .map(|start| {
            (
                start.clone(),
                asteroid_map
                    .iter()
                    .filter(|asteroid| *asteroid != start)
                    .map(|asteroid| 
                            Complex::new(asteroid.re - start.re, asteroid.im - start.im))
                    .map(|distance| distance.unscale(num::integer::gcd(distance.re, distance.im)))
                    .collect::<HashSet<Complex<i32>>>()
                    .len() as i32,
            )
        })
        .max_by(|(_, count_1), (_, count_2)| count_1.cmp(count_2))
        .unwrap()
}
