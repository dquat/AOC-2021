use std::fs;
const SAMPLE_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;
const DIAGONAL_LR: [usize; 5] = [0, 6, 12, 18, 24];
const DIAGONAL_RL: [usize; 5] = [4, 8, 12, 16, 20];

struct Board {
    data   : [[u32 ; 25]; 100],
    marked : [[bool; 25]; 100],
    draws  : [u32; 100],
    won    : [bool; 100],
    done   : usize,
}

impl Board {
    fn new() -> Board {
        Board {
            data   : [[0    ; 25]; 100],
            marked : [[false; 25]; 100],
            draws  : [0; 100],
            won    : [false; 100],
            done   : 0,
        }
    }

    #[inline]
    fn set_data(&mut self, board: usize, board_item: usize, data: u32) {
        self.data[board][board_item] = data;
    }

    #[inline]
    fn get(&self, board: usize, x: usize, y: usize) -> (u32, bool) {
        (self.data[board][y * 5 + x], self.marked[board][y * 5 + x])
    }

    #[inline]
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
        return win;
    }

    fn generate_board(string: String) -> Board {
        let mut lines = string.lines();
        let mut boards = Board::new();
        let mut draws = [0; 100];
        lines
            .next()
            .unwrap()
            .split(",")
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
        while let Some(line) = lines.next() {
            if line.is_empty() {
                index = 0;
                board_index += 1;
                continue;
            }
            let mut numbers = line.split_ascii_whitespace();
            while let Some(x) = numbers.next() {
                let x = str::parse::<u32>(x).unwrap();
                boards.set_data(board_index, index, x);
                index += 1;
            }
        }
        boards
    }

    fn next_win(&mut self) -> Option<(usize, u32)> {
        if self.done == 100 { return None; }
        let mut winning = None;
        let mut last_call = 0;
        for i in 0..100 {
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
                    last_call = self.draws[i];
                    winning = Some((j, last_call));
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

pub fn aoc_day4() {
    let string =
        fs::read_to_string("src/aoc-day4-input")
            .expect("Failed to read input file!");

    let mut boards = Board::generate_board(string);
    let (win, last_draw) = boards.next_win().unwrap();
    println!("Result of challenge 1: {}", boards.unmarked_value(win) * last_draw);

    let (mut board, mut last_board_call) = (0, 0);
    while let Some((next_board, last_called)) = boards.next_win() {
        board = next_board;
        last_board_call = last_called;
    }
    println!("Result of challenge 2: {}", boards.unmarked_value(board) * last_board_call);
}