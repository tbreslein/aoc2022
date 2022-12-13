#[derive(Debug)]
struct Tree {
    pub height: i32,
    pub visible: bool,
}
type Forest = Vec<Vec<Tree>>;

fn parse(data: &str) -> Forest {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree {
                    height: c.to_digit(10).unwrap() as i32,
                    visible: false,
                })
                .collect()
        })
        .collect()
}

pub fn solve_p1(data: &str) -> usize {
    let mut forest = parse(data);
    let cols = forest.len();
    let rows = forest[0].len();
    let mut count = 2 * cols + 2 * rows - 4;
    let mut tallest_height;
    // rows looked at from the west
    for j in 1..cols - 1 {
        tallest_height = forest[j][0].height;
        for i in 1..rows - 1 {
            if !forest[j][i].visible && forest[j][i].height > tallest_height {
                count += 1;
                forest[j][i].visible = true;
            }
            tallest_height = tallest_height.max(forest[j][i].height);
        }
    }
    // rows looked at from the east
    for j in 1..cols - 1 {
        tallest_height = forest[j][cols - 1].height;
        for i in (1..rows - 1).rev() {
            if !forest[j][i].visible && forest[j][i].height > tallest_height {
                count += 1;
                forest[j][i].visible = true;
            }
            tallest_height = tallest_height.max(forest[j][i].height);
        }
    }
    // columns looked at from the north
    for i in 1..rows - 1 {
        tallest_height = forest[0][i].height;
        for j in 1..cols - 1 {
            if !forest[j][i].visible && forest[j][i].height > tallest_height {
                count += 1;
                forest[j][i].visible = true;
            }
            tallest_height = tallest_height.max(forest[j][i].height);
        }
    }
    // columns looked at from the south
    for i in 1..rows - 1 {
        tallest_height = forest[rows - 1][i].height;
        for j in (1..cols - 1).rev() {
            if !forest[j][i].visible && forest[j][i].height > tallest_height {
                count += 1;
                forest[j][i].visible = true;
            }
            tallest_height = tallest_height.max(forest[j][i].height);
        }
    }
    return count;
}

pub fn solve_p2(data: &str) -> usize {
    let forest = parse(data);

    let mut best_score = 0;
    let mut north_score;
    let mut south_score;
    let mut west_score;
    let mut east_score;

    let cols = forest.len();
    let rows = forest[0].len();

    for row in 0..rows {
        for col in 0..cols {
            let this_tree = forest[row][col].height;

            // look north
            north_score = 0;
            for i in (0..row).rev() {
                north_score += 1;
                if forest[i][col].height >= this_tree {
                    break;
                }
            }

            // look south
            south_score = 0;
            for i in row + 1..rows {
                south_score += 1;
                if forest[i][col].height >= this_tree {
                    break;
                }
            }

            // look west
            west_score = 0;
            for j in (0..col).rev() {
                west_score += 1;
                if forest[row][j].height >= this_tree {
                    break;
                }
            }

            // look east
            east_score = 0;
            for j in col + 1..cols {
                east_score += 1;
                if forest[row][j].height >= this_tree {
                    break;
                }
            }

            best_score = best_score.max(north_score * south_score * west_score * east_score);
        }
    }
    // dbg!(&forest);
    return best_score;
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "30373
25512
65332
33549
35390";
        assert_eq!(solve_p1(&data), 21);
    }

    #[test]
    fn p2_test() {
        let data = "30373
25512
65332
33549
35390";
        assert_eq!(solve_p2(&data), 8);
    }
}
