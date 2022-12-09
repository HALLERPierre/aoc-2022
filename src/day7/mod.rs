const INPUT: &str = include_str!("./input");

use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
struct Directory {
    files: Vec<i32>,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Directory>>>,
    size: Option<i32>,
}

impl Directory {
    pub fn new() -> Directory {
        return Directory {
            files: vec![],
            parent: None,
            children: vec![],
            size: None,
        };
    }
}

pub fn puzzle1() {
    let root_directory = init_root_directory();
    compute_size(Rc::clone(&root_directory));
    {
        println!("{}", root_directory.borrow().size.unwrap());
    }
}

fn compute_size(directory: Rc<RefCell<Directory>>) {
    for sub_dir in directory.borrow().children.iter() {
        compute_size(Rc::clone(sub_dir));
    }
    directory.borrow_mut().size = directory
        .borrow()
        .children
        .iter()
        .map(|dir| dir.borrow().size)
        .sum();
}

fn init_root_directory() -> Rc<RefCell<Directory>> {
    let lines = INPUT.split("\n").filter(|line| line.len() > 0);

    let root = Rc::new(RefCell::new(Directory::new()));
    let mut current = Rc::clone(&root);
    let mut indent = 0;
    for (index, line) in lines.enumerate() {
        {
            println!("{}", line);
        }
        if index == 0 {
            continue;
        }
        let current_indent = line.find("-").expect("should have '-'");

        // return to parent
        if current_indent < indent {
            let current_clone = Rc::clone(&current);
            current = Rc::clone(
                current_clone
                    .borrow()
                    .parent
                    .as_ref()
                    .expect("should have parent"),
            );
        }

        // is child directory
        if line.contains("dir") {
            let child = Rc::new(RefCell::new(Directory::new()));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
            }
            current = child;
        }

        // it's a file
        if line.contains("file") {
            let file_size = find_file_size(line);
            current.borrow_mut().files.push(file_size);
        }

        indent = current_indent;
    }

    return root;
}

fn find_file_size(line: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"size=(\d+)").unwrap();
    }

    let captures = RE.captures(line).expect("should have number");

    return captures[1].parse::<i32>().expect("should be int");
}

pub fn puzzle2() {
    println!("WIP");
}
