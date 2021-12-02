use std::fs;

pub fn aoc_day2() {
    let lines =
        fs::read_to_string("src/aoc-day2-input")
            .expect("Failed to read input file!")
            .lines();

    // part 1
    let (mut horizontal, mut depth) = (0, 0);
    lines
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
    lines
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