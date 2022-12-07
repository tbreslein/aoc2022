pub fn solve_p1(data: &str) -> i32 {
    return data
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let (ll, lr) = l.split_once('-').unwrap();
            let (rl, rr) = r.split_once('-').unwrap();
            return (
                ll.parse::<i32>().unwrap(),
                lr.parse::<i32>().unwrap(),
                rl.parse::<i32>().unwrap(),
                rr.parse::<i32>().unwrap(),
            );
        })
        .fold(0, |acc, (ll, lr, rl, rr)| {
            if (ll >= rl && lr <= rr) || (rl >= ll && rr <= lr) {
                acc + 1
            } else {
                acc
            }
        });
}

pub fn solve_p2(data: &str) -> i32 {
    return data
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            let (ll, lr) = l.split_once('-').unwrap();
            let (rl, rr) = r.split_once('-').unwrap();
            return (
                ll.parse::<i32>().unwrap(),
                lr.parse::<i32>().unwrap(),
                rl.parse::<i32>().unwrap(),
                rr.parse::<i32>().unwrap(),
            );
        })
        .fold(0, |acc, (ll, lr, rl, rr)| {
            if (lr >= rl && ll <= rl) || (rr >= ll && rl <= ll) {
                acc + 1
            } else {
                acc
            }
        });
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_p1(&data), 2);
    }

    #[test]
    fn p2_test() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve_p2(&data), 4);
    }
}
