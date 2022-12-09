mod init_stock;
mod orders;
mod stock_modification;
mod types;

pub fn puzzle1() {
    let init_stock = init_stock::get_init_stock();
    let orders = orders::get_orders();
    let final_stock = orders::execute_orders_9000(init_stock, orders);
    let last_crates = get_last_crates(final_stock);

    println!("Last crate on each stack {}", last_crates);
}

pub fn puzzle2() {
    let init_stock = init_stock::get_init_stock();
    let orders = orders::get_orders();
    let final_stock = orders::execute_orders_9001(init_stock, orders);
    let last_crates = get_last_crates(final_stock);

    println!("Last crate on each stack {}", last_crates);
}

fn get_last_crates(stock: types::Stock) -> String {
    let mut crates: Vec<char> = Vec::new();

    for stack in stock {
        crates.push(stack.clone().pop().expect("No crate on stack"));
    }

    return crates.iter().collect();
}
