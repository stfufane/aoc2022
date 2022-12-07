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
        self.children
            .iter()
            .find(|elem| elem.as_ref().borrow().value == value)
    }

    pub fn children_size(&self) -> u32 {
        self.children
            .iter()
            .map(|elem| elem.as_ref().borrow().size + elem.as_ref().borrow().children_size())
            .sum()
    }

    pub fn get_directories_sizes(&self, directories: &mut Vec<u32>) {
        self.children.iter().for_each(|elem| {
            if elem.as_ref().borrow().children.is_empty() {
                return; // Ignore leaves.
            } 
            elem.as_ref().borrow().get_directories_sizes(directories); // Call method recursively.
            let dir_size = elem.as_ref().borrow().size + elem.as_ref().borrow().children_size();
            // println!("{} = {}", elem.as_ref().borrow().value, dir_size);
            directories.push(dir_size);
        });
    }
}

const TOTAL_SPACE: u32 = 70000000;
const REQUIRED_SPACE: u32 = 30000000;

fn main() {
    let mut used_space: u32 = 0;
    let mut directory_spaces: Vec<u32> = Vec::new();

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
                if cmd[2] == ".." { // Go back to the parent.
                    current = Rc::clone(current_clone.as_ref().borrow_mut().parent.as_ref().unwrap());
                } else { // Go to a child directory
                    current = Rc::clone(
                        current_clone
                            .as_ref()
                            .borrow_mut()
                            .find_child(cmd[2])
                            .as_ref()
                            .unwrap(),
                    );
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
                    used_space += mut_child.size;
                }
            }
        }
    }

    root.as_ref()
        .borrow()
        .get_directories_sizes(&mut directory_spaces); // Gather the size of all the directories recursively.
    directory_spaces.sort(); // To get the first eligible result for part 2.

    println!(
        "Part 1 : Sum of directories < 100.000 is {}",
        directory_spaces
            .iter()
            .filter(|&&d| d < 100000)
            .sum::<u32>()
    );
    println!(
        "Part 2 : Space to free : {}",
        directory_spaces
            .iter()
            .find(|&value| { TOTAL_SPACE - used_space + value >= REQUIRED_SPACE })
            .unwrap()
    );
}
