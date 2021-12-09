use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let h = input.len();
    let w = input[0].len();

    let mut v: Vec<Vec<u32>> = vec![vec![10; w]; 1];
    let mut v2 = input.clone();

    v.append(&mut v2);
    v.push(vec![10; w]);

    let mut level = 0;
    for y in 1..=h {
        for x in 1..w - 1 {
            let hl = v[y][x];
            if v[y - 1][x] > hl && v[y + 1][x] > hl && v[y][x - 1] > hl && v[y][x + 1] > hl {
                level += hl + 1;
            }
        }
        let hl = v[y][0];
        if v[y - 1][0] > hl && v[y + 1][0] > hl && v[y][1] > hl {
            level += hl + 1;
        }
        let hl = v[y][w - 1];
        if v[y - 1][w - 1] > hl && v[y + 1][w - 1] > hl && v[y][w - 2] > hl {
            level += hl + 1;
        }
    }

    level
}

fn mapbasin(map: &Vec<Vec<u32>>, x: usize, y: usize, w: usize, h: usize) -> usize {
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    let mut search: HashSet<(usize, usize)> = HashSet::new();
    search.insert((x, y));
    while search.len() != 0 {
        let mut next = HashSet::new();
        for (xi, yi) in search.iter() {
            if *xi > 0 && map[*yi][*xi - 1] != 9 && !s.contains(&(*xi - 1, *yi)) {
                next.insert((*xi - 1, *yi));
            }
            if *xi < w - 1 && map[*yi][*xi + 1] != 9 && !s.contains(&(*xi + 1, *yi)) {
                next.insert((*xi + 1, *yi));
            }
            if map[*yi - 1][*xi] < 9 && !s.contains(&(*xi, *yi - 1)) {
                next.insert((*xi, *yi - 1));
            }
            if map[*yi + 1][*xi] < 9 && !s.contains(&(*xi, *yi + 1)) {
                next.insert((*xi, *yi + 1));
            }
        }
        s = s.union(&search).cloned().collect();
        search = next;
    }

    s.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let h = input.len();
    let w = input[0].len();

    let mut v: Vec<Vec<u32>> = vec![vec![9; w]; 1];
    let mut v2 = input.clone();

    v.append(&mut v2);
    v.push(vec![9; w]);

    let mut basins: Vec<usize> = Vec::new();
    for y in 1..=h {
        for x in 1..w - 1 {
            let hl = v[y][x];
            if v[y - 1][x] > hl && v[y + 1][x] > hl && v[y][x - 1] > hl && v[y][x + 1] > hl {
                basins.push(mapbasin(&v, x, y, w, h));
            }
        }
        let hl = v[y][0];
        if v[y - 1][0] > hl && v[y + 1][0] > hl && v[y][1] > hl {
            basins.push(mapbasin(&v, 0, y, w, h));
        }
        let hl = v[y][w - 1];
        if v[y - 1][w - 1] > hl && v[y + 1][w - 1] > hl && v[y][w - 2] > hl {
            basins.push(mapbasin(&v, w - 1, y, w, h));
        }
    }
    basins.sort_by(|a, b| b.cmp(a));
    println!("{:?}", basins);
    basins.iter().take(3).fold(1, |acc, x| acc * *x as u32)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(&generator(
                "2199943210
3987894921
9856789892
8767896789
9899965678
"
            )),
            15
        );
        assert_eq!(
            part2(&generator(
                "2199943210
3987894921
9856789892
8767896789
9899965678
"
            )),
            1134
        );
    }
}
