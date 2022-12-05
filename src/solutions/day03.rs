use std::collections::HashMap;
use std::iter::zip;

fn get_prios() -> HashMap<char, i32> {
    let mut prio = HashMap::<char, i32>::new();
    for (c, i) in zip(('a'..='z').chain('A'..='Z'), 1..) {
        prio.insert(c, i);
    }
    return prio;
}


pub fn solve_p1(data: &str) -> i32 {
    let prio = get_prios();
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
    let prio = get_prios();
    let mut tally = 0;
    let mut lines = data.lines().into_iter();
    while let Ok(chunk) = lines.next_chunk::<3>() {
        for c in chunk[0].chars() {
            if chunk[1].contains(c) && chunk[2].contains(c) {
                tally += *prio.get(&c).unwrap();
                break;
            }
        }
    }
    return tally;
}

#[cfg(test)]
mod test {
    use super::solve_p2;

    #[test]
    fn p2_test () {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n";
        assert_eq!(solve_p2(data), 70);
    }
}

