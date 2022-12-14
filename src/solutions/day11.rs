#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Sqr,
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    pub op: Operation,
    pub test_value: u64,
    pub throw_if_true_to: usize,
    pub throw_if_false_to: usize,
    pub counter: usize,
}

impl Monkey {
    pub fn inspect_items(&mut self, m: u64, divisor: u64) {
        for item in self.items.iter_mut() {
            *item = match self.op {
                Operation::Add(x) => *item + x,
                Operation::Mul(x) => *item * x,
                Operation::Sqr => *item * *item,
            }
            .div_floor(divisor)
                % m;
        }
        self.counter += self.items.len();
    }

    pub fn get_throw_index(&self) -> usize {
        if self.items.first().unwrap() % self.test_value == 0 {
            self.throw_if_true_to
        } else {
            self.throw_if_false_to
        }
    }
}

fn parse(data: &str) -> Vec<Monkey> {
    data.split("\n\n")
        .map(|monkey_str| {
            let mut monkey_lines = monkey_str.lines();
            let _ = monkey_lines.next().unwrap();
            let items = monkey_lines
                .next()
                .unwrap()
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();
            let op = match monkey_lines
                .next()
                .unwrap()
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_once(" ")
                .unwrap()
            {
                ("*", "old") => Operation::Sqr,
                ("*", val) => Operation::Mul(val.parse().unwrap()),
                ("+", val) => Operation::Add(val.parse().unwrap()),
                _ => panic!("wtf"),
            };
            let test_value = monkey_lines
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap();
            let throw_if_true_to = monkey_lines
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            let throw_if_false_to = monkey_lines
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap();
            return Monkey {
                items,
                op,
                test_value,
                throw_if_true_to,
                throw_if_false_to,
                counter: 0,
            };
        })
        .collect()
}

fn solve(data: &str, rounds: i32, divisor: u64) -> usize {
    let mut monkeys = parse(data);
    // yay, modulo arithmatic...
    // This is the safety factor to keep the worry levels in check by modulo-ing each worry level
    // after its inspection by this value. Funnily enough, since the test_values seem to all be
    // prime numbers, this is the same value I would get if I determined the LCM for those values,
    // but this is quicker to write down.
    let m = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test_value);

    for _ in 1..=rounds {
        for i in 0..monkeys.len() {
            monkeys[i].inspect_items(m, divisor);
            for _ in 0..monkeys[i].items.len() {
                let throw_index = monkeys[i].get_throw_index();
                let item = monkeys[i].items.remove(0);
                monkeys[throw_index].items.push(item);
            }
        }
    }
    let max_vals = monkeys.iter().fold((0, 0), |(max1, max2), monkey| {
        if monkey.counter > max1 {
            (monkey.counter, max1)
        } else if monkey.counter > max2 {
            (max1, monkey.counter)
        } else {
            (max1, max2)
        }
    });
    return max_vals.0 * max_vals.1;
}

pub fn solve_p1(data: &str) -> usize {
    solve(data, 20, 3)
}

pub fn solve_p2(data: &str) -> usize {
    solve(data, 10_000, 1)
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(solve_p1(&data), 10605);
    }

    #[test]
    fn p2_test() {
        let data = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(solve_p2(&data), 2713310158);
    }
}
