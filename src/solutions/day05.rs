use std::io::{BufRead, BufReader, Read};

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let lines: Vec<_> = reader.lines().map(|res| res.unwrap()).collect();

    println!("Part 1: {}", react_wrapper(&lines[0]));
    println!("Part 2: {:?}", remove_and_react(&lines[0]));
}

fn react_wrapper(line: &str) -> usize {
    let mut polymer: Vec<char> = line.chars().collect();
    
    react(&mut polymer)
}

/// In place reacts the polymer, returning final size.
fn react(polymer: &mut [char]) -> usize {
    let empty = '_'; // marker indicating removed items
    let mut last = empty;
    let mut last_i = 0;
    let mut reacted = true;

    while reacted {
        reacted = false;
        for i in 0..polymer.len() {
            let cur = polymer[i];
            if cur == '_' {
                continue;
            }

            if (cur.is_lowercase() && cur.to_ascii_uppercase() == last) ||
                (cur.is_uppercase() && cur.to_ascii_lowercase() == last) {
                polymer[i] = empty;
                polymer[last_i] = empty;
                last = empty;
                last_i = 0;
                reacted = true;
                break;
            }

            last = cur;
            last_i = i;
        }
    }

    polymer.iter().filter(|c| **c != empty).count()
}

fn remove_and_react(line: &str) -> (char, usize) {
    let empty = '_';
    let polymer: Vec<char> = line.chars().collect();
    let mut results = Vec::new();

    for typ in ('a' as u8)..=('z' as u8) {
        let typ_l = typ as char;
        let typ_u = typ_l.to_ascii_uppercase();
        let mut clone = polymer.clone();
        for c in clone.iter_mut() {
            if *c == typ_l || *c == typ_u {
                *c = empty;
            }
        }

        results.push((typ_l, react(&mut clone)));
    }

    results.sort_by_key(|&(t, n)| n);

    results[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn polymer_reduction_example() {
        let poly = "dabAcCaCBAcCcaDA";
        assert_eq!(10, react_wrapper(&poly));
    }

    #[test]
    fn mirrored_polymer() {
        let poly = "ZYXWVUTSRQPONMLKJIHGFEDCBAabcdefghijklmnopqrstuvwxyz";
        assert_eq!(0, react_wrapper(&poly));
    }

    #[test]
    fn polymer_remove_reduce_example() {
        let poly = "dabAcCaCBAcCcaDA";
        assert_eq!(('c', 4), remove_and_react(&poly));
    }
}
