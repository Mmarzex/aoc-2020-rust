use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;

#[derive(Debug)]
pub struct PasswordEntry {
    password: String,
    check_character: String,
    lower_bound: i32,
    upper_bound: i32,
}

impl fmt::Display for PasswordEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "({}, {}, {}, {})",
            self.password, self.check_character, self.lower_bound, self.upper_bound
        )
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|l| {
            let line = l.to_string();
            let split_pass: Vec<&str> = line.split(" ").collect();
            let password = split_pass[2].to_string();
            let bounds: Vec<&str> = split_pass[0].split("-").collect();
            let split_check: Vec<&str> = split_pass[1].split(":").collect();
            let check_character = split_check[0].to_string();
            let lower_bound = bounds[0].parse::<i32>().unwrap();
            let upper_bound = bounds[1].parse::<i32>().unwrap();
            PasswordEntry {
                password,
                check_character,
                lower_bound,
                upper_bound,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<PasswordEntry>) -> i32 {
    let valid_passwords: Vec<&PasswordEntry> = input
        .into_iter()
        .filter(|entry| {
            let c: i32 = entry.password.matches(&entry.check_character).count() as i32;
            c >= entry.lower_bound && c <= entry.upper_bound
        })
        .collect();
    println!("{:?}", valid_passwords.len());
    valid_passwords.len() as i32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<PasswordEntry>) -> i32 {
    let valid_passwords: Vec<&PasswordEntry> = input.into_iter()
    .filter(|entry| {
        let a = entry.password.chars().nth((entry.lower_bound - 1) as usize).unwrap().to_string();
        let b = entry.password.chars().nth((entry.upper_bound - 1) as usize).unwrap().to_string();
        (a == entry.check_character && b != entry.check_character) || (a != entry.check_character && b == entry.check_character)
    }).collect();
    valid_passwords.len() as i32
}
