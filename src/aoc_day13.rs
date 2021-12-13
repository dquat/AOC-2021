use std::collections::HashSet;
use std::fs;
use FoldDirection::*;

const NUM_POINTS    : usize =  782 ; // the number of points in my input is 782, probably the same f
const NUM_FOLDS     : usize =   12 ; // number of folds in my input is 12, probably the same for others too
const SKIP_LEN      : usize =   11 ; // length of `fold along ` = 11
const COLOR_EFFECTS : bool  = true ; // set this to false, if you get unreadable / uncool output from part 2, ie your console does not support rgb colors

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(input: &str) -> Point {
        let (x, y) =
            input
                .split_once(",")
                .unwrap();
        let (x, y) = (
            x
                .trim()
                .parse::<usize>()
                .unwrap(),
            y
                .trim()
                .parse::<usize>()
                .unwrap()
        );
        Point{ x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum FoldDirection { X, Y, }

#[derive(Debug, Clone, Copy)]
struct Fold {
    direction : FoldDirection,
    value     : usize,
}

impl Fold {
    fn parse(input: &str) -> Fold {
        let mut chars = input.chars();
        let fold_dir =
            if chars.next().unwrap() == 'x' {
                X
            } else {
                Y
            };
        chars.next(); // skip '='
        let value =
            chars
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
        Fold{ direction: fold_dir, value }
    }
}

fn print_points(points: &[Point], width: usize, height: usize) {
    use crossterm::{execute, style::{Color, SetForegroundColor, ResetColor},};
    for y in 0..height {
        for x in 0..width {
            // Ooh some cool color effects!
            if points.contains(&Point { x, y }) {
                if COLOR_EFFECTS {
                    let color = Color::Rgb {
                        r: 150,
                        g: ((x as f64 / width as f64) * 205.0) as u8 + 50,
                        b: ((y as f64 / height as f64) * 205.0) as u8 + 50,
                    };
                    execute!(std::io::stdout(), SetForegroundColor(color)).unwrap();
                    print!("\u{2588}\u{2588}"); // ██
                    execute!(std::io::stdout(), ResetColor).unwrap();
                } else {
                    print!("\u{2588}\u{2588}"); // ██
                }
            } else {
                if COLOR_EFFECTS {
                    let c = ((((width - x) + (height - y)) as f64 / (width + height) as f64) * 255.0) as u8;
                    let color = Color::Rgb {
                        r: c,
                        g: c,
                        b: c,
                    };
                    execute!(std::io::stdout(), SetForegroundColor(color)).unwrap();
                    print!("\u{2588}\u{2588}"); // ██
                    execute!(std::io::stdout(), ResetColor).unwrap();
                } else {
                    print!("  ");
                }
            }
        }
        println!();
    }
}

fn fold_points(fold: Fold, points: &[Point]) -> [Point; NUM_POINTS] {
    let mut new_points = [Point { x: 0, y: 0 }; NUM_POINTS];
    points
        .iter()
        .enumerate()
        .for_each(|(i, point)| {
            let mut folded = false;
            match fold.direction {
                X => {
                    if point.x > fold.value {
                        new_points[i] = Point { x: fold.value - (point.x - fold.value), y: point.y };
                        folded = true;
                    }
                },
                Y => {
                    if point.y > fold.value {
                        new_points[i] = Point { x: point.x, y: fold.value - (point.y - fold.value) };
                        folded = true;
                    }
                },
            }
            if !folded { new_points[i] = *point; }
    });
    new_points
}

pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day13-input")
            .expect("Failed to read input file!");
    let mut points = [Point { x: 0, y: 0 }; NUM_POINTS];
    let mut lines =
        string
            .lines()
            .peekable();

    // parse points
    let mut i = 0;
    while lines.peek() != Some(&"") {
        let line =
            lines
                .next()
                .unwrap();
        points[i] = Point::parse(line);
        i += 1;
    }
    lines.next();

    // parse folds
    let mut folds = [Fold { direction: X, value: 0 }; NUM_FOLDS];
    lines
        .enumerate()
        .for_each(|(i, line)| {
        let fold = line.split_at(SKIP_LEN).1;
        folds[i] = Fold::parse(fold);
    });

    // part 1
    let first_points = fold_points(folds[0], &points);
    let mut seen = HashSet::new();
    let num =
        first_points
            .iter()
            .filter(|&p| seen.insert(p))
            .count();
    println!("Result of challenge 1: {}", num);

    // part 2
    folds
        .iter()
        .for_each(|&f| {
        points = fold_points(f, &points);
    });
    println!("Result of challenge 2:");
    print_points(&points, 39, 6);
}