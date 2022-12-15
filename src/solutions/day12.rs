use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    elevation: u8,
    is_end: bool,
    cost: i32,
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Edge {
    node: Pos,
    cost: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Edge {}

fn parse(data: &str) -> Vec<Vec<Node>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Node {
                        elevation: 'a' as u8,
                        is_end: false,
                        cost: 0,
                    },
                    'E' => Node {
                        elevation: 'z' as u8 + 1,
                        is_end: true,
                        cost: i32::MAX,
                    },
                    'a'..='z' => Node {
                        elevation: c as u8,
                        is_end: false,
                        cost: i32::MAX,
                    },
                    _ => panic!("wtf"),
                })
                .collect()
        })
        .collect()
}

fn update_neighbors((j, i): &(usize, usize), map: &Vec<Vec<Node>>, neighbors: &mut Vec<Pos>) {
    neighbors.clear();
    if *j > 0 && map[*j - 1][*i].elevation - 1 <= map[*j][*i].elevation {
        neighbors.push((*j - 1, *i));
    }
    if *i > 0 && map[*j][*i - 1].elevation - 1 <= map[*j][*i].elevation {
        neighbors.push((*j, *i - 1));
    }
    if *j < map.len() - 1 && map[*j + 1][*i].elevation - 1 <= map[*j][*i].elevation {
        neighbors.push((*j + 1, *i));
    }
    if *i < map[0].len() - 1 && map[*j][*i + 1].elevation - 1 <= map[*j][*i].elevation {
        neighbors.push((*j, *i + 1));
    }
}

pub fn solve_p1(data: &str) -> i32 {
    let mut map = parse(data);
    // find starting point
    let (mut j, mut i) = (0, 0);
    for jstart in 0..map.len() {
        for istart in 0..map[0].len() {
            if map[jstart][istart].cost == 0 {
                (j, i) = (jstart, istart);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Edge {
        node: (j, i),
        cost: 0,
    });

    let mut neighbors = Vec::with_capacity(3); // any node never has more than 3 unvisited neighbors
    while let Some(Edge { node, cost }) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }

        update_neighbors(&node, &map, &mut neighbors);
        for neighbor in neighbors.iter() {
            let new_cost = cost + 1;
            if map[neighbor.0][neighbor.1].is_end {
                return new_cost;
            }

            if new_cost < map[neighbor.0][neighbor.1].cost {
                map[neighbor.0][neighbor.1].cost = new_cost;
                queue.push(Edge {
                    node: neighbor.clone(),
                    cost: new_cost,
                });
            }
        }
    }

    return 0;
}

pub fn solve_p2(data: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(solve_p1(&data), 31);
    }

    #[test]
    fn p2_test() {
        let data = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(solve_p2(&data), 0);
    }
}
