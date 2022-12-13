use std::collections::HashSet;

#[derive(Debug)]
enum Command {
    R(usize),
    L(usize),
    D(usize),
    U(usize),
}
type CommandList = Vec<Command>;

fn parse(data: &str) -> CommandList {
    return data
        .lines()
        .map(|line| {
            if let Some((direction, steps)) = line.split_once(" ") {
                return match direction {
                    "R" => Command::R(steps.parse().unwrap()),
                    "L" => Command::L(steps.parse().unwrap()),
                    "U" => Command::U(steps.parse().unwrap()),
                    "D" => Command::D(steps.parse().unwrap()),
                    _ => panic!("wtf"),
                };
            };
            panic!("wtf2");
        })
        .collect();
}

fn are_adjacent(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1
}

fn move_rope<const S: usize>(rope: &mut [(i32, i32); S], set: &mut HashSet<(i32, i32)>) {
    for i in 1..S {
        if are_adjacent(&rope[i-1], &rope[i]) {
            continue;
        }
        if rope[i-1].0 > rope[i].0 {
            rope[i].0 = (rope[i-1].0 + rope[i].0).div_ceil(2);
        } else {
            rope[i].0 = (rope[i-1].0 + rope[i].0).div_floor(2);
        }
        if rope[i-1].1 > rope[i].1 {
            rope[i].1 = (rope[i-1].1 + rope[i].1).div_ceil(2);
        } else {
            rope[i].1 = (rope[i-1].1 + rope[i].1).div_floor(2);
        }
    }
    set.insert(rope.last().unwrap().clone());
}

pub fn solve_p1(data: &str) -> usize {
    solve::<2>(&data)
}

pub fn solve_p2(data: &str) -> usize {
    solve::<9>(&data)
}

fn solve<const S: usize>(data: &str) -> usize {
    let commands = parse(data);
    let mut set = HashSet::new();
    let mut rope = &mut [(0, 0); S];

    set.insert(rope.last().unwrap().clone());

    for command in commands.iter() {
        match command {
            Command::R(steps) => {
                for _ in 1..=*steps {
                    rope[0] = (rope[0].0, rope[0].1 + 1);
                    move_rope(&mut rope, &mut set);
                }
            }
            Command::L(steps) => {
                for _ in 1..=*steps {
                    rope[0] = (rope[0].0, rope[0].1 - 1);
                    move_rope(&mut rope, &mut set);
                }
            }
            Command::U(steps) => {
                for _ in 1..=*steps {
                    rope[0] = (rope[0].0 + 1, rope[0].1);
                    move_rope(&mut rope, &mut set);
                }
            }
            Command::D(steps) => {
                for _ in 1..=*steps {
                    rope[0] = (rope[0].0 - 1, rope[0].1);
                    move_rope(&mut rope, &mut set);
                }
            }
        };
    }
    return set.len();
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(solve_p1(&data), 13);
    }

    #[test]
    fn p2_test() {
        let data = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(solve_p2(&data), 36);
    }
}
