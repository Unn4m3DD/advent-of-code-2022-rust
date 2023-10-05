use std::vec;

#[derive(Debug)]
struct File {}

#[derive(Debug)]
enum Tree {
    Leaf(File),
    Node(String, Vec<Tree>, u64),
}

impl Tree {
    fn new(input: &str) -> Tree {
        let mut tree = Tree::Node("".to_string(), vec![], 0);
        let mut current_dir = vec![];
        input.split("\n").for_each(|line| {
            if line.starts_with("$ cd") {
                let path = line.split("$ cd").collect::<Vec<&str>>()[1].trim();
                if path == ".." {
                    current_dir.pop();
                } else if path == "/" {
                    current_dir = vec![];
                } else {
                    current_dir.push(path);
                }
            } else if !line.starts_with("$") {
                if !line.starts_with("dir") {
                    let parts = line.split(" ").collect::<Vec<&str>>();
                    let size = parts[0].trim().parse::<u64>().unwrap();
                    let name = parts[1].trim();
                    tree.add(&current_dir, name.to_string(), size);
                }
            }
        });
        tree
    }
    fn add(&mut self, dir: &[&str], name: String, size: u64) {
        let current_path = dir.get(0);
        if let Tree::Node(_, _, node_size) = self {
            *node_size += size;
        }
        match current_path {
            None => {
                if let Tree::Node(_, children, _) = self {
                    children.push(Tree::Leaf(File {}));
                };
            }
            Some(path) => {
                if let Tree::Node(_, children, _) = self {
                    let sub_tree = children.into_iter().find(|e| {
                        if let Tree::Node(name, _, _) = e {
                            path == &name
                        } else {
                            false
                        }
                    });
                    if let Some(sub_tree) = sub_tree {
                        sub_tree.add(&dir[1..], name, size);
                    } else {
                        let mut new_tree = Tree::Node(path.to_string(), vec![], 0);
                        new_tree.add(&dir[1..], name, size);
                        children.push(new_tree);
                    }
                };
            }
        }
    }

    fn traverse(&self, listener: &mut dyn FnMut((&String, &Vec<Tree>, &u64))) {
        match self {
            Tree::Leaf(_) => {}
            Tree::Node(name, children, size) => {
                listener((name, children, size));
                children.iter().for_each(|e| e.traverse(listener));
            }
        }
    }
}

fn run_a(input: &str) {
    let tree = Tree::new(input);
    let mut big_dir_size_sum = 0;
    tree.traverse(&mut |(_, _, size)| {
        if *size < 100000 {
            big_dir_size_sum += size;
        }
    });
    println!("a: {}", big_dir_size_sum);
}

fn run_b(input: &str) {
    let tree = Tree::new(input);
    let mut total_size = 0;
    if let Tree::Node(_, _, size) = tree {
        total_size = size;
    }
    let needed_space = total_size - (70000000 - 30000000);
    let mut dir_min_size: u64 = u64::MAX;
    tree.traverse(&mut |(_, _, size)| {
        if *size > needed_space && dir_min_size > *size {
            dir_min_size = *size;
        }
    });
    println!("b: {}", dir_min_size);
}

pub fn run() {
    let input = include_str!("../inputs/day-07.txt");
    run_a(input);
    run_b(input);
}
