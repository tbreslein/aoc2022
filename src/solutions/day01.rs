pub fn solve_p1(data: &str) -> i32 {
    data.split("\n\n")
        .map(|block| block.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
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
