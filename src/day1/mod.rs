use std::cmp::Ordering;

pub fn puzzle1() {
    let calories_by_elves = get_calories_by_elves();
    let max_calories = calories_by_elves.iter().max().expect("Max value");

    println!("Max calories : {:?}", max_calories)
}

pub fn puzzle2() {
    let mut calories_by_elves = get_calories_by_elves();
    calories_by_elves.sort_by(|a, b| {
        let result = a - b;
        if result < 0 {
            return Ordering::Greater;
        }
        if result > 0 {
            return Ordering::Less;
        }
        return Ordering::Equal;
    });

    let top_3_calories = &calories_by_elves[..3];
    let top_3_calories_sum: i32 = top_3_calories.to_vec().iter().sum();
    println!("Top 3 calories sum : {:?}", top_3_calories_sum)
}

fn get_calories_by_elves() -> Vec<i32> {
    return include_str!("./input")
        .split("\n\n")
        .map(|calories| {
            let calories_by_elves = calories
                .split("\n")
                .filter(|calorie| calorie.len() > 0)
                .map(|calorie| calorie.parse::<i32>().expect("should be int"))
                .reduce(|a, b| a + b)
                .expect("should be int");
            return calories_by_elves;
        })
        .collect::<Vec<i32>>();
}
