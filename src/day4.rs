use aoc_runner_derive::{aoc, aoc_generator};

pub struct Bingo {
    boards: Vec<Board>,
    instructions: Vec<i32>,
}

pub struct Board(Vec<Vec<(i32, bool)>>);

impl Board {
    fn score(&self) -> i32 {
        self.0
            .iter()
            .flatten()
            .fold(0, |acc, (i, m)| if !*m { acc + *i } else { acc })
    }

    fn mark(&self, x: i32) -> Self {
        Board(
            self.0
                .iter()
                .map(|l| {
                    l.iter()
                        .map(|(i, m)| if *i == x { (*i, true) } else { (*i, *m) })
                        .collect()
                })
                .collect(),
        )
    }

    fn won(&self) -> bool {
        let mut v = vec![0, 0, 0, 0, 0];
        for l in self.0.iter() {
            if l.iter().fold(0, |acc2, (_, m)| acc2 + (*m as i32)) == 5 {
                return true;
            }
            v = l
                .iter()
                .enumerate()
                .map(|(i, (_, m))| v[i] + (*m as i32))
                .collect();
        }
        v.contains(&5)
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board(self.0.to_owned())
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Bingo {
    let mut lines = input.lines();
    let instructions = &lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .clone();
    let bingo = lines.filter(|l| l.len() > 3).collect::<Vec<&str>>()[0..]
        .chunks(5)
        .into_iter()
        .map(|c| {
            Board(
                c.to_owned()
                    .iter()
                    .map(|l| {
                        l.split_ascii_whitespace()
                            .map(|c| (c.parse::<i32>().unwrap(), false))
                            .collect::<Vec<(i32, bool)>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<Board>>();

    Bingo {
        boards: bingo,
        instructions: instructions.to_vec(),
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &Bingo) -> i32 {
    let mut boards = input.boards.to_vec();
    for x in input.instructions.iter() {
        let mut bs: Vec<Board> = vec![];
        for board in boards.iter() {
            let marked = board.mark(*x);
            if marked.won() {
                return marked.score() * x;
            }
            bs.push(marked);
        }
        boards = bs.clone();
    }
    0
}

#[aoc(day4, part2)]
pub fn part2(input: &Bingo) -> i32 {
    let mut boards = input.boards.to_vec();
    for x in input.instructions.iter() {
        let mut bs: Vec<Board> = vec![];
        let mut last_score = 0;
        for board in boards.iter() {
            let marked = board.mark(*x);
            if !marked.won() {
                bs.push(marked)
            } else {
                last_score = marked.score() * x
            }
        }
        boards = bs.clone();
        if boards.len() == 0 {
            return last_score;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn sample() {
        assert_eq!(
            part1(&generator(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            )),
            4512
        );
        assert_eq!(
            part2(&generator(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            )),
            1924
        );
    }
}
