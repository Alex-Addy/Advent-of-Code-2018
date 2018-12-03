use std::io::{BufRead, BufReader, Read};
use std::ops::Deref;

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let boxes: Vec<_> = reader.lines().map(|res| res.unwrap()).collect();

    println!("Part 1: {}", checksum(&boxes));
    println!("Part 2: {}", off_by_one(&boxes));
}

fn checksum<T: Deref<Target = str>>(boxes: &[T]) -> usize {
    let mut doubles = 0;
    let mut triples = 0;
    for s in boxes {
        let mut letters = vec![0u8; 26];
        for b in s.bytes() {
            assert!(b >= 0x61 && b <= 0x7A);
            letters[(b - 0x61) as usize] += 1;
        }

        let found_double = letters.iter().any(|&n| n == 2);
        let found_triple = letters.iter().any(|&n| n == 3);

        if found_double {
            doubles += 1;
        }
        if found_triple {
            triples += 1;
        }
    }

    doubles * triples
}

fn off_by_one<T: Deref<Target = str>>(boxes: &[T]) -> String {
    let mut ids = None;
    for j in 0..boxes.len() {
        for k in (j + 1)..boxes.len() {
            if distance(&boxes[j], &boxes[k]) == 1 {
                ids = Some((boxes[j].to_string(), boxes[k].to_string()));
                break;
            }
        }
    }
    let (left, right) = ids.expect("bad input, could not find ids off by one");

    left.chars()
        .zip(right.chars())
        .filter(|&t| t.0 == t.1)
        .map(|t| t.0)
        .collect()
}

/// Return the number of characters that are different between the strings.
///
/// Will panic if the strings are not the same length.
fn distance(left: &str, right: &str) -> usize {
    if left.len() != right.len() {
        panic!("variable string length not supported!");
    }

    left.chars()
        .zip(right.chars())
        .filter(|&t| t.0 != t.1)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_each() {
        let boxes = vec!["abcdefff", "aabcdef"];
        assert_eq!(1, checksum(&boxes));
    }

    #[test]
    fn example_checksum() {
        let boxes = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        assert_eq!(12, checksum(&boxes));
    }

    #[test]
    fn example_off_by_one() {
        let boxes = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        assert_eq!("fgij", off_by_one(&boxes));
    }
}
