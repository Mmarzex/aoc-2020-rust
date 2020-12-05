use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| {
        let num_str = l.to_string();
        num_str.parse::<i32>().unwrap()
    }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let combos = input.into_iter().combinations(2).find_position(|c| c[0] + c[1] == 2020).unwrap();
    combos.1[0] * combos.1[1]
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let combos = input.into_iter().combinations(3).find_position(|c| c[0] + c[1] + c[2] == 2020).unwrap();
    combos.1[0] * combos.1[1] * combos.1[2]
}