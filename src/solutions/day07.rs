use std::io::{BufRead, BufReader, Read};
use std::ops::Deref;
use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let lines: Vec<_> = reader.lines().map(|res| res.unwrap()).collect();

    println!("Part 1: {}", do_work(&lines));
}

fn do_work<T: Deref<Target = str>>(lines: &[T]) -> String {
    let parsed = parse_input(lines);
    solve(parsed)
}

fn parse_input<T: Deref<Target = str>>(lines: &[T]) -> HashMap<char, Vec<char>> {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let mut nodes = HashMap::new();

    for l in lines {
        let caps = re.captures(l).unwrap();
        let pre_req = caps[1].as_bytes()[0] as char;
        let node = caps[2].as_bytes()[0] as char;

        let n = nodes.entry(node).or_insert(vec![]);
        n.push(pre_req);
    }

    nodes
}

fn solve(work: HashMap<char, Vec<char>>) -> String {
    let mut order = String::new();
    
    // find root nodes
    let mut done: HashSet<char> = HashSet::new();
    {
        let mut not_found = HashSet::new();
        let mut found = HashSet::new();
        for (k, v) in work.iter() {
            not_found.remove(k);
            found.insert(k);
            for c in v {
                if !found.contains(c) {
                    not_found.insert(*c);
                }
            }
        }

        not_found.iter().map(|c| done.insert(*c));
    }

    // add nodes in alphabetical order
    {
        let mut sorted = done.iter().map(|c| *c).collect::<Vec<char>>();
        sorted.sort();
        for c in &sorted {
            order.push(*c);
        }
    }

    // begin solving
    let mut available = HashSet::new();
    'outer: loop {
        // find available nodes
        // TODO replace this with hashset operations?
        // done ^ available?
        for (k, v) in work.iter() {
            if done.contains(k) {
                continue;
            }
 
            for req in v {
                if !done.contains(req) {
                    continue 'outer;
                }
            }

            available.insert(k);
        }

        if available.is_empty() {
            break;
        }

        // add nodes in alphabetical order
        {
            let mut sorted = available.drain().map(|c| *c).collect::<Vec<char>>();
            sorted.sort();
            for c in &sorted {
                order.push(*c)
            }
        }

        available.clear()
    }

    order
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn part_1_example_parse() {
        let example = vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];

        let mut expected = HashMap::new();
        expected.insert('A', vec!['C']);
        expected.insert('F', vec!['C']);
        expected.insert('B', vec!['A']);
        expected.insert('D', vec!['A']);
        expected.insert('E', vec!['B', 'D', 'F']);

        assert_eq!(expected, parse_input(&example));
    }

    #[test]
    fn part_1_example_solve() {
        let example = vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];

        let expected = "CABDFE";

        assert_eq!(expected, do_work(&example));
    }
}
