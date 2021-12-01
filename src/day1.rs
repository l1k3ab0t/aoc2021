use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .fold((-1, -1), |t, x| {
            let (acc, pre) = t;
            if x > pre {
                (acc + 1, x)
            } else {
                (acc, x)
            }
        })
        .0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let v: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut r = 0;
    for i in 1..v.len() - 2 {
        if v[i - 1] < v[i + 2] {
            r += 1;
        }
    }
    r
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
