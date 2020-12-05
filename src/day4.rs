use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::{Regex, Captures};
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    let parsed_input: Vec<String> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.to_string()).join(" "))
        .collect();
    parsed_input
        .iter()
        .map(|s: &String| {
            let v: Vec<Vec<String>> = s
                .split(" ")
                .map(|v| v.to_string().split(":").map(|z| z.to_string()).collect())
                .collect();
            let mut m = HashMap::new();
            for vv in v {
                let mut i = vv.into_iter();
                let k = i.nth(0).unwrap();
                let vvv = i.nth(0).unwrap();
                if k != "cid" {
                    m.entry(k).or_insert(vvv);
                }
            }
            m
        })
        .filter(|h| {
            let keys = h.keys();
            itertools::equal(
                itertools::sorted(keys),
                vec!["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"],
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<HashMap<String, String>>) -> usize {
    input.len()
}

fn validate_year(key: &str, value: usize) -> bool {
    match key {
        "byr" => value >= 1920 && value <= 2002,
        "eyr" => value >= 2020 && value <= 2030,
        "iyr" => value >= 2010 && value <= 2020,
        _ => false
    }
}

fn check_height(captures: Captures) -> bool {
    let unit = &captures["unit"];
    let value = captures["val"].parse::<usize>().unwrap();
    match unit {
        "cm" => value >= 150 && value <= 193,
        "in" => value >= 59 && value <= 76,
        _ => false
    }
}

fn validate_height(value: &String) -> bool {
    let height_regex = Regex::new(r"(?x)(?P<val>\d+)(?P<unit>(cm|in))").unwrap();
    let caps = height_regex.captures(value);
    match caps {
        None => return false,
        Some(inner) => return check_height(inner)
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<HashMap<String, String>>) -> usize {
    let hair_color_regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
    let pid_regex = Regex::new(r"^(\d{9})$").unwrap();
    let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    input
        .iter()
        .filter(|h| {
            hair_color_regex.is_match(h.get("hcl").unwrap())
                && pid_regex.is_match(h.get("pid").unwrap())
                && eye_colors.iter().any(|s| s.to_string() == *h.get("ecl").unwrap())
                && validate_year("byr", h.get("byr").unwrap().parse::<usize>().unwrap())
                && validate_year("eyr", h.get("eyr").unwrap().parse::<usize>().unwrap())
                && validate_year("iyr", h.get("iyr").unwrap().parse::<usize>().unwrap())
                && validate_height(h.get("hgt").unwrap())
        })
        .collect::<Vec<&HashMap<String, String>>>()
        .len()
}
