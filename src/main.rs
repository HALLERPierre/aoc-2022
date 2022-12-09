use std::env;

mod day1;

mod day2;

mod day3;

mod day4;

mod day5;

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
        _ => {
            panic!("unkown day {}", day)
        }
    }
}
