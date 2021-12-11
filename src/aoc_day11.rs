use std::fs;

const WIDTH  : usize = 10;
const HEIGHT : usize = 10;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    #[inline]
    fn new(x: usize, y: usize) -> Point { Point { x, y }                               }
    #[inline]
    fn index(&self)            -> usize { self.y * WIDTH + self.x                      }
    #[inline]
    fn from(index: usize)      -> Point { Point { x: index % WIDTH, y: index / WIDTH } }
    #[inline]
    fn left(&self)             -> Point { Point::new(self.x - 1, self.y)            }
    #[inline]
    fn left_down(&self)        -> Point { Point::new(self.x - 1, self.y + 1)     }
    #[inline]
    fn left_up(&self)          -> Point { Point::new(self.x - 1, self.y - 1)     }
    #[inline]
    fn right(&self)            -> Point { Point::new(self.x + 1, self.y)            }
    #[inline]
    fn right_down(&self)       -> Point { Point::new(self.x + 1, self.y + 1)     }
    #[inline]
    fn right_up(&self)         -> Point { Point::new(self.x + 1, self.y - 1)     }
    #[inline]
    fn up(&self)               -> Point { Point::new(self.x, self.y - 1)            }
    #[inline]
    fn down(&self)             -> Point { Point::new(self.x, self.y + 1)            }
}

#[derive(Debug, Clone, Copy)]
struct Octo {
    energy   : [u8  ; WIDTH * HEIGHT],
    flashed  : [bool; WIDTH * HEIGHT],
    flashes  : usize,
    was_full : bool,
}

impl Octo {
    fn new(energy: [u8; 100]) -> Octo {
        Octo {
            energy,
            flashed  : [false; WIDTH * HEIGHT],
            flashes  : 0,
            was_full : false,
        }
    }

    #[inline]
    fn step(&mut self) {
        for i in 0..(WIDTH * HEIGHT) {
            if !self.flashed[i] {
                self.energy[i] += 1;
                self.check_flash(i);
            }
        }
        self.was_full =
            self.flashed
                .iter()
                .all(|&x| x);
        self.flashed = [false; WIDTH * HEIGHT];
    }

    fn check_flash(&mut self, index: usize) {
        let p = &mut self.energy[index];
        let flashed = &mut self.flashed[index];
        if *p > 9 && !*flashed {
            self.flashes += 1;
            *flashed = true;
            *p = 0;
            for x in self.get_adjacent(Point::from(index)) {
                if x == 255 { continue; }
                if !self.flashed[x] {
                    self.energy[x] += 1;
                    self.check_flash(x);
                }
            }
        }
    }

    #[inline]
    fn get_adjacent(&self, p: Point) -> [usize; 8] {
        assert!(p.x < 10 && p.y < 10);
        // 0: left, 1: right, 2: up, 3: down, 4: right_up, 5: right_down, 6: left_up, 7: left_down
        let mut adjacent = [255; 8];

        // normal
        if p.x > 0                                // left
        { adjacent[0] = p.left ().index(); }
        if p.x + 1 < WIDTH                        // right
        { adjacent[1] = p.right().index(); }
        if p.y > 0                                // up
        { adjacent[2] = p.up   ().index(); }
        if p.y + 1 < HEIGHT                       // down
        { adjacent[3] = p.down ().index(); }

        // diagonals
        if p.x + 1 < WIDTH && p.y > 0             // right, diagonal up
        { adjacent[4] = p.right_up  ().index(); }
        if p.x + 1 < WIDTH && p.y + 1 < WIDTH     // right, diagonal down
        { adjacent[5] = p.right_down().index(); }
        if p.x > 0         && p.y > 0             // left, diagonal up
        { adjacent[6] = p.left_up   ().index(); }
        if p.x > 0         && p.y + 1 < HEIGHT    // left, diagonal down
        { adjacent[7] = p.left_down ().index(); }

        adjacent
    }
}


pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day11-input")
            .expect("Failed to read input file!");
    let mut energy_levels = [0; WIDTH * HEIGHT];
    string
        .lines()
        .enumerate()
        .for_each(
            |(y, line)|
                line
                    .bytes()
                    .enumerate()
                    .for_each(
                        |(x, b)|
                            energy_levels[y * WIDTH + x] = b - 48
                    )
        );

    // part 1
    let mut octopuses = Octo::new(energy_levels);
    for step in 0..100 {
        octopuses.step();
    }
    println!("Result of challenge 1: {}", octopuses.flashes);

    // part 2
    let mut octopuses = Octo::new(energy_levels);
    let mut step = 0;
    loop {
        step += 1;
        octopuses.step();
        if octopuses.was_full {
            break;
        }
    }
    println!("Result of challenge 2: {}", step);
}