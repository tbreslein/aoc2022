use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// Fixed and modified version of this:
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
#[derive(Debug)]
struct Node {
    pub name: String,
    pub size: usize,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(name: String, size: usize) -> Self {
        return Node {
            name,
            size,
            parent: None,
            children: vec![],
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
        self.children.push(new_node);
    }

    pub fn measure_size(&mut self) -> usize {
        for child in self.children.iter() {
            self.size += child.borrow_mut().measure_size();
        }
        return self.size;
    }
}

fn set_node_to_child(node: &mut Rc<RefCell<Node>>, name: String) {
    if let Some(child) = get_child_node(node, name) {
        *node = child;
    }
    return;
}

fn get_child_node(parent: &Rc<RefCell<Node>>, name: String) -> Option<Rc<RefCell<Node>>> {
    let children = &parent.borrow().children;
    for i in 0..children.len() {
        if children[i].borrow().name == name {
            return Some(Rc::clone(&children[i]));
        }
    }
    return None;
}

fn parse(data: &str) -> Rc<RefCell<Node>> {
    let mut lines = data.lines().peekable();
    assert_eq!(lines.next().unwrap(), "$ cd /");
    let root_node = Rc::new(RefCell::new(Node::new("/".to_string(), 0)));
    let mut current_node = Rc::clone(&root_node);

    'core: while let Some(line) = lines.next() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            match dir {
                ".." => {
                    let parent = Rc::clone(
                        &current_node
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap(),
                    );
                    current_node = parent;
                }
                "/" => current_node = root_node.clone(),
                _ => set_node_to_child(&mut current_node, dir.to_string()),
            };
            continue 'core;
        }

        if line == "$ ls" {
            'ls_output: while let Some(next_line) = lines.peek() {
                if next_line.starts_with("$") {
                    continue 'core;
                }
                if let Some(new_dir) = next_line.strip_prefix("dir ") {
                    current_node
                        .borrow_mut()
                        .add_child(Rc::new(RefCell::new(Node {
                            name: new_dir.to_string(),
                            size: 0,
                            parent: Some(Rc::downgrade(&current_node)),
                            children: vec![],
                        })));
                    lines.next();
                    continue 'ls_output;
                }
                if let Some((size, name)) = next_line.split_once(" ") {
                    current_node
                        .borrow_mut()
                        .add_child(Rc::new(RefCell::new(Node {
                            name: name.to_string(),
                            size: size.parse().unwrap(),
                            parent: Some(Rc::downgrade(&current_node)),
                            children: vec![],
                        })));
                }
                lines.next();
            }
        }
    }

    // Now that the dir structure is parsed, let's fix the recursive size values of each dir
    // NOTE: I could have probably done this during the exploration / when parsing the commands,
    // but since we cannot assume the traversel in the commands to be "well behaved" (no jumping
    // around, apart from "cd .." chains), I decided to split this into two parts.
    current_node = Rc::clone(&root_node);
    current_node.borrow_mut().measure_size();

    return root_node;
}

pub fn solve_p1(data: &str) -> usize {
    let fs = parse(data);
    return find_small_dir_sum(&fs, 0);
}

fn find_small_dir_sum(fs: &Rc<RefCell<Node>>, sum: usize) -> usize {
    return sum
        // Make sure to only count directories; files do not have children, so just check whether
        // the children field is empty
        + if !fs.borrow().children.is_empty() && fs.borrow().size < 100_000 {
            fs.borrow().size
        } else {
            0
        }
        + fs.borrow()
            .children
            .iter()
            .fold(0, |acc, child| acc + find_small_dir_sum(child, sum));
}

pub fn solve_p2(_data: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(solve_p1(&data), 95437);
    }

    #[test]
    fn p2_test() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_p2(&data), 19);
    }
}
