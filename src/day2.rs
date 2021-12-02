use aoc_runner_derive::{aoc, aoc_generator};

pub enum Inst {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Inst> {
    input
        .lines()
        .map(|l| l.trim().split(" ").collect())
        .map(|s: Vec<&str>| match s[0] {
            "forward" => Inst::Forward(s[1].parse::<i32>().unwrap()),
            "down" => Inst::Down(s[1].parse::<i32>().unwrap()),
            "up" => Inst::Up(s[1].parse::<i32>().unwrap()),
            _ => unreachable![],
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Inst>) -> i32 {
    input
        .iter()
        .fold([0, 0], |[h, d], i| match i {
            Inst::Forward(v) => [h, d + v],
            Inst::Down(v) => [h + v, d],
            Inst::Up(v) => [h - v, d],
        })
        .iter()
        .product()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Inst>) -> i32 {
    input.iter().fold([0, 0, 0], |[a, h, d], i| match i {
        Inst::Forward(v) => [a, h + v, d + a * v],
        Inst::Down(v) => [a + v, h, d],
        Inst::Up(v) => [a - v, h, d],
    })[1..]
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(&generator(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2
"
            )),
            150
        );
        assert_eq!(
            part2(&generator(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2
"
            )),
            900
        );
    }
}
