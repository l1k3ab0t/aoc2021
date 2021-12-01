use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .fold((-1, -1), |(acc, pre), x| (acc + (pre < x) as i32, x))
        .0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let v: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    v.iter().zip(v[3..].iter()).filter(|x| x.0 < x.1).count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(
                "199
200
208
210
200
207
240
269
260
263",
            ),
            7,
        );
        assert_eq!(
            part2(
                "199
200
208
210
200
207
240
269
260
263",
            ),
            5,
        );
    }
}
