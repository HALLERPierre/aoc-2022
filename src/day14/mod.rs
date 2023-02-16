use std::cmp;
const INPUT: &str = include_str!("./input");

pub fn puzzle1() {
    let rocks = parse_rocks();
    let rocks_coords = get_rocks_coords(rocks);
    let void_y = rocks_coords
        .iter()
        .map(|(_x, y)| y)
        .max()
        .expect("Should have a void");

    let mut is_in_void = false;
    let mut rocks_with_sand = rocks_coords.clone();
    let mut count_sand = 0;
    while !is_in_void {
        let (new_rocks, new_is_in_void) =
            spawn_sand(&rocks_with_sand, *void_y, (500 as i32, 0 as i32));
        rocks_with_sand = new_rocks.clone();
        is_in_void = new_is_in_void;
        if !is_in_void {
            count_sand += 1
        }
    }
    println!("There is {} units of sand to rest", count_sand)
}

pub fn puzzle2() {
    let rocks = parse_rocks();
    let rocks_coords = get_rocks_coords(rocks);
    let floor = rocks_coords
        .iter()
        .map(|(_x, y)| y)
        .max()
        .expect("Should have a void")
        + 2;

    let mut rocks_with_sand = rocks_coords.clone();
    let mut count_sand = 0;
    let mut is_spawn_blocked = false;
    while !is_spawn_blocked {
        let (new_rocks, new_is_spawn_blocked) =
            spawn_sand_with_floor(&rocks_with_sand, floor, (500 as i32, 0 as i32));
        is_spawn_blocked = new_is_spawn_blocked;
        count_sand += 1;
        rocks_with_sand = new_rocks.clone();
        println!("count_sand: {}", count_sand);
        let last_rock = new_rocks.clone().pop();
        println!("{:?}", last_rock);
    }
    println!("There is {} units of sand to rest", count_sand)
}

fn is_blocked(rocks: &Vec<(i32, i32)>, x: i32, y: i32, is_floor_reached: bool) -> bool {
    let result = rocks.iter().any(|(rock_x, rock_y)| {
        if is_floor_reached {
            return *rock_x == x;
        }
        return *rock_x == x && *rock_y == y;
    });
    return result;
}

fn spawn_sand_with_floor(
    rocks: &Vec<(i32, i32)>,
    floor_y: i32,
    spawn_coords: (i32, i32),
) -> (Vec<(i32, i32)>, bool) {
    let mut new_rocks = rocks.clone();
    let mut moved = true;
    let mut current_coords = spawn_coords;
    while moved {
        let (x, y) = current_coords;
        let down_y = y + 1;
        let is_floor_reached = down_y == floor_y;
        if is_floor_reached {
            break;
        }
        if !is_blocked(rocks, x, down_y, is_floor_reached) {
            current_coords = (x, down_y);
            continue;
        }
        if !is_blocked(rocks, x - 1, down_y, is_floor_reached) {
            current_coords = (x - 1, down_y);
            continue;
        }
        if !is_blocked(rocks, x + 1, down_y, is_floor_reached) {
            current_coords = (x + 1, down_y);
            continue;
        }
        moved = false;
    }
    new_rocks.push(current_coords);
    return (
        new_rocks,
        current_coords.0 == spawn_coords.0 && current_coords.1 == spawn_coords.1,
    );
}

fn spawn_sand(
    rocks: &Vec<(i32, i32)>,
    void_y: i32,
    spawn_coords: (i32, i32),
) -> (Vec<(i32, i32)>, bool) {
    let mut new_rocks = rocks.clone();
    let mut moved = true;
    let mut current_coords = spawn_coords;
    let mut is_in_void = false;
    while moved && !is_in_void {
        let (x, y) = current_coords;
        if y + 1 > void_y {
            is_in_void = true;
        }
        if !rocks.contains(&(x, y + 1)) {
            current_coords = (x, y + 1);
            continue;
        }
        if !rocks.contains(&(x - 1, y + 1)) {
            current_coords = (x - 1, y + 1);
            continue;
        }
        if !rocks.contains(&(x + 1, y + 1)) {
            current_coords = (x + 1, y + 1);
            continue;
        }
        moved = false;
    }
    new_rocks.push(current_coords);
    return (new_rocks, is_in_void);
}

fn get_rocks_coords(rocks: Vec<Vec<Vec<i32>>>) -> Vec<(i32, i32)> {
    let mut rocks_coords: Vec<(i32, i32)> = vec![];

    rocks.iter().for_each(|rock_path| {
        for rock in 1..rock_path.iter().len() {
            let prev_x = rock_path
                .get(rock - 1)
                .expect("rock should exists")
                .get(0)
                .expect("should have a x");
            let prev_y = rock_path
                .get(rock - 1)
                .expect("rock should exists")
                .get(1)
                .expect("should have a x");
            let x = rock_path
                .get(rock)
                .expect("rock should exists")
                .get(0)
                .expect("should have a x");
            let y = rock_path
                .get(rock)
                .expect("rock should exists")
                .get(1)
                .expect("should have a x");

            let min_x = cmp::min(*prev_x, *x);
            let max_x = cmp::max(*prev_x, *x);
            let min_y = cmp::min(*prev_y, *y);
            let max_y = cmp::max(*prev_y, *y);
            for cur_x in min_x..=max_x {
                rocks_coords.push((cur_x, *y))
            }
            for cur_y in min_y..=max_y {
                rocks_coords.push((*x, cur_y))
            }
        }
    });

    rocks_coords.sort_unstable();
    rocks_coords.dedup();
    return rocks_coords;
}

fn parse_rocks() -> Vec<Vec<Vec<i32>>> {
    INPUT
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.replace(' ', "")
                .split("->")
                .map(|coords| {
                    coords
                        .split(',')
                        .map(|coord| coord.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<Vec<Vec<i32>>>>()
}
