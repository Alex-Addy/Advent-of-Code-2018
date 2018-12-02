
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read};

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let changes: Vec<_> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    info!("Part 1: {}", solve_p1(&changes));
    info!("Part 2: {}", solve_p2(&changes));
}

fn solve_p1(changes: &[isize]) -> isize {
    changes.iter().sum()
}

fn solve_p2(changes: &[isize]) -> isize {
    let mut freqs = HashSet::new();
    let mut f = 0;
    let mut i = 0;
    loop {
        f += changes[i % changes.len()];
        if freqs.contains(&f) {
            break;
        }
        freqs.insert(f);
        i += 1;
    }

    f
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positive_values() {
        let input = vec![1, 2, 3, 1, 3];
        assert_eq!(10, solve_p1(&input));
    }

    #[test]
    fn negative_values() {
        let input = vec![-1, -2, -3, -1, -3];
        assert_eq!(-10, solve_p1(&input));
    }

    #[test]
    fn mixed_values() {
        let input = vec![-1, 2, 0, -3, 1, -3];
        assert_eq!(-4, solve_p1(&input));
    }

    #[test]
    fn repeat_test() {
        let changes = vec![1, -2, 3, 1];
        assert_eq!(2, solve_p2(&changes));
    }
}
