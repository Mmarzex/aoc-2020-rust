use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l: &str| {
        l.to_string()
    }).collect()
}

fn calculate_hit_trees(input: &Vec<String>, dx: usize, dy: usize) -> usize {
    let width = input[0].chars().count();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut trees_hit: usize = 0;
    while y < input.len() - 1 {
        if x + dx >= width {
            x = (x + dx) - width;
        } else {
            x = x + dx;
        }

        y = y + dy;

        if input[y].chars().nth(x).unwrap().to_string() == "#" {
            trees_hit += 1;
        }
    }
    trees_hit
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<String>) -> usize {
    calculate_hit_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    let slopes = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let results:usize = slopes.into_iter().map(|s| {
        calculate_hit_trees(input, s[0], s[1])
    }).product();
    println!("{:?}", results);
    1
}