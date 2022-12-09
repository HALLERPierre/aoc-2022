use crate::day5::stock_modification;
use crate::day5::types;

pub fn get_init_stock() -> types::Stock {
    let mut stock = get_empty_stock();

    let lines_with_stocks = get_lines_with_stocks();

    for line in lines_with_stocks.iter() {
        let line_chars = line.chars().collect::<Vec<char>>();

        for index_char in 0..line_chars.len() {
            let char = *line_chars.get(index_char).expect("no char");
            if char == ' ' || char == '[' || char == ']' {
                continue;
            }
            let stack_index = (index_char + 1) / 4;

            stock = stock_modification::add_to_stock_9000(stock, stack_index, char);
        }
    }

    return stock;
}

fn get_empty_stock() -> Vec<Vec<char>> {
    // get the stock count line
    let stock_count = include_str!("./input")
        .split("\n")
        .find(|line| line.len() > 0 && line.starts_with(" 1"))
        .expect("no stock line found")
        .split_ascii_whitespace()
        .map(|stock_number| {
            stock_number
                .parse::<i32>()
                .expect("can't parse stock number")
        })
        .max()
        .expect("no stock max found");

    let mut stocks: Vec<Vec<char>> = Vec::new();

    for _ in 0..stock_count {
        stocks.push(Vec::new());
    }
    return stocks;
}

fn get_lines_with_stocks() -> Vec<String> {
    let mut lines_with_stocks = include_str!("./input")
        .split("\n")
        .filter(|line| line.len() > 0 && line.contains('['))
        .map(str::to_string)
        .collect::<Vec<String>>();

    lines_with_stocks.reverse();
    return lines_with_stocks;
}
