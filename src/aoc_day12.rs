use std::collections::HashMap;
use std::fs;

fn gen_tree<'a>(
    hm         : &HashMap<&'a str, Vec<&'a str>>,
    node       : &'a str,
    mut seen   : Vec<&'a str>,
    mut twice  : bool,
    filter     : bool,
) -> usize {
    if seen.contains(&node) {
        if !twice {
            return 0;
        } else {
            twice = false;
        }
    }
    if node == "end" { return 1; }
    if node == node.to_ascii_lowercase() { seen.push(node); }
    hm[node]
        .iter()
        .filter(
            |&&x|
                if filter {
                    x != "start"
                } else {
                    true
                })
        .map(
            |&n|
                gen_tree(
                    hm,
                    n,
                    seen.clone(),
                    twice,
                    filter
                )
        )
        .sum()
}

pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day12-input")
            .expect("Failed to read input file!");

    let mut cave_systems = HashMap::new();
    string
        .lines()
        .for_each(|line| {
            let split = line.split_once('-').unwrap();
            cave_systems
                .entry(split.0)
                .or_insert(Vec::new())
                .push(split.1);
            cave_systems
                .entry(split.1)
                .or_insert(Vec::new())
                .push(split.0);
        });
    // part 1
    println!("Result of challenge 1: {}",
             gen_tree(
                 &cave_systems,
                 "start",
                 Vec::new(),
                 false,
                 false
             )
    );
    // part 2
    println!("Result of challenge 2: {}",
             gen_tree(
                 &cave_systems,
                 "start",
                 Vec::new(),
                 true,
                 true
             )
    );
}