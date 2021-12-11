use std::fs;

fn get_score(line: &str) -> (usize, Vec<char>) {
    let mut check = Vec::with_capacity(line.len());
    let mut score: usize = 0;
    for b in line.bytes() {
        match b {
            b'(' => check.push(')'),
            b'[' => check.push(']'),
            b'{' => check.push('}'),
            b'<' => check.push('>'),

            b')' => if check.pop() != Some(')') { score +=     3; },
            b']' => if check.pop() != Some(']') { score +=    57; },
            b'}' => if check.pop() != Some('}') { score +=  1197; },
            b'>' => if check.pop() != Some('>') { score += 25137; },

            _ => unreachable!(),
        }
    }
    (score, check)
}

fn get_incomplete(line: &str) -> usize {
    let (_, mut remaining) = get_score(line);
    let mut score = 0;
    remaining
        .iter()
        .rev() // important lol, I got stuck on this one (not for long!)
        .for_each(|&x| match x {
            ')' => score = (5 * score) + 1,
            ']' => score = (5 * score) + 2,
            '}' => score = (5 * score) + 3,
            '>' => score = (5 * score) + 4,
            _ => unreachable!()
        });
    score
}

pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day10-input")
            .expect("Failed to read input file!");

    let score =
    string
        .lines()
        .fold(
            0,
            |acc, line|
                acc + get_score(line).0
        );
    println!("Result of challenge 1: {}", score);

    let mut scores = Vec::with_capacity(string.lines().count());
    string
        .lines()
        .filter(
            |&line|
                get_score(line).0 == 0
        )
        .for_each(
            |line|
                scores.push(
                    get_incomplete(line)
                )
        );
    scores.sort();
    println!("Result of challenge 2: {}", scores[scores.len() / 2]);
}