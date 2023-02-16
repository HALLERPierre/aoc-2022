use std::env;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args
        .get(1)
        .expect("day required")
        .parse::<i32>()
        .expect("Expect day to be integer");

    match day {
        1 => {
            day1::puzzle1();
            day1::puzzle2();
        }
        2 => {
            day2::puzzle1();
            day2::puzzle2();
        }
        3 => {
            day3::puzzle1();
            day3::puzzle2();
        }
        4 => {
            day4::puzzle1();
            day4::puzzle2();
        }
        5 => {
            day5::puzzle1();
            day5::puzzle2();
        }
        6 => {
            day6::puzzle1();
            day6::puzzle2();
        }
        7 => {
            day7::puzzle1();
            day7::puzzle2();
        }
        8 => {
            day8::puzzle1();
            day8::puzzle2();
        }
        9 => {
            day9::puzzle1();
            day9::puzzle2();
        }
        10 => {
            day10::puzzle1();
            day10::puzzle2();
        }
        11 => {
            day11::puzzle1();
            day11::puzzle2();
        }
        12 => {
            day12::puzzle1();
            day12::puzzle2();
        }
        13 => {
            day13::puzzle1();
            day13::puzzle2();
        }
        14 => {
            day14::puzzle1();
            day14::puzzle2();
        }
        _ => {
            panic!("unkown day {}", day)
        }
    }
}
