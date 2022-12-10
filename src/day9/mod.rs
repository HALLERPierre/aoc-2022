use std::collections::HashSet;

const INPUT: &str = include_str!("./input");

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

type Motion = (Direction, i32);

pub fn puzzle1() {
    let motions = get_motions();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut tail_positions: HashSet<Position> = HashSet::new();

    tail_positions.insert(tail);
    for motion in motions.iter() {
        let (direction, steps) = motion;

        for _ in 0..*steps {
            let (x_delta, y_delta) = get_delta_from_direction(*direction);
            head = Position {
                x: head.x + x_delta,
                y: head.y + y_delta,
            };
            if !is_adjacent(head, tail) {
                let (x_tail_delta, y_tail_delta) = get_delta_to_close_tail(head, tail);
                tail = Position {
                    x: tail.x + x_tail_delta,
                    y: tail.y + y_tail_delta,
                };
            }

            tail_positions.insert(tail);
        }
    }

    println!("1_ Tail has been in {} positions", tail_positions.len());
}

pub fn puzzle2() {
    let motions = get_motions();
    let mut head = Position { x: 0, y: 0 };
    let mut tails = (0..9)
        .map(|_| Position { x: 0, y: 0 })
        .collect::<Vec<Position>>();

    let last_tail_position = tails.len() - 1;

    let mut tail_positions: HashSet<Position> = HashSet::new();
    tail_positions.insert(tails[last_tail_position]);

    for motion in motions.iter() {
        let (direction, steps) = motion;

        for _ in 0..*steps {
            let (x_delta, y_delta) = get_delta_from_direction(*direction);
            head = Position {
                x: head.x + x_delta,
                y: head.y + y_delta,
            };
            let mut last_knot = head.clone();

            for (index, tail) in tails.clone().iter().enumerate() {
                if !is_adjacent(last_knot, tail.clone()) {
                    let (x_tail_delta, y_tail_delta) =
                        get_delta_to_close_tail(last_knot, tail.clone());

                    tails[index] = Position {
                        x: tail.x + x_tail_delta,
                        y: tail.y + y_tail_delta,
                    }
                }

                last_knot = tails[index].clone();
            }

            tail_positions.insert(tails[last_tail_position]);
        }
    }

    println!("1_ Tail has been in {} positions", tail_positions.len());
}

fn get_delta_to_close_tail(head: Position, tail: Position) -> (i32, i32) {
    let x = if head.x == tail.x {
        0
    } else if head.x > tail.x {
        1
    } else {
        -1
    };

    let y = if head.y == tail.y {
        0
    } else if head.y > tail.y {
        1
    } else {
        -1
    };

    return (x, y);
}

fn is_adjacent(pos1: Position, pos2: Position) -> bool {
    return i32::abs(pos1.x - pos2.x) <= 1 && i32::abs(pos1.y - pos2.y) <= 1;
}

fn get_delta_from_direction(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::UP => return (0, 1),
        Direction::DOWN => return (0, -1),
        Direction::RIGHT => {
            return (1, 0);
        }
        Direction::LEFT => {
            return (-1, 0);
        }
    }
}

fn get_motions() -> Vec<Motion> {
    INPUT
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let splitted = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let direction =
                get_direction_from_string(splitted.get(0).expect("should have direction"));
            let steps = splitted
                .get(1)
                .expect("should have steps")
                .parse::<i32>()
                .expect("should be integer");
            return (direction, steps);
        })
        .collect()
}

fn get_direction_from_string(direction: &str) -> Direction {
    match direction {
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        "R" => Direction::RIGHT,
        "L" => Direction::LEFT,
        _ => panic!("Unknown direction"),
    }
}
