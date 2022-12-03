use std::collections::HashMap;
use std::iter::zip;

pub fn solve_p1(data: &str) -> i32 {
    let mut prio = HashMap::<char, i32>::new();
    for (c, i) in zip('a'..='z', 1..) {
        prio.insert(c, i);
    }
    for (c, i) in zip('A'..='Z', 27..) {
        prio.insert(c, i);
    }

    data.lines()
        .map(|line| line.split_at((0.5 * line.len() as f64) as usize))
        .map(|(l, r)| {
            for c in l.chars() {
                if r.contains(c) {
                    return *prio.get(&c).unwrap();
                }
            }
            return 0;
        })
        .sum()
}

pub fn solve_p2(data: &str) -> i32 {
    let tuple = data
        .split("\n\n")
        .map(|block| block.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .fold((0, 0, 0), |t, x| {
            if x > t.0 {
                (x, t.0, t.1)
            } else if x > t.1 {
                (t.0, x, t.1)
            } else if x > t.2 {
                (t.0, t.1, x)
            } else {
                t
            }
        });
    return tuple.0 + tuple.1 + tuple.2;
}
