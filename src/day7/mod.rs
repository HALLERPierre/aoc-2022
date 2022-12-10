/**
 *
 *
 *
 *
 *
 *
 *
 *			RAGEQUIT ON RUST, DONE IN TS
 *
 *
 *
 *
 *
 *
 *
 *
 */

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

    let clone = Rc::clone(&root_directory);
    compute_size(clone);
    println!("FAILED");
}

pub fn puzzle2() {
    println!("FAILED");
}

fn compute_size(directory: Rc<RefCell<Directory>>) {
    let clone = Rc::clone(&directory);

    for sub_dir in clone.borrow().children.iter() {
        compute_size(Rc::clone(sub_dir));
    }

    let children = Rc::clone(&Rc::new(clone.borrow().children.to_vec()));
    let files = Rc::clone(&Rc::new(clone.borrow().files.to_vec()));

    let sub_dirs_size: i32 = children
        .iter()
        .map(|dir| dir.borrow().size.unwrap_or(0))
        .sum();
    let files_size: i32 = files.iter().sum();

    clone.borrow_mut().size = Some(sub_dirs_size + files_size);
}

fn init_root_directory() -> Rc<RefCell<Directory>> {
    let lines = INPUT.split("\n").filter(|line| line.len() > 0);

    let root = Rc::new(RefCell::new(Directory::new()));
    let mut current = Rc::clone(&root);
    for line in lines {
        {
            println!("{}", line);
        }
        if line == "$ cd /" || line.starts_with("dir") || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd") {
            // return to parent
            if line == "$ cd .." {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(
                    current_clone
                        .borrow()
                        .parent
                        .as_ref()
                        .expect("should have parent"),
                );
            }
            // Going into child dir
            else {
                let child = Rc::new(RefCell::new(Directory::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
                current = child;
            }
        }

        // it's a file
        if !line.starts_with("$") {
            let file_size = find_file_size(line);
            current.borrow_mut().files.push(file_size);
        }
    }

    return root;
}

fn find_file_size(line: &str) -> i32 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let captures = RE.captures(line).expect("should have number");

    return captures[1].parse::<i32>().expect("should be int");
}
