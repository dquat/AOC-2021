use std::fs;

const POINT_LEN   : usize = 100; // x - size
const POINT_LINES : usize = 100; // y - size

fn check_lowest(points: &Vec<u8>, x: usize, y: usize) -> bool {
    // back: 0, forward: 1, up: 2, down: 3
    let surround = get_surrounding(points, x, y);
    let point = points[x + y * POINT_LEN];
    let mut point_is_lowest = true;
    for (next, _, _) in surround {
        if next <= point {
            point_is_lowest = false;
            break;
        }
    }
    point_is_lowest
}

fn get_surrounding(points: &Vec<u8>, x: usize, y: usize) -> [(u8, usize, usize); 4] {
    let mut surrounding = [(u8::MAX, usize::MAX, usize::MAX); 4];
    if x != 0              { surrounding[0] = (points[(x - 1) + y * POINT_LEN], x - 1, y) }
    if x + 1 < POINT_LEN   { surrounding[1] = (points[(x + 1) + y * POINT_LEN], x + 1, y) }
    if y != 0              { surrounding[2] = (points[x + (y - 1) * POINT_LEN], x, y - 1) }
    if y + 1 < POINT_LINES { surrounding[3] = (points[x + (y + 1) * POINT_LEN], x, y + 1) }
    surrounding
}

pub fn get_lowest(points: &Vec<u8>) -> Vec<u8> {
    let mut lowest = Vec::new();
    for x in 0..POINT_LEN {
        for y in 0..POINT_LINES {
            if check_lowest(points, x, y) {
                lowest.push(points[x + y * POINT_LEN]);
            }
        }
    }
    lowest
}

pub fn get_not_9(
    mut surrounding: Vec<(u8, usize, usize)>,
    points: &Vec<u8>,
    x: usize,
    y: usize
) -> Vec<(u8, usize, usize)> {
    for (s, x, y) in get_surrounding(points, x, y) {
        if s < 9u8 && !surrounding.contains(&(s, x, y)) {
            surrounding.push((s, x, y));
            surrounding = get_not_9(surrounding, points, x, y);
        }
    }
    surrounding
}

pub fn get_basins(points: &Vec<u8>) -> Vec<Vec<(u8, usize, usize)>> {
    let mut surrounded = Vec::new();
    for x in 0..POINT_LEN {
        for y in 0..POINT_LINES {
            if check_lowest(&points, x, y) {
                surrounded.push(get_not_9(vec![(points[x + y * POINT_LEN], x, y)], &points, x, y));
            }
        }
    }
    surrounded
}

pub fn solve() {
    let string =
        fs::read_to_string("src/aoc-day9-input")
            .expect("Failed to read input file!");

    // parse
    let mut points = Vec::with_capacity(POINT_LEN * POINT_LINES);
    string
        .lines()
        .for_each(|line|
            line
                .bytes()
                .for_each(|b| {
                    points.push(b - 48); // convert to integer
                })
        );

    // part 1
    let lowest =
        get_lowest(&points)
            .iter()
            .fold(
                0usize,
                |acc, &x|
                    acc + (x as usize + 1)
            );
    println!("Result for challenge 1: {}", lowest);

    // part 2
    let basins = get_basins(&points);
    let mut largest = [0; 3];
    basins
        .iter()
        .for_each(|basin| {
        for l in largest.iter_mut() {
            let len = basin.len();
            if len > *l {
                *l = len;
                break;
            }
        }
    });
    println!("Result of challenge 2: {}",
             largest
                 .iter()
                 .product::<usize>()
    );
}