use std::fs;

fn aoc_day1() {
    let string =
        fs::read_to_string("src/aoc-day1-input")
            .expect("Failed to read input file!");
    // get file contents as a vector of u-ints
    let measurements =
            string.lines()
            .map(|x| str::parse(x).unwrap())
            .collect::<Vec<u32>>();

    let mut prev = u32::MAX;

    // part 1
    let challenge_1 =
        measurements
        .iter()
        .filter(|&&x| {
            let result = x > prev;
            prev = x;
            result
        })
        .count();
    println!("Result of challenge 1 is: {}", challenge_1);

    // part 2
    let mut prev = [u32::MAX, u32::MAX, u32::MAX];
    let challenge_2 =
        measurements
        .iter()
            .enumerate()
            .filter(|&(i, &x)| {
                let index = (i + 1) % 3;
                let result = x > prev[index];
                prev[index] = x;
                result
            })
            .count();
    println!("Result of challenge 2 is: {}", challenge_2);
}

fn aoc_day2() {
    let string =
        fs::read_to_string("src/aoc-day2-input")
            .expect("Failed to read input file!");

    // part 1
    let (mut horizontal, mut depth) = (0, 0);
    string
        .lines()
        .for_each(|line| {
            let (lhs, number) = line.split_once(" ").unwrap();
            let number = number.parse::<i32>().unwrap();
            match lhs {
                "forward" => horizontal += number,
                "up"      => depth -= number,
                "down"    => depth += number,
                _         => unreachable!(),
            };
        });
    println!("Result of challenge 1 is: {}", horizontal * depth);

    // part 2
    let (mut horizontal, mut depth, mut aim) = (0, 0, 0);
    string
        .lines()
        .for_each(|line| {
            let (lhs, number) = line.split_once(" ").unwrap();
            let number = number.parse::<i32>().unwrap();
            match lhs {
                "forward" => {
                    horizontal += number;
                    depth += number * aim;
                }
                "up"      => aim -= number,
                "down"    => aim += number,
                _         => unreachable!(),
            };
        });
    println!("Result of challenge 2 is: {}", horizontal * depth);
}

fn main() {
    aoc_day1();
    aoc_day2();
}
