use std::fs;

fn count_fishes(input: &String, days: usize) -> usize {
    let mut fish_count = [0usize; 9];
    let mut tmp_count = [0; 9];
    input
        .split(',')
        .for_each(|x| {
            let i = x.parse::<usize>().unwrap();
            fish_count[i] += 1;
        });
    for _ in 0..days {
        let zero = fish_count[0];
        for (i, &count) in fish_count.iter().skip(1).enumerate() {
            tmp_count[i] = count;
        }
        fish_count = tmp_count;
        fish_count[8] = zero;
        fish_count[6] += zero;
    }
    let mut total_fishes = 0;
    for count in fish_count.iter() {
        total_fishes += count;
    }
   total_fishes
}

pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day6-input")
            .expect("Failed to read input file!");
    // part 1
    println!("Result for challenge 1: {}", count_fishes(&string, 80));
    // part 2
    println!("Result for challenge 2: {}", count_fishes(&string, 256));
}