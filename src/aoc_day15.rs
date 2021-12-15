use std::fs;

const WIDTH  : usize = 100;
const HEIGHT : usize = 100;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x : usize,
    y : usize,
}

impl Point {
    fn new   (x: usize, y: usize)         -> Point { Point { x, y } }
    fn index (&self, width: usize)        -> usize { self.y * width + self.x }
    fn from  (index: usize, width: usize) -> Point { Point::new(index % width, index / width) }
}

fn get_next(point: Point, width: usize) -> [usize; 4] {
    let (x, y) = (point.x as isize, point.y as isize);
    [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
    ]
        .map(|(x, y)| {
            let (x, y) = (x as usize, y as usize);
            if x < width {
                Point::new(x, y).index(width)
            } else {
                usize::MAX
            }
        })
}

fn get_shortest(points: &[u8], width: usize) -> usize {
    let len = points.len();
    let mut queue =
        Vec::with_capacity(len);
    queue.push((0, 0usize));
    let mut visited =
        Vec::with_capacity(len);
    while queue.len() > 0 {
        let (point, mut cost) = queue.pop().unwrap();
        if len - 1 == point {
            return cost;
        }
        let directions =
            get_next(Point::from(point, width), width);
        directions
            .map(|idx| {
                if len > idx && !visited.contains(&idx) {
                    visited.push(idx);
                    queue.push((idx, cost + points[idx] as usize));
                }
            });
        queue.sort_by_cached_key(|&(_, a)| -(a as isize)); // sort by descending order
    }
    unreachable!()
}

pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day15-input")
            .expect("Failed to read input file!");
    let mut array = [0; WIDTH * HEIGHT];
    string
        .lines()
        .collect::<String>()
        .bytes()
        .enumerate()
        .for_each(|(i, b)| {
            array[i] = b - 48;
        });
    println!("Result of challenge 1: {}", get_shortest(&array, WIDTH));
    let mut array = Vec::with_capacity(WIDTH * HEIGHT * 25);
    for i in 0..5 {
        string
            .lines()
            .enumerate()
            .for_each(|(idx, line)| {
                for j in 0..5 {
                    line
                        .bytes()
                        .enumerate()
                        .for_each(|(bidx, b)| {
                            let val = (b - 48) + i as u8 + j as u8;
                            array.push(if val > 9 { val - 9 } else { val });
                        });
                }
            });
    }
    println!("Result of challenge 2: {}", get_shortest(array.as_slice(), WIDTH * 5));
}