// Why do these annoying warnings have to be on by default?
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

extern crate crossterm; // has been used in day 13.

mod  aoc_day1;
mod  aoc_day2;
mod  aoc_day3;
mod  aoc_day4;
mod  aoc_day5;
mod  aoc_day6;
mod  aoc_day7;
mod  aoc_day8;
mod  aoc_day9;
mod aoc_day10;
mod aoc_day11;
mod aoc_day12;
mod aoc_day13;
mod aoc_day14;
mod aoc_day15;
mod aoc_day16;
mod aoc_day17;
mod aoc_day18;
mod aoc_day19;
mod aoc_day20;
mod aoc_day21;
mod aoc_day22;
mod aoc_day23;
mod aoc_day24;
mod aoc_day25;

macro_rules! solve_days {
    ($($day: ident,)*) => {{
        $(let day = stringify!($day).replace("aoc_day", "");
        println!("\x1b[38;5;3mDay {}: \x1b[0m", day);
               // ^^^^^^^^^^^^        ^^^^^^^ reset color
               // yellow color
        let time = std::time::Instant::now();
        $day::solve();
        println!("\x1b[38;5;3mCompleted day {} in {:?}\n\x1b[0m", day, time.elapsed());)*
               // ^^^^^^^^^^^^ yellow color             ^^^^^^^ reset color
    }}
}

fn main() {
    solve_days!(
        aoc_day14,
        // aoc_day13,
        // aoc_day12,
        // aoc_day11,
        // aoc_day10,
        //  aoc_day9,
        //  aoc_day8,
        //  aoc_day7,
        //  aoc_day6,
        //  aoc_day5,
        //  aoc_day4,
        //  aoc_day3,
        //  aoc_day2,
        //  aoc_day1,
    );
}
