#[derive(Debug)]
struct Step {
    amount: usize,
    from: usize,
    to: usize,
}
type Instructions = Vec<Step>;
type Stack = Vec<char>;
type Hold = Vec<Stack>;

fn parse(data: &str) -> (Hold, Instructions) {
    let (raw_hold, raw_instructions) = data.split_once("\n\n").unwrap();
    let mut hold_iterator = raw_hold.lines().rev();
    let length = hold_iterator
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .len();
    dbg!(length);
    let mut hold: Hold = vec![vec![]; length];
    for line in hold_iterator {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..length {
            let pos = 1 + i * 4;
            if chars.len() < pos {
                continue;
            }
            if chars[pos].is_alphabetic() {
                hold[i].push(chars[pos]);
            }
        }
    }
    let instructions = raw_instructions
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            return Step {
                amount: words[1].parse::<usize>().unwrap(),
                from: words[3].parse().unwrap(),
                to: words[5].parse().unwrap(),
            };
        })
        .collect();
    return (hold, instructions);
}

pub fn solve_p1(data: &str) -> i32 {
    let (hold, instructions) = parse(data);
    dbg!(&hold);
    dbg!(&instructions);
    0
}

pub fn solve_p2(data: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve_p1(&data), 2);
    }

    #[test]
    fn p2_test() {
        let data = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve_p2(&data), 4);
    }
}
