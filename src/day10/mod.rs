const INPUT: &str = include_str!("./input");
use lazy_static::lazy_static;
use regex::Regex;

pub fn puzzle1() {
    let target_cycles: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    let mut instructions = get_instructions();
    let mut current_cycle = 0;
    let mut x_value = 1;
    let mut next_instruction: Option<(i32, i32)> = None;

    let mut signal_strength = 0;

    while instructions.len() > 0 || next_instruction != None {
        current_cycle += 1;

        if target_cycles.iter().find(|cycle| current_cycle == **cycle) != None {
            signal_strength += current_cycle * x_value;
        }

        if next_instruction == None {
            let next_value = instructions[0];

            instructions = instructions[1..].to_vec();

            if next_value == 0 {
                next_instruction = Some((current_cycle, next_value));
            } else {
                next_instruction = Some((current_cycle + 1, next_value));
            }
        }

        let (target_cycle, value) = next_instruction.expect("No more instruction");
        if target_cycle > current_cycle {
            continue;
        }
        x_value += value;
        next_instruction = None;
    }
    println!("1_ signal strength is: {}", signal_strength);
}

pub fn puzzle2() {
    let mut instructions = get_instructions();
    let mut current_cycle = 0;
    let mut x_value = 1;
    let mut next_instruction: Option<(i32, i32)> = None;

    println!("2_ Here is the 8 letters :");

    while instructions.len() > 0 || next_instruction != None {
        current_cycle += 1;

        let pixel_position = (current_cycle - 1) % 40;

        if pixel_position == 0 {
            print!("\n");
        }

        if pixel_position - 1 <= x_value && pixel_position + 1 >= x_value {
            print!("#");
        } else {
            print!(".");
        }

        if next_instruction == None {
            let next_value = instructions[0];

            instructions = instructions[1..].to_vec();

            if next_value == 0 {
                next_instruction = Some((current_cycle, next_value));
            } else {
                next_instruction = Some((current_cycle + 1, next_value));
            }
        }

        let (target_cycle, value) = next_instruction.expect("No more instruction");
        if target_cycle > current_cycle {
            continue;
        }
        x_value += value;
        next_instruction = None;
    }
    println!("");
}

pub fn get_instructions() -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?\d+)").unwrap();
    }

    return INPUT
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|instruction| {
            if instruction == "noop" {
                return 0 as i32;
            } else {
                let captures = RE.captures(instruction).expect("should have number");
                let value = captures[1].parse::<i32>().expect("should be int");
                return value;
            }
        })
        .collect::<Vec<i32>>();
}
