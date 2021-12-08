use std::fs;

fn get_fuel(positions: &Vec<usize>, constant: bool) -> usize {
    let &max = positions.iter().max().unwrap();
    let &min = positions.iter().min().unwrap();
    let mut min_fuel = usize::MAX;
    for i in min..max {
        let mut fuel = 0;
        positions
            .iter()
            .for_each(|&p| {
                if constant {
                    fuel += isize::abs(i as isize - p as isize) as usize;
                } else {
                    fuel += (1..=isize::abs(i as isize - p as isize) as usize).sum::<usize>();
                }
            });
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

pub fn solve() {
    let string =
        fs::read_to_string("src/aoc-day7-input")
            .expect("Failed to read input file!");
    let positions =
        string
            .split(",")
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    println!("Result for challenge 1: {}", get_fuel(&positions, true));
    println!("Result for challenge 2: {}", get_fuel(&positions, false));
}