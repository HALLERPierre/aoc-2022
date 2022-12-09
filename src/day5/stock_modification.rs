use crate::day5::types;

pub fn remove_from_stock_9001(
    stock: types::Stock,
    stack_index: usize,
    count: usize,
) -> (types::Stock, Vec<char>) {
    let mut new_stock: types::Stock = Vec::new();
    let mut elements: Option<Vec<char>> = None;

    for i in 0..stock.iter().len() {
        let mut new_stack = stock.get(i).expect("should have stack").clone();
        if i == stack_index {
            elements = Some(new_stack[new_stack.len() - count..].to_vec());
            new_stack.truncate(new_stack.len() - count);
        }

        new_stock.push(new_stack)
    }

    return (new_stock, elements.expect("No element found"));
}

pub fn add_to_stock_9001(
    stock: types::Stock,
    stack_index: usize,
    elements: Vec<char>,
) -> Vec<Vec<char>> {
    let mut new_stock: types::Stock = Vec::new();

    for i in 0..stock.iter().len() {
        let mut new_stack = stock.get(i).expect("should have stack").clone();
        if i == stack_index {
            new_stack.append(elements.clone().as_mut());
        }

        new_stock.push(new_stack)
    }

    return new_stock;
}

pub fn add_to_stock_9000(stock: types::Stock, stack_index: usize, element: char) -> Vec<Vec<char>> {
    let mut new_stock: types::Stock = Vec::new();

    for i in 0..stock.iter().len() {
        let mut new_stack = stock.get(i).expect("should have stack").clone();
        if i == stack_index {
            new_stack.push(element);
        }

        new_stock.push(new_stack)
    }

    return new_stock;
}

pub fn remove_from_stock_9000(stock: types::Stock, stack_index: usize) -> (types::Stock, char) {
    let mut new_stock: types::Stock = Vec::new();
    let mut element: Option<char> = None;

    for i in 0..stock.iter().len() {
        let mut new_stack = stock.get(i).expect("should have stack").clone();
        if i == stack_index {
            element = new_stack.pop();
        }

        new_stock.push(new_stack)
    }

    return (new_stock, element.expect("No element found"));
}
