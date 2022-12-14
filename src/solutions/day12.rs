use std::collections::HashMap;
use std::iter::zip;

#[derive(Debug)]
struct Node {
    elevation: i32,
    cost: i32,
    is_end: bool,
}

fn parse(data: &str) -> Vec<Vec<Node>> {
    let mut lettermap = HashMap::new();
    for (k, v) in zip('a'..='z', 1..) {
        lettermap.insert(k, v);
    }
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Node {
                        elevation: 0,
                        cost: 0,
                        is_end: false,
                    },
                    'E' => Node {
                        elevation: i32::MAX,
                        cost: i32::MAX,
                        is_end: true,
                    },
                    letter @ 'a'..='z' => Node {
                        elevation: lettermap.get(&letter).unwrap().clone(),
                        cost: i32::MAX,
                        is_end: false,
                    },
                    _ => panic!("wtf"),
                })
                .collect()
        })
        .collect()
}

pub fn solve_p1(data: &str) -> i32 {
    let mut heightmap = parse(data);
    let mut j = 0;
    let mut i = 0;
    // find starting point
    'outer: for (j_start, col) in heightmap.iter().enumerate() {
        for (i_start, x) in col.iter().enumerate() {
            if let Node { elevation: 0, .. } = x {
                j = j_start;
                i = i_start;
                break 'outer;
            }
        }
    }
    let result = explore(&mut heightmap, j, i);
    return result.0;
}

fn explore(heightmap: &mut Vec<Vec<Node>>, j: usize, i: usize) -> (i32, bool) {
    let num_cols = heightmap.len();
    let num_rows = heightmap[0].len();

    // check if E nearby in this order: north, south, west, east
    if (j > 0 && heightmap[j - 1][i].is_end)
        || (j < num_cols - 1 && heightmap[j + 1][i].is_end)
        || (i > 0 && heightmap[j][i - 1].is_end)
        || (i < num_rows - 1 && heightmap[j][i + 1].is_end)
    {
        return (heightmap[j][i].cost + 1, true);
    }

    // explore north
    if j > 0 {
        let j_test = j - 1;
        let i_test = i;
        if heightmap[j_test][i_test].cost == i32::MAX
            && heightmap[j_test][i_test].elevation - heightmap[j][i].elevation <= 1
        {
            heightmap[j_test][i_test].cost = heightmap[j][i].cost + 1;
            if let x @ (_, true) = explore(heightmap, j_test, i_test) {
                return x;
            }
        }
    }

    // explore south
    if j < num_cols - 1 {
        let j_test = j + 1;
        let i_test = i;
        if heightmap[j_test][i_test].cost == i32::MAX
            && heightmap[j_test][i_test].elevation - heightmap[j][i].elevation <= 1
        {
            heightmap[j_test][i_test].cost = heightmap[j][i].cost + 1;
            if let x @ (_, true) = explore(heightmap, j_test, i_test) {
                return x;
            }
        }
    }

    // explore west
    if i > 0 {
        let j_test = j;
        let i_test = i - 1;
        if heightmap[j_test][i_test].cost == i32::MAX
            && heightmap[j_test][i_test].elevation - heightmap[j][i].elevation <= 1
        {
            heightmap[j_test][i_test].cost = heightmap[j][i].cost + 1;
            if let x @ (_, true) = explore(heightmap, j_test, i_test) {
                return x;
            }
        }
    }

    // explore east
    if i < num_rows - 1 {
        let j_test = j;
        let i_test = i + 1;
        if heightmap[j_test][i_test].cost == i32::MAX
            && heightmap[j_test][i_test].elevation - heightmap[j][i].elevation <= 1
        {
            heightmap[j_test][i_test].cost = heightmap[j][i].cost + 1;
            if let x @ (_, true) = explore(heightmap, j_test, i_test) {
                return x;
            }
        }
    }

    return (0, false);
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
