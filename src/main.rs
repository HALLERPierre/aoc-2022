use std::env;

#[path = "day_1/day_1.rs"]
mod day1;

#[path = "day_2/day_2.rs"]
mod day2;

#[path = "day_3/day_3.rs"]
mod day3;

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
        _ => {
            panic!("unkown day {}", day)
        }
    }
}
