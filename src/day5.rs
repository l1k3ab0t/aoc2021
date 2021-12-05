use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;
use std::collections::HashMap;

type Vent = Vec<u32>;

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Vent> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split(" -> ")
                .map(|i| {
                    i.split(",")
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                        .into_iter()
                })
                .flat_map(|i| i)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vent>>()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Vent>) -> usize {
    let mut m: HashMap<(u32, u32), u32> = HashMap::new();
    for vent in input.iter() {
        if vent[0] == vent[2] {
            let start = cmp::min(vent[1], vent[3]);
            let end = cmp::max(vent[1], vent[3]);
            (start..=end).for_each(|y| *m.entry((vent[0], y)).or_insert(0) += 1);
        } else if vent[1] == vent[3] {
            let start = cmp::min(vent[0], vent[2]);
            let end = cmp::max(vent[0], vent[2]);
            (start..=end).for_each(|x| *m.entry((x, vent[1])).or_insert(0) += 1);
        }
    }
    m.values().filter(|v| v > &&1).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Vent>) -> usize {
    let mut m: HashMap<(u32, u32), u32> = HashMap::new();
    for vent in input.iter() {
        let start_x = cmp::min(vent[0], vent[2]);
        let start_y = cmp::min(vent[1], vent[3]);
        let end_x = cmp::max(vent[0], vent[2]);
        let end_y = cmp::max(vent[1], vent[3]);
        if vent[0] == vent[2] {
            (start_x..=end_x).for_each(|y| *m.entry((vent[0], y)).or_insert(0) += 1);
        } else if vent[1] == vent[3] {
            (start_y..=end_y).for_each(|x| *m.entry((x, vent[1])).or_insert(0) += 1);
        } else {
            let mut r_x: Vec<u32> = (start_x..=end_x).collect();
            let mut r_y: Vec<u32> = (start_y..=end_y).collect();

            if vent[0] > vent[2] {
                r_x = r_x.into_iter().rev().collect();
            }
            if vent[1] > vent[3] {
                r_y = r_y.into_iter().rev().collect();
            }
            r_x.into_iter()
                .zip(r_y.into_iter())
                .for_each(|(x, y)| *m.entry((x, y)).or_insert(0) += 1);
        }
    }
    m.values().filter(|v| v > &&1).count()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(&generator(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            )),
            5
        );
        assert_eq!(part2(&generator("")), 0);
    }
}
