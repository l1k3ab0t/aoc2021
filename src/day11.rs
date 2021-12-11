use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day11)]
pub fn generator(input: &str) -> HashMap<(i8, i8), u8> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| ((x as i8, y as i8), c.to_digit(10).unwrap() as u8))
                .collect::<Vec<((i8, i8), u8)>>()
        })
        .flatten()
        .collect::<HashMap<_, _>>()
}

fn filter_flash(map: &HashMap<(i8, i8), u8>) -> Option<HashSet<(i8, i8)>> {
    let m: HashSet<(i8, i8)> = map
        .iter()
        .filter(|(_, v)| **v > 9)
        .map(|(k, _)| *k)
        .collect();
    if m.len() > 0 {
        Some(m)
    } else {
        None
    }
}

pub fn step(input: &HashMap<(i8, i8), u8>) -> (u32, HashMap<(i8, i8), u8>) {
    let mut flashed = HashSet::new();
    let mut octupuses: HashMap<(i8, i8), u8> = input.iter().map(|(k, v)| (*k, v + 1)).collect();

    while let Some(flashing) = filter_flash(&octupuses) {
        for octupus in flashing.iter() {
            flashed.insert(*octupus);
            octupuses.insert(*octupus, 0);
            for accending in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .map(|(x, y)| (x + octupus.0, y + octupus.1))
            {
                if octupuses.contains_key(&accending) && !flashed.contains(&accending) {
                    octupuses.entry(accending).and_modify(|v| *v += 1);
                }
            }
        }
    }

    (flashed.len() as u32, octupuses)
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<(i8, i8), u8>) -> u32 {
    let mut flashes = 0;
    let mut map = input.clone();
    for _ in 0..100 {
        let (f, m2) = step(&map);
        map = m2;
        flashes += f;
    }
    flashes
}

#[aoc(day11, part2)]
pub fn part2(input: &HashMap<(i8, i8), u8>) -> u32 {
    let mut i = 1;
    let count = input.len() as u32;
    let mut map = input.clone();
    loop {
        let (flashes, m2) = step(&map);
        if flashes == count {
            return i;
        }

        map = m2;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(&generator(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            )),
            1656
        );
        assert_eq!(
            part2(&generator(
                "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            )),
            195
        );
    }
}
