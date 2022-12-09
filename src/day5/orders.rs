use crate::day5::stock_modification;
use crate::day5::types;

pub fn execute_orders_9000(stock: types::Stock, orders: types::Orders) -> types::Stock {
    let mut new_stock = stock;
    for order in orders.iter() {
        let (count, from, to) = order;
        for _ in 0..*count {
            let (updated_stock, element) =
                stock_modification::remove_from_stock_9000(new_stock.clone(), *from);

            new_stock = stock_modification::add_to_stock_9000(updated_stock, *to, element);
        }
    }

    return new_stock;
}

pub fn execute_orders_9001(stock: types::Stock, orders: types::Orders) -> types::Stock {
    let mut new_stock = stock;
    for order in orders.iter() {
        let (count, from, to) = order;

        let (updated_stock, elements) =
            stock_modification::remove_from_stock_9001(new_stock.clone(), *from, *count);

        new_stock = stock_modification::add_to_stock_9001(updated_stock, *to, elements);
    }

    return new_stock;
}

pub fn get_orders() -> types::Orders {
    include_str!("./input")
        .split("\n")
        .filter(|line| line.starts_with("move"))
        .map(get_orders_from_line)
        .collect()
}

fn get_orders_from_line(line: &str) -> (usize, usize, usize) {
    let orders: Vec<usize> = line
        .replace("move", "to")
        .replace("from", " ")
        .replace("to", " ")
        .split_ascii_whitespace()
        .map(|order| order.parse().expect("should be int"))
        .collect();
    return (
        *orders.get(0).expect("no count"),
        *orders.get(1).expect("no from") - 1,
        *orders.get(2).expect("no to") - 1,
    );
}
