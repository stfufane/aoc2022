use std::cell::RefCell;
use std::rc::Rc;

use itertools::Itertools;

// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
#[derive(PartialEq)]
struct TreeNode {
    pub value: String,
    pub size: u32,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(value: &str, size: u32) -> TreeNode {
        TreeNode {
            value: String::from(value),
            size,
            children: vec![],
            parent: None,
        }
    }

    pub fn find_child(&self, value: &str) -> Option<&Rc<RefCell<TreeNode>>> {
        self.children.iter().find(|elem| {
            elem.as_ref().borrow().value == value
        })
    }

    pub fn children_size(&self) -> u32 {
        self.children.iter().map(|elem| elem.as_ref().borrow().size).sum()
    }
}

fn main() {
    let mut part1_result: u32 = 0;
    const INPUT: &str = include_str!("../input.txt");
    let root = Rc::new(RefCell::new(TreeNode::new("/", 0)));
    let mut current = Rc::clone(&root);
    for line in INPUT.lines() {
        if line.starts_with('$') {
            let cmd = line.split_whitespace().collect_vec();
            if cmd[1] == "ls" {
                continue; // Nothing to do here, read the next lines
            } 
            if cmd[1] == "cd" {
                // Change the current node
                let current_clone = Rc::clone(&current);
                if cmd[2] == ".." { 
                    // Get the size of all the children this parent has.
                    let current_size = current.as_ref().borrow_mut().children_size();
                    current.as_ref().borrow_mut().size = current_size;
                    // Part 1 : Get the directories that use less than 100000 storage space.
                    if current.as_ref().borrow().size < 100000 { part1_result += current.as_ref().borrow().size; }
                    // Go back to the parent.
                    current = Rc::clone(current_clone.as_ref().borrow_mut().parent.as_ref().unwrap());
                } else { 
                    // Go to a child directory
                    current = Rc::clone(current_clone.as_ref().borrow_mut().find_child(cmd[2]).as_ref().unwrap());  
                }
            }
        } else {
            let (size, name) = line
                .split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap();
            // Add current line content to the tree.
            let child = Rc::new(RefCell::new(TreeNode::new(name, 0)));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
                if size.parse::<u32>().is_ok() {
                    mut_child.size = size.parse().unwrap();
                }
            }
        }
    }
    println!("Part 1 : sum of folders < 100.000 is {}", part1_result);
}
