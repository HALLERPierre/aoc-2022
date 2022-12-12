const INPUT: &str = include_str!("./input");

use evalexpr::*;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation_raw: String,
    divider: i64,
    if_true: i64,
    if_false: i64,
}

pub fn puzzle1() {
    let mut monkeys = get_monkeys();
    let mut round = 1;
    let monkeys_size = monkeys.len();
    let mut monkeys_activity = monkeys.iter().map(|_| 0).collect::<Vec<i64>>();

    while round <= 20 {
        for index in 0..monkeys_size {
            let monkey = monkeys.clone()[index].clone();

            for item in monkey.items.iter() {
                monkeys_activity[index] += 1;

                let mut configuration = HashMapConfiguration::new();
                configuration.insert_variable("old", *item as i64);

                let mut new_worry =
                    eval_int_with_configuration(&monkey.operation_raw, &configuration)
                        .expect("should be evalued") as i64;

                new_worry = (new_worry as f32 / 3.0).floor() as i64;

                let target_monkey_index = if new_worry % monkey.divider == 0 {
                    monkey.if_true as usize
                } else {
                    monkey.if_false as usize
                };
                let target_monkey = monkeys[target_monkey_index].clone();

                let mut target_monkey_items = target_monkey.items.clone();

                target_monkey_items.push(new_worry);

                monkeys[target_monkey_index] = Monkey {
                    items: target_monkey_items,
                    operation_raw: target_monkey.operation_raw.clone(),
                    divider: target_monkey.divider,
                    if_true: target_monkey.if_true,
                    if_false: target_monkey.if_false,
                }
            }

            monkeys[index] = Monkey {
                items: vec![],
                operation_raw: monkey.operation_raw.clone(),
                divider: monkey.divider,
                if_true: monkey.if_true,
                if_false: monkey.if_false,
            }
        }
        round += 1;
    }

    monkeys_activity.sort();

    let monkey_business = monkeys_activity[monkeys_activity.len() - 2..]
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .expect("should have business");

    println!("1_ monkey business is {}", monkey_business);
}

pub fn puzzle2() {
    let mut monkeys = get_monkeys();
    let mut round = 1;
    let monkeys_size = monkeys.len();
    let mut monkeys_activity = monkeys.iter().map(|_| 0).collect::<Vec<i64>>();

    let largest_common_divider = monkeys
        .iter()
        .clone()
        .map(|monkey| monkey.divider)
        .reduce(|a, b| {
            return a * b;
        })
        .expect("should have largest divisor");

    while round <= 10000 {
        for index in 0..monkeys_size {
            let monkey = monkeys.clone()[index].clone();

            for item in monkey.items.iter() {
                monkeys_activity[index] += 1;

                let mut configuration = HashMapConfiguration::new();
                configuration.insert_variable("old", *item as i64);

                let mut new_worry =
                    eval_int_with_configuration(&monkey.operation_raw, &configuration)
                        .expect("should be evalued") as i64;

                new_worry = new_worry % largest_common_divider;

                let target_monkey_index = if new_worry % monkey.divider == 0 {
                    monkey.if_true as usize
                } else {
                    monkey.if_false as usize
                };

                let target_monkey = monkeys[target_monkey_index].clone();

                let mut target_monkey_items = target_monkey.items.clone();

                target_monkey_items.push(new_worry);

                monkeys[target_monkey_index] = Monkey {
                    items: target_monkey_items,
                    operation_raw: target_monkey.operation_raw.clone(),
                    divider: target_monkey.divider,
                    if_true: target_monkey.if_true,
                    if_false: target_monkey.if_false,
                }
            }

            monkeys[index] = Monkey {
                items: vec![],
                operation_raw: monkey.operation_raw.clone(),
                divider: monkey.divider,
                if_true: monkey.if_true,
                if_false: monkey.if_false,
            }
        }
        round += 1;
    }

    monkeys_activity.sort();

    let monkey_business = monkeys_activity[monkeys_activity.len() - 2..]
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .expect("should have business");

    println!("2_ monkey business is {}", monkey_business);
}

fn get_monkeys() -> Vec<Monkey> {
    return INPUT
        .split("\n\n")
        .map(|monkey_raw| {
            let lines = monkey_raw.split("\n").collect::<Vec<&str>>();
            let starting_item = lines[1]
                .replace("Starting items: ", "")
                .replace(" ", "")
                .split(",")
                .map(|item| item.parse::<i64>().expect("should be int"))
                .collect::<Vec<i64>>();
            let operation_raw = lines[2].replace("  Operation: new = ", "");
            let divisor = lines[3]
                .replace("  Test: divisible by ", "")
                .parse::<i64>()
                .expect("should be int");
            let if_true = lines[4]
                .replace("    If true: throw to monkey ", "")
                .parse::<i64>()
                .expect("should be int");
            let if_false = lines[5]
                .replace("    If false: throw to monkey ", "")
                .parse::<i64>()
                .expect("should be int");
            return Monkey {
                items: starting_item,
                operation_raw,
                divider: divisor,
                if_true,
                if_false,
            };
        })
        .collect::<Vec<Monkey>>();
}
