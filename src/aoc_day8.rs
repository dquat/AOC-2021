use std::fs;

fn sort_string(input: &str) -> String {
    let mut sorted =
        input
            .bytes()
            .collect::<Vec<u8>>();
    sorted
        .sort_by(
            |&a, b|
                a.cmp(b)
        );

    let sorted =
        String::from_utf8(sorted)
            .unwrap();
    sorted
}

fn contains_all(input: &str, test: &str) -> bool {
    let mut num = 0;
    for c in test.chars() {
        num += input.contains(c) as usize;
    }
    num == input.len()
}

fn get_containing_input<'a, F>(input: &'a Vec<&str>, x: F) -> &'a str
where F: FnMut(&&&str) -> bool {
    input
        .iter()
        .filter(x)
        .next()
        .unwrap()
}

pub fn solve() {
    let string =
        fs::read_to_string("src/aoc-day8-input")
            .expect("Failed to read input file!");
    let mut outputs: Vec<&str>      = Vec::new();
    let mut inputs : Vec<Vec<&str>> = Vec::new();
    string
        .lines()
        .for_each(|line| {
            let mut split =
                line
                    .split("|");
            let input =
                split
                    .next()
                    .unwrap()
                    .trim()
                    .split(" ");
            let output =
                split
                    .next()
                    .unwrap()
                    .trim().
                    split(" ");
            inputs.push(input.collect::<Vec<&str>>());
            outputs.extend(output.collect::<Vec<&str>>().iter());
        });
    // part 1
    let result =
        outputs
            .iter()
            .filter(|x| x.len() == 2
                || x.len() == 3
                || x.len() == 4
                || x.len() == 7
            )
            .map(|_| 1)
            .sum::<usize>();
    println!("Result for challenge 1: {}", result);

    // part 2
    let mut result = 0;
    inputs
        .iter()
        .enumerate()
        .for_each(|(i, inp)| {

            let out =
                outputs[i * 4..(i * 4) + 4]
                    .iter()
                    .map(|&s| s)
                    .collect::<Vec<&str>>();

            let mut numbers = [""; 10];
            inp
                .iter()
                .for_each(|&x| {
                match x.len() {
                    2 => numbers[1] = x,
                    3 => numbers[7] = x,
                    4 => numbers[4] = x,
                    7 => numbers[8] = x,
                    _ => (),
                };
            });

            numbers[9] = get_containing_input(inp,
                |&&x| x.len() == 6
                    && contains_all(numbers[4], x)
            );
            numbers[0] = get_containing_input(inp,
                |&&x| x.len() == 6
                    && contains_all(numbers[7], x)
                    && x != numbers[9]
            );
            numbers[3] = get_containing_input(inp,
                |&&x| x.len() == 5
                    && contains_all(numbers[1], x)
                    && contains_all(x, numbers[9])
            );
            numbers[5] = get_containing_input(inp,
                |&&x| x.len() == 5
                    && contains_all(x, numbers[9])
                    && x != numbers[3]
            );
            numbers[6] = get_containing_input(inp,
                |&&x| x.len() == 6
                    && x != numbers[9]
                    && x != numbers[0]
            );
            numbers[6] = get_containing_input(inp,
                |&&x| x.len() == 6
                    && x != numbers[9]
                    && x != numbers[0]
            );
            numbers[2] = get_containing_input(inp,
                |&&x| x.len() == 5
                    && x != numbers[5]
                    && x != numbers[3]
            );
            let mut number = String::with_capacity(4);
            out
                .iter()
                .for_each(|&o| {
                let find =
                    numbers
                        .iter()
                        .position(|&x| {
                            sort_string(x) == sort_string(o)
                        });
                number
                    .push_str(
                        &find
                            .unwrap()
                            .to_string()
                    );
            });
            result +=
                number
                    .parse::<usize>()
                    .unwrap();
        });
    println!("Result for challenge 2: {}", result);
}