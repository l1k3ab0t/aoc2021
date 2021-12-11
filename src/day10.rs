use aoc_runner_derive::{aoc, aoc_generator};

use either::*;
use std::collections::HashMap;

fn check_corrupt(s: &str) -> Either<u32, &str> {
    if s.is_empty() {
        return Left(0);
    }

    let pairs: Vec<(char, char)> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    let points: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let c0 = s.chars().nth(0).unwrap();

    if points.keys().any(|k| *k == c0) {
        return Right(s);
    } else {
        match check_corrupt(&s[1..]) {
            Left(x) => return Left(x),
            Right(s2) => {
                let c1 = s2.chars().nth(0).unwrap();
                if pairs.contains(&(c0, c1)) {
                    return check_corrupt(&s2[1..]);
                } else {
                    return Left(*points.get(&c1).unwrap());
                }
            }
        }
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |acc, l| acc + check_corrupt(l.trim()).unwrap_left())
}

fn check_incomplete<'a>(s: &'a str, closures: &Vec<char>) -> Either<Vec<char>, &'a str> {
    if s.is_empty() {
        return Left(closures.to_vec());
    }

    let c0 = s.chars().nth(0).unwrap();

    if [')', ']', '}', '>'].iter().any(|k| *k == c0) {
        return Right(s);
    } else {
        match check_incomplete(&s[1..], closures) {
            Left(x) => {
                let mut v = x.clone();
                v.push(c0);
                return Left(v);
            }
            Right(s2) => return check_incomplete(&s2[1..], closures),
        }
    }
}

fn value(c: &char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let mut v: Vec<u64> = input
        .lines()
        .filter(|l| check_corrupt(l.trim()).unwrap_left() == 0)
        .map(|l| {
            check_incomplete(l.trim(), &mut Vec::new())
                .unwrap_left()
                .iter()
                .fold(0, |acc, c| acc * 5 + value(&c))
        })
        .collect();
    v.sort();
    v[v.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            26397
        );
        assert_eq!(
            part2(
                "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            ),
            288957
        );
    }
}
