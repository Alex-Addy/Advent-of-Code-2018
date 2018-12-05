use std::io::{BufRead, BufReader, Read};
use std::ops::Deref;

use regex::Regex;

const DATE_REG: &str = r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]";

pub fn work<R: Read>(r: R) {
    let reader = BufReader::new(r);

    let lines: Vec<_> = reader.lines().map(|res| res.unwrap()).collect();

    println!("Part 1: {}", most_asleep_guard_minute(&lines));
}

fn most_asleep_guard_minute<T: Deref<Target = str>>(lines: &[T]) -> usize {
    let records: Vec<_> = lines.iter().map(|s| Record::from_line(s)).collect();

    0
}

#[derive(PartialEq, Debug)]
struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl Date {
    fn from_line(line: &str) -> Self {
        let re = Regex::new(DATE_REG).unwrap();
        let caps = re.captures(line).unwrap();
        let date_nums: Vec<usize> = caps
            .iter()
            .skip(1) // first item is match for entire regex
            .map(|m| m.unwrap().as_str().parse().unwrap())
            .collect();
        Date {
            year: date_nums[0],
            month: date_nums[1],
            day: date_nums[2],
            hour: date_nums[3],
            minute: date_nums[4],
        }
    }
}

#[derive(PartialEq, Debug)]
enum Action {
    Sleep,
    WakeUp,
    Guard(usize),
}

impl Action {
    fn from_line(line: &str) -> Self {
        let wake_re = Regex::new(r"wakes up").unwrap();
        let sleep_re = Regex::new(r"falls asleep").unwrap();
        if wake_re.is_match(line) {
            Action::WakeUp
        } else if sleep_re.is_match(line) {
            Action::Sleep
        } else {
            let guard_re = Regex::new(r"Guard #(\d+)").unwrap();
            let caps = guard_re.captures(line).unwrap();
            let guard = caps[1].parse().unwrap();
            Action::Guard(guard)
        }
    }
}

struct Record {
    time: Date,
    act: Action,
}

impl Record {
    fn from_line(line: &str) -> Self {
        Record {
            time: Date::from_line(line),
            act: Action::from_line(line),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn use_regex_cap_groups() {
        let line = "[1518-11-01 00:05] falls asleep";
        let re = Regex::new(DATE_REG).unwrap();
        let caps = re.captures(line).unwrap();
        let strs: Vec<&str> = caps.iter().map(|m| m.unwrap().as_str()).collect();
        let expected = vec!["1518", "11", "01", "00", "05"];
        assert_eq!(expected.as_slice(), &strs[1..]);
    }

    #[test]
    fn get_date_from_line() {
        let line = "[1518-11-01 00:05] falls asleep";
        let date = Date::from_line(line);
        let expected = Date {
            year: 1518,
            month: 11,
            day: 01,
            hour: 00,
            minute: 05,
        };
        assert_eq!(expected, date);
    }

    #[test]
    fn get_wake_action_from_line() {
        let line = "[1518-11-21 00:27] wakes up";
        let action = Action::from_line(line);
        assert_eq!(Action::WakeUp, action);
    }

    #[test]
    fn get_sleep_action_from_line() {
        let line = "[1518-04-05 00:03] falls asleep";
        let action = Action::from_line(line);
        assert_eq!(Action::Sleep, action);
    }

    #[test]
    fn get_guard_action_from_line() {
        let line = "[1518-11-07 23:59] Guard #683 begins shift";
        let action = Action::from_line(line);
        assert_eq!(Action::Guard(683), action);
    }

    #[test]
    fn part_1_example() {
        let lines = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep          ",
            "[1518-11-01 00:25] wakes up              ",
            "[1518-11-01 00:30] falls asleep          ",
            "[1518-11-01 00:55] wakes up              ",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep          ",
            "[1518-11-02 00:50] wakes up              ",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep          ",
            "[1518-11-03 00:29] wakes up              ",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep          ",
            "[1518-11-04 00:46] wakes up              ",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep          ",
            "[1518-11-05 00:55] wakes up              ",
        ];

        assert_eq!(240, most_asleep_guard_minute(&lines));
    }
}
