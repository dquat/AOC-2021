use std::collections::HashMap;
use std::fs;

struct Tree<'a> {
    seen : Vec<&'a str>,
    hm   : &'a HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Tree<'a> {
    fn new(hm : &'a HashMap<&'a str, Vec<&'a str>>) -> Tree<'a> {
        Tree { seen : Vec::new(), hm }
    }

    fn gen_tree(
        &mut self,
        node       : &'a str,
        mut twice  : bool,
        filter     : bool,
    ) -> usize {
        if self.seen.contains(&node) {
            if !twice {
                return 0;
            } else {
                twice = false;
            }
        }
        if node == "end" { return 1; }
        if node == node.to_ascii_lowercase() { self.seen.push(node); }
        self.hm[node]
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
                    self.gen_tree(
                        n,
                        twice,
                        filter
                    )
            )
            .sum()
    }
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
    let mut tree = Tree::new(&cave_systems);
    println!("Result of challenge 1: {}",
             tree.gen_tree("start", false, false)
    );
    // part 2
    let mut tree = Tree::new(&cave_systems);
    println!("Result of challenge 2: {}",
             tree.gen_tree("start", true, true)
    );
}