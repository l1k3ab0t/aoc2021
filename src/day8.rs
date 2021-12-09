use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashSet;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        for segment in line.split(" | ").collect::<Vec<_>>()[1].trim().split(" ") {
            let l = segment.len();
            if l == 2 || l == 3 || l == 4 || l == 7 {
                result += 1;
            }
        }
    }

    result
}

fn decode(input: &str, output: &str) -> u32 {
    let segments: Vec<_> = input.trim().split(" ").collect();
    let s1: HashSet<char> = segments
        .iter()
        .find(|s| s.len() == 2)
        .unwrap()
        .chars()
        .collect();
    let s2: HashSet<char> = segments
        .iter()
        .find(|s| s.len() == 4)
        .unwrap()
        .chars()
        .collect();

    convert(output, &s1, &s2)
}

fn convert(output: &str, dec1: &HashSet<char>, dec2: &HashSet<char>) -> u32 {
    let mut s = String::with_capacity(4);

    for w in output.split(" ") {
        let cs: HashSet<char> = w.chars().collect();
        let intersec1 = dec1.intersection(&cs).count();
        let intersec2 = dec2.intersection(&cs).count();

        match (w.len(), intersec1, intersec2) {
            (2, _, _) => s.push('1'),
            (5, 1, 2) => s.push('2'),
            (5, 2, 3) => s.push('3'),
            (4, _, _) => s.push('4'),
            (5, 1, 3) => s.push('5'),
            (6, 1, 3) => s.push('6'),
            (3, _, _) => s.push('7'),
            (7, _, _) => s.push('8'),
            (6, 2, 4) => s.push('9'),
            (6, 2, 3) => s.push('0'),
            _ => unreachable!(),
        }
    }
    s.parse().unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let segments: Vec<_> = line.split(" | ").collect();
        let x = decode(segments[0], segments[1]);
        result += x;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            ),
            26
        );
        assert_eq!(
            part2("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            ),
            61229
        );
    }
}
