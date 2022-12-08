// this could probably be done way more efficiently
fn has_duplicates(slc: &[char]) -> bool {
    let mut chars = vec![' '; slc.len()];
    chars.copy_from_slice(slc);
    chars.sort();
    let (_, dups) = chars.partition_dedup();
    return !dups.is_empty();
}

pub fn solve_p1(data: &str) -> usize {
    let chars = data.chars().collect::<Vec<char>>();
    let mut iter = chars.windows(4);
    for i in 4.. {
        if !has_duplicates(iter.next().unwrap()){
            return i;
        }
    }
    return 0;
}

pub fn solve_p2(data: &str) -> usize {
    let chars = data.chars().collect::<Vec<char>>();
    let mut iter = chars.windows(14);
    for i in 14.. {
        if !has_duplicates(iter.next().unwrap()){
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::solve_p1;
    use super::solve_p2;

    #[test]
    fn p1_test() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_p1(&data), 7);
    }

    #[test]
    fn p2_test() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_p2(&data), 19);
    }
}
