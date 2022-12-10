const INPUT: &str = include_str!("./input");

#[derive(Debug)]

struct Tree {
    visible: bool,
}

pub fn puzzle1() {
    let forest = parse_input();
    let trees = get_trees_from_forest(forest);
    let visible_trees = trees
        .iter()
        .filter(|tree| tree.visible)
        .collect::<Vec<&Tree>>();
    println!("1_ {} visible trees from the outside", visible_trees.len());
}

pub fn puzzle2() {
    let forest = parse_input();
    let scenics = get_scenic_from_forest(forest);
    let hightest_scenic = scenics.iter().max().expect("should have a max");
    println!("2_ The highest scenic score is {}", hightest_scenic);
}

fn parse_input() -> Vec<Vec<i32>> {
    INPUT
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            return line
                .chars()
                .map(|element| element.to_digit(10).expect("should be int") as i32)
                .collect::<Vec<i32>>();
        })
        .collect::<Vec<Vec<i32>>>()
}

fn get_scenic_from_forest(forest: Vec<Vec<i32>>) -> Vec<i32> {
    let mut scenics: Vec<i32> = vec![];
    let x_length = forest.len() - 1;
    for (x, tree_line) in forest.iter().enumerate() {
        let y_length = tree_line.len() - 1;
        for (y, tree) in tree_line.iter().enumerate() {
            if x == 0 || y == 0 || x == x_length || y == y_length {
                scenics.push(0);
                continue;
            }
            let mut scenic_from_left = 0;
            for x in (0..x).rev() {
                let los_tree = forest
                    .get(x)
                    .expect("shoul have x")
                    .get(y)
                    .expect("should have y");
                if *los_tree >= *tree {
                    scenic_from_left += 1;
                    break;
                }
                scenic_from_left += 1;
            }

            let mut scenic_from_right = 0;
            for x in x + 1..=x_length {
                let los_tree = forest
                    .get(x)
                    .expect("shoul have x")
                    .get(y)
                    .expect("should have y");
                if *los_tree >= *tree {
                    scenic_from_right += 1;
                    break;
                }
                scenic_from_right += 1;
            }

            let mut scenic_from_top = 0;
            for y in (0..y).rev() {
                let los_tree = forest
                    .get(x)
                    .expect("shoul have x")
                    .get(y)
                    .expect("should have y");
                if *los_tree >= *tree {
                    scenic_from_top += 1;
                    break;
                }
                scenic_from_top += 1;
            }

            let mut scenic_from_bottom = 0;
            for y in y + 1..=y_length {
                let los_tree = forest
                    .get(x)
                    .expect("shoul have x")
                    .get(y)
                    .expect("should have y");
                if *los_tree >= *tree {
                    scenic_from_bottom += 1;
                    break;
                }
                scenic_from_bottom += 1;
            }
            scenics
                .push(scenic_from_left * scenic_from_right * scenic_from_top * scenic_from_bottom)
        }
    }
    scenics
}

fn get_trees_from_forest(forest: Vec<Vec<i32>>) -> Vec<Tree> {
    let mut trees: Vec<Tree> = vec![];
    let x_length = forest.len() - 1;
    for (x, tree_line) in forest.iter().enumerate() {
        let y_length = tree_line.len() - 1;
        for (y, tree) in tree_line.iter().enumerate() {
            if x == 0 || y == 0 || x == x_length || y == y_length {
                trees.push(Tree { visible: true });
                continue;
            }

            let is_hidden_from_left = (0..x).find(|x| {
                let los_tree = forest
                    .get(*x)
                    .expect("shoul have x")
                    .get(y)
                    .expect("should have y");
                return *los_tree >= *tree;
            }) != None;
            let is_hidden_from_right = (x + 1..=x_length).find(|x| {
                let los_tree = forest
                    .get(*x)
                    .expect("shoul have x")
                    .get(y)
                    .expect("should have y");
                return *los_tree >= *tree;
            }) != None;
            let is_hidden_from_top = (0..y).find(|y| {
                let los_tree = forest
                    .get(x)
                    .expect("shoul have x")
                    .get(*y)
                    .expect("should have y");
                return *los_tree >= *tree;
            }) != None;
            let is_hidden_from_bottom = (y + 1..=y_length).find(|y| {
                let los_tree = forest
                    .get(x)
                    .expect("shoul have x")
                    .get(*y)
                    .expect("should have y");
                return *los_tree >= *tree;
            }) != None;

            let is_hidden = is_hidden_from_left
                && is_hidden_from_right
                && is_hidden_from_top
                && is_hidden_from_bottom;

            trees.push(Tree {
                visible: !is_hidden,
            })
        }
    }
    trees
}
