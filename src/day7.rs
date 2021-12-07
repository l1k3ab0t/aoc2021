use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
    let mut v = input.clone();
    v.sort();
    let median = v[v.len() / 2];
    v.iter().fold(0, |acc, x| acc + (median - x).abs())
}

#[aoc(day7, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
    let avg = (input.iter().sum::<i32>() as f32 / input.len() as f32).round() as i32;
    println!("{}", avg);

    (avg - 1..=avg + 1)
        .map(|avg2| {
            input.iter().fold(0, |acc, x| {
                acc + (1..=(avg2 - x).abs()).fold(0, |acc2, y| acc2 + y)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(part1(&generator("16,1,2,0,4,2,7,1,2,14")), 37);
        assert_eq!(part2(&generator("16,1,2,0,4,2,7,1,2,14")), 168);
    }
}
