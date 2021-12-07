use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn generator(input: &str) -> HashMap<u8, u64> {
    input
        .trim()
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
}

pub fn simulate(input: &HashMap<u8, u64>, days: u32) -> u64 {
    let mut map = input.clone();
    for _ in 1..=days {
        let mut next = HashMap::new();
        let v: u64 = *map.get(&0).unwrap_or(&0);
        next.insert(8, v);
        next.insert(6, v);

        for age in 1..=8 {
            let v: u64 = *map.get(&age).unwrap_or(&0);
            let next_age = age - 1;
            *next.entry(next_age).or_insert(0) += v;
        }
        map = next.clone();
    }
    map.values().fold(0, |acc, &x| acc + x)
}

#[aoc(day6, part1)]
pub fn part1(input: &HashMap<u8, u64>) -> u64 {
    simulate(input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &HashMap<u8, u64>) -> u64 {
    simulate(input, 256)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(part1(&generator("3,4,3,1,2")), 5934);
        assert_eq!(part2(&generator("3,4,3,1,2")), 26984457539);
    }
}
