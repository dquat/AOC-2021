use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(string: &String, allow_diag: bool) -> [((isize, isize), (isize, isize)); 500] {
    let mut lines = [((0, 0), (0, 0)); 500];
    // parse all points
    string
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            let (start, end) =
                line
                    .split_once("->")
                    .unwrap();
            let ((sx, sy), (ex, ey)) = (
                start
                    .trim()
                    .split_once(",")
                    .unwrap(),
                end
                    .trim()
                    .split_once(",")
                    .unwrap(),
            );
            if allow_diag || (sx == ex || sy == ey) {
                let (sx, sy, ex, ey) = (
                    // use isize because usize is annoying
                    sx.parse::<isize>().unwrap(),
                    sy.parse::<isize>().unwrap(),
                    ex.parse::<isize>().unwrap(),
                    ey.parse::<isize>().unwrap()
                );
                lines[i] = ((sx, sy), (ex, ey));
            }
        });
    lines
}

fn get_line_points((
        (sx, sy),
        (ex, ey)
    ): (
        (isize, isize),
        (isize, isize)
    )
) -> Vec<(isize, isize)> {
    // a vector of the slope
    let vec = (
        (ex - sx),
        (ey - sy)
    );
    let mut points = vec![(sx, sy)];
    // calculate the normal of the vector
    // vec / vec_len = vec normal (we need this because the slope factor needs to be in unit-length)
    let vec_len = ((
        vec.0.pow(2) + vec.1.pow(2)
    ) as f64).sqrt();
    // get the slope, and round the values, because they would be zero otherwise
    let (slope_x, slope_y) = (
        (vec.0 as f64 / vec_len).round() as isize,
        (vec.1 as f64 / vec_len).round() as isize
    );
    // start off with the start points
    let (mut x, mut y) = (sx, sy);
    // since we are dealing with integers and only 45 degree values,
    // this condition works in all cases
    while x != ex || y != ey {
        // add the slope to the current point
        x += slope_x;
        y += slope_y;
        points.push((x, y));
    }
    points
}

fn calculate_intersections(lines: &[((isize, isize), (isize, isize))]) -> usize {
    let mut point_count: HashMap<(isize, isize), usize> = HashMap::with_capacity(lines.len());
    for &line in lines {
        // generate points for each line
        get_line_points(line)
            .iter()
            .for_each(|&point| {
                // if point is seen, add a point to the point count
                if point_count.contains_key(&point) {
                    let mut point = point_count.get_mut(&point).unwrap();
                    *point += 1;
                } else {
                    // if not, create the point
                    point_count.insert(point, 1);
                }
            });
    }
    // filter all the points that have no intersections
    point_count.values().filter(|&&x| x >= 2).count()
}

pub fn aoc_day5() {
    let string =
        fs::read_to_string("src/aoc-day5-input")
            .expect("Failed to read input file!");
    let time = std::time::Instant::now();
    // part 1
    let lines = parse_input(&string, false);
    println!("Result of challenge 1: {}", calculate_intersections(&lines));

    // part 2
    let lines = parse_input(&string, true);
    println!("Result of challenge 2: {}", calculate_intersections(&lines));
    println!("Time: {:?}", time.elapsed());
}