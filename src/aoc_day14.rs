use std::collections::HashMap;
use std::fs;

fn calc_chars(
    start_polymer : String,
    rules         : &HashMap<&str, char>
) -> (HashMap<char, usize>, HashMap<String, usize>) {
    let mut occurrences =
        HashMap::with_capacity(start_polymer.len());
    let mut poly_chars =
        start_polymer
            .chars();
    let mut prev =
        poly_chars
            .next()
            .unwrap();
    let mut ch_occurrences =
        HashMap::with_capacity(occurrences.capacity() * 2);
    ch_occurrences.insert(prev, 1);
    poly_chars.for_each(|c| {
        let entry =
            ch_occurrences
                .entry(c)
                .or_insert(0);
        *entry += 1;
        let string = format!("{}{}", prev, c);
        if let Some(&rule) = rules.get(&string.as_str()) {
            let entry =
                occurrences
                    .entry(string)
                    .or_insert(0);
            *entry += 1;
        }
        prev = c;
    });
    (ch_occurrences, occurrences)
}

fn solve_days(
    ch_occurrences : &mut HashMap<char, usize>,
    occurrences    : &mut HashMap<String, usize>,
    rules          : &HashMap<&str, char>,
    steps          : usize,
) -> usize {
    for _step in 0..steps {
        let mut add = Vec::with_capacity(occurrences.capacity() * 2);
        occurrences
            .iter_mut()
            .for_each(|(key, val)| {
                if *val > 0 {
                    let &rule =
                        rules
                            .get(&key.as_str())
                            .unwrap();
                    let mut new_key = key.clone();
                    new_key.insert(1, rule);
                    add.push((
                        new_key
                            .chars()
                            .take(2)
                            .collect::<String>(),
                        *val
                    ));
                    add.push((
                        new_key
                            .chars()
                            .skip(1)
                            .collect::<String>(),
                        *val
                    ));
                    let entry =
                        ch_occurrences
                            .entry(rule)
                            .or_insert(0);
                    *entry += *val;
                    *val = 0;
                }
            });
        add
            .iter()
            .for_each(|(key, val)| {
                let entry =
                    occurrences
                        .entry(key.clone())
                        .or_insert(0);
                *entry += *val;
            });
    }
    ch_occurrences
        .values()
        .max()
        .unwrap()
        -
    ch_occurrences
        .values()
        .min()
        .unwrap()
}

pub fn solve() {
    let string = //SAMPLE_INPUT.to_string();
        fs::read_to_string("src/inputs/aoc-day14-input")
            .expect("Failed to read input file!");
    let mut lines = string.lines();
    let start_polymer =
        lines
            .next()
            .unwrap()
            .to_string();
    lines.next(); // skip the newline
    let mut rules = HashMap::new();
    lines.for_each(|line| {
        let (check, rule) =
            line
                .split_once(" -> ")
                .unwrap();
        rules.insert(
            check,
            rule
                .chars()
                .next()
                .unwrap()
        );
    });
    let (
        mut ch_occurrences,
        mut occurrences
    ) = calc_chars(start_polymer, &rules);
    let part1 = solve_days(&mut ch_occurrences, &mut occurrences, &rules, 10);
    let part2 = solve_days(&mut ch_occurrences, &mut occurrences, &rules, 30);
    println!("Result of challenge 1: {}", part1);
    println!("Result of challenge 2: {}", part2);
}