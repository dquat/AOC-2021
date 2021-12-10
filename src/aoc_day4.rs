use std::fs;

struct Board {
    data   : [[u32 ; 25]; 100],
    marked : [[bool; 25]; 100],
    draws  : [u32; 100],
    won    : [bool; 100],
    done   : usize,
    last   : usize,
}

impl Board {
    fn new() -> Board {
        Board {
            data   : [[0    ; 25]; 100],
            marked : [[false; 25]; 100],
            draws  : [0; 100],
            won    : [false; 100],
            done   : 0,
            last   : 0,
        }
    }

    #[inline]
    fn set_data(&mut self, board: usize, board_item: usize, data: u32) {
        self.data[board][board_item] = data;
    }

    fn check_win_array(&self, board: usize, array: [usize; 5]) -> bool {
        let mut win = false;
        let mut win_locations = [false; 5];
        array.iter().enumerate().for_each(|(i, &x)| {
            if self.marked[board][x] {
                win_locations[i] = true;
            }
            if i == 4 {
                win = win_locations.iter().all(|&x| x);
            }
        });
        win
    }

    #[inline]
    fn check_win(&self, board: usize) -> bool {
        let mut win = false;
        for i in 0..2 {
            for p in 0..5usize {
                let arrays = [
                    // vertical
                    [p, p + 5, p + 10, p + 15, p + 20],
                    // horizontal
                    [p * 5, p * 5 + 1, p * 5 + 2, p * 5 + 3, p * 5 + 4],
                ];
                win =
                    self.check_win_array(board, arrays[i]);
                if win { break; }
            }
            if win { break; }
        }
        win
    }

    fn generate_board(string: String) -> Board {
        let mut lines = string.lines();
        let mut boards = Board::new();
        let mut draws = [0; 100];
        lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .for_each(
                |(i, x)|
                    draws[i] =
                        str::parse::<u32>(x)
                            .unwrap()
            );
        boards.draws = draws;
        lines.next();
        let (mut index, mut board_index) = (0, 0);
        for line in lines {
            if line.is_empty() {
                index = 0;
                board_index += 1;
                continue;
            }
            let mut numbers = line.split_ascii_whitespace();
            for x in numbers {
                let x = str::parse::<u32>(x).unwrap();
                boards.set_data(board_index, index, x);
                index += 1;
            }
        }
        boards
    }

    #[inline]
    fn next_win(&mut self) -> Option<(usize, u32)> {
        if self.done == 100 { return None; }
        let mut winning = None;
        for i in self.last..100 {
            if winning.is_some() { break; }
            let drawn = &self.draws[..(i + 1)];
            for j in 0..100 {
                if self.won[j] { continue; }
                drawn
                    .iter()
                    .for_each(|&x| {
                        if let Some(mark) =
                        self.data[j]
                            .iter()
                            .position(|&y| y == x) {
                            self.marked[j][mark] = true;
                        }
                    });
                if self.check_win(j) {
                    self.won[j] = true;
                    self.done += 1;
                    self.last = i;
                    winning = Some((j,  self.draws[i]));
                    break;
                };
            }
        }
        winning
    }

    fn unmarked_value(&self, board: usize) -> u32 {
        let mut unmarked = 0;
        for (i, &marked) in self.marked[board].iter().enumerate() {
            if !marked {
                unmarked += self.data[board][i];
            }
        }
        unmarked
    }
}

pub fn solve() {
    let string =
        fs::read_to_string("src/inputs/aoc-day4-input")
            .expect("Failed to read input file!");
    let time = std::time::Instant::now();
    let mut boards = Board::generate_board(string);
    let (win, last_draw) = boards.next_win().unwrap();
    println!("Result of challenge 1: {}", boards.unmarked_value(win) * last_draw);

    let (mut board, mut last_board_call) = (0, 0);
    while let Some((next_board, last_called)) = boards.next_win() {
        board = next_board;
        last_board_call = last_called;
    }
    println!("Result of challenge 2: {}", boards.unmarked_value(board) * last_board_call);
    println!("Time: {:?}", time.elapsed());
}