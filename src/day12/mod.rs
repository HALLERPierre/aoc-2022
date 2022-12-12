const INPUT: &str = include_str!("./input");

pub fn puzzle1() {
    let (maze, start, target) = get_maze();
    let (resolved_maze, _) = resolve_maze(maze, start, target);
    let distance = get_distance(resolved_maze);

    println!("1_ Distance to highest point from start is {}", distance);
}

pub fn puzzle2() {
    let (maze, _, target) = get_maze();

    let mut possible_starts: Vec<(i32, i32)> = vec![];

    for x in 0..maze.len() {
        for y in 0..maze[0].len() {
            let cell = maze[x][y];
            if cell == 0 {
                possible_starts.push((x as i32, y as i32));
            }
        }
    }

    let minumum_distance = possible_starts
        .iter()
        .map(|&start| {
            let (resolved_maze, target_reached) = resolve_maze(maze.clone(), start, target);
            if !target_reached {
                return i32::MAX;
            }
            return get_distance(resolved_maze);
        })
        .min();

    println!(
        "2_ Minimum distance is {}",
        minumum_distance.expect("should have min distance")
    );
}

fn get_distance(resolved_maze: Vec<Vec<Option<i32>>>) -> i32 {
    return resolved_maze
        .iter()
        .map(|line| line.iter().map(|distance| distance.unwrap_or(-1)).max())
        .max()
        .unwrap()
        .unwrap();
}

fn resolve_maze(
    maze: Vec<Vec<i32>>,
    start_index: (i32, i32),
    target_index: (i32, i32),
) -> (Vec<Vec<Option<i32>>>, bool) {
    let x_len = maze.len();
    let y_len = maze[0].len();
    let mut resolved_maze: Vec<Vec<Option<i32>>> = vec![vec![None; y_len]; x_len];

    resolved_maze[start_index.0 as usize][start_index.1 as usize] = Some(0);

    let mut target_reached = false;
    let mut path_found = true;
    let mut i = 0;

    while !target_reached && path_found {
        path_found = false;
        for x in 0..resolved_maze.len() {
            for y in 0..resolved_maze[0].len() {
                let cell = resolved_maze[x][y];
                if cell != None && cell.unwrap() == i {
                    let cell_heigh = maze[x][y];
                    let neighboors = [
                        (x as i32 - 1, y as i32),
                        (x as i32 + 1, y as i32),
                        (x as i32, y as i32 - 1),
                        (x as i32, y as i32 + 1),
                    ];
                    for neigh in neighboors {
                        let (x_neigh, y_neigh) = neigh;
                        if x_neigh < 0
                            || x_neigh >= x_len as i32
                            || y_neigh < 0
                            || y_neigh >= y_len as i32
                        {
                            continue;
                        }
                        let is_neigh_reachable =
                            maze[x_neigh as usize][y_neigh as usize] <= cell_heigh + 1;

                        if is_neigh_reachable
                            && resolved_maze[x_neigh as usize][y_neigh as usize] == None
                        {
                            path_found = true;
                            resolved_maze[x_neigh as usize][y_neigh as usize] = Some(i + 1);
                        }
                    }
                }
            }
        }

        i += 1;
        target_reached = resolved_maze[target_index.0 as usize][target_index.1 as usize] != None;
    }

    return (resolved_maze, target_reached);
}

fn get_maze() -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    let a_value = 'a' as i32;
    let mut start_index: (i32, i32) = (-1, -1);
    let mut target_index: (i32, i32) = (-1, -1);

    let maze = INPUT
        .split("\n")
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(x, line)| {
            let cells = line.chars();
            return cells
                .enumerate()
                .map(|(y, c)| {
                    if c == 'S' {
                        start_index = (x as i32, y as i32);
                        return 0;
                    }
                    if c == 'E' {
                        target_index = (x as i32, y as i32);
                        return ('z' as i32) - a_value;
                    }
                    (c as i32) - a_value
                })
                .collect::<Vec<i32>>();
        })
        .collect::<Vec<Vec<i32>>>();

    return (maze, start_index, target_index);
}
