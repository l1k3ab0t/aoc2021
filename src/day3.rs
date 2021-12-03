use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let c = input[0].len();
    let gamma = input
        .iter()
        .fold(vec![0; c], |a: Vec<i32>, x: &Vec<i32>| {
            a.iter().enumerate().map(|(i, y)| x[i] + y).collect()
        })
        .iter()
        .map(|x| x * 2 >= input.len() as i32)
        .fold(0, |acc, x| (acc << 1) + (x as i32));

    gamma * (gamma ^ ((1 << c) - 1))
}

pub fn count_index(input: &Vec<Vec<i32>>, i: usize) -> usize {
    input.iter().fold(0, |acc, x| acc + x[i] as usize)
}

pub fn filter_step(input: &Vec<Vec<i32>>, step: usize, cmp: fn(usize, usize) -> bool) -> i32 {
    if input.len() == 1 {
        return input[0].iter().fold(0, |acc, x| (acc << 1) + x);
    }

    if cmp(2 * count_index(input, step), input.len()) {
        filter_step(
            &input
                .into_iter()
                .filter(|x| x[step] == 0)
                .cloned()
                .collect::<Vec<_>>(),
            step + 1,
            cmp,
        )
    } else {
        filter_step(
            &input
                .into_iter()
                .filter(|x| x[step] == 1)
                .cloned()
                .collect::<Vec<_>>(),
            step + 1,
            cmp,
        )
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    filter_step(input, 0, |x, y| x < y) * filter_step(input, 0, |x, y| x >= y)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(&generator(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            198
        );
        assert_eq!(
            part2(&generator(
                "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            )),
            230
        );
    }
}
