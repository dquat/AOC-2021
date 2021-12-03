use std::fs;
use std::slice::Iter;

fn occurrences(string: Iter<&str>) -> ([i32; 12], [i32; 12]) {
    let (mut num_zeroes, mut num_ones) = (
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );

    string.for_each(|line| {
        line
            .bytes()
            .enumerate()
            .for_each(|(i, b)|
                match b {
                    b'0' => num_zeroes[i] += 1,
                    b'1' => num_ones[i] += 1,
                    _ => unreachable!(),
                }
            );
    });
    (num_zeroes, num_ones)
}

pub fn aoc_day3() {
    let string =
        fs::read_to_string("src/aoc-day3-input")
            .expect("Failed to read input file!");

    // part 1
    let (num_zeroes, num_ones) =
        occurrences(
            string
                .lines()
                .collect::<Vec<_>>()
                .iter()
        );

    let (mut gamma_rate, mut epsilon_rate) = (String::with_capacity(12), String::with_capacity(12));
    for i in 0..12 {
        let (zero, one) = (num_zeroes[i], num_ones[i]);
        if one > zero {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    println!(
        "Result of challenge 1 is: {}",
            isize::from_str_radix(&*gamma_rate, 2).unwrap()
            * isize::from_str_radix(&*epsilon_rate, 2).unwrap()
    );

    // part 2
    let mut result = 1;
    for ty in 0..2 {
        let mut remaining: Vec<&str> =
            string
                .lines()
                .collect();
        for i in 0..12 {
            if remaining.len() == 1 {
                break;
            }
            let (num_zeroes, num_ones) =
                occurrences(remaining.iter());
            let (zero, one) = (num_zeroes[i], num_ones[i]);
            remaining.retain(|&line| {
                let byte =
                    line
                        .bytes()
                        .nth(i)
                        .unwrap();
                if ty == 0
                    && (one > zero && byte == b'1'
                    || one < zero && byte == b'0'
                    || one == zero && byte == b'1')
                    || ty == 1
                    && (one > zero && byte == b'0'
                    || one < zero && byte == b'1'
                    || one == zero && byte == b'0')
                { false }
                else { true }
            });
        }
        result *= isize::from_str_radix(remaining[0], 2).unwrap();
    }
    println!("Result of challenge 2 is: {}", result);
}