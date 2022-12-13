use std::{
    cell::RefCell,
    cmp::min,
    rc::{Rc, Weak},
};

// Fixed and modified version of this:
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
#[derive(Debug)]
struct Node {
    pub name: String,
    pub size: usize,
    // Both the parent and the children are reference counted to make sure that they are only
    // dropped when actually no body is referencing them anymore.
    // The parent needs a weak reference to the parent so that dropping this node does not also
    // attempt to drop the parent (which would result in an endless loop, because then the parent
    // would try to drop the child it has a strong reference to, but then that child will, again,
    // try to drop the parent, and so on...)
    // Also, in both cases the nodes are wrapped in RefCells to enable inner mutability, since in
    // my design for this solution I need to be able to change the size field after the fact. I
    // could have done without it, if I measured directory size directly.
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

fn set_node_to_parent(node: &mut Rc<RefCell<Node>>) {
    let parent = Rc::clone(&node.borrow().parent.as_ref().unwrap().upgrade().unwrap());
    *node = parent;
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

    // loop annotation needed, because the ls block needs to be able to continue this loop from
    // within a nested while loop
    'core: while let Some(line) = lines.next() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            match dir {
                ".." => set_node_to_parent(&mut current_node),
                "/" => current_node = root_node.clone(),
                _ => set_node_to_child(&mut current_node, dir.to_string()),
            };
            continue 'core;
        }

        if line == "$ ls" {
            // another while loop that keeps peeking at the next line until that next line is
            // another command instead of ls output. When that point is reached, go back to the
            // core loop and iterate that
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
    // recursively iterate through each node that is not just a file and whose size is smaller
    // than 100_000, and sum up the sizes of those nodes
    fn f(fs: &Rc<RefCell<Node>>, sum: usize) -> usize {
        return fs.borrow()
            .children
            .iter()
            .fold(0, |acc, child| acc + f(child, sum))
        // Make sure to only count directories; files do not have children, so just check whether
        // the children field is empty
        + if !fs.borrow().children.is_empty() && fs.borrow().size < 100_000 {
            fs.borrow().size
        } else {
            0
        } + sum;
    }

    let fs = parse(data);
    return f(&fs, 0);
}

pub fn solve_p2(data: &str) -> usize {
    // recursively iterate through each node that is not just a file and whose size is larger
    // than the needed_space, and find the smallest among those
    fn f(fs: &Rc<RefCell<Node>>, min_size: usize, needed_space: usize) -> usize {
        return min(
            fs.borrow().children.iter().fold(min_size, |acc, child| {
                min(acc, f(child, min_size, needed_space))
            }),
            if !fs.borrow().children.is_empty() && fs.borrow().size > needed_space {
                fs.borrow().size
            } else {
                usize::MAX
            },
        );
    }

    let fs = parse(data);
    let needed_space = 30_000_000 - (70_000_000 - fs.borrow().size);
    return f(&fs, usize::MAX, needed_space);
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
        assert_eq!(solve_p2(&data), 24933642);
    }
}
