use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn get_values(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse::<i32>()).collect()
}

#[aoc(day01, part1)]
pub fn sample1(inputs: &[i32]) -> i32 {
    return -1
}

#[aoc(day01, part2)]
pub fn sample2(inputs: &[i32]) -> i32
{
    return -2;
}
