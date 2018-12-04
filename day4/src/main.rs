extern crate regex;
extern crate chrono;

use chrono::prelude::*;
use regex::{Regex, Match, Captures};

fn main() {
    let input = include_str!("../input.txt");
}

fn most_asleep(input: &str) -> u32 {
    10
}

#[derive(Debug, PartialEq)]
enum RecordType {
    Shift,
    Sleep,
    Wake,
}

#[derive(Debug, PartialEq)]
struct Record {
    record_type: RecordType,
    ts: DateTime<Utc>,
    guard: u32,
}

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up

fn parse(content: &str, last_guard: Option<u32>) -> Option<Record> {
    let shift_re = Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+)\] Guard #(?P<guard>\d+) begins shift").unwrap();
    let sleep_re = Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+)\] falls asleep").unwrap();
    let wake_re = Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+)\] wakes up").unwrap();

    if let Some(captures) = shift_re.captures(content) {
        return Some(Record {
            record_type: RecordType::Shift,
            ts: extract_time(&captures),
            guard: extract_u32(captures.name("guard")),
        })
    }

    if let Some(captures) = sleep_re.captures(content) {
        return Some(Record {
            record_type: RecordType::Sleep,
            ts: extract_time(&captures),
            guard: last_guard.unwrap(),
        })
    }

    if let Some(captures) = wake_re.captures(content) {
        return Some(Record {
            record_type: RecordType::Wake,
            ts: extract_time(&captures),
            guard: last_guard.unwrap(),
        })
    }

    None
}

fn extract_time(captures: &Captures) -> DateTime<Utc> {
    let year = extract_i32(captures.name("y"));
    let month = extract_u32(captures.name("m"));
    let day = extract_u32(captures.name("d"));

    let hour = extract_u32(captures.name("H"));
    let minute = extract_u32(captures.name("M"));

    Utc.ymd(year, month, day).and_hms(hour, minute, 0)
}

fn extract_u32(value: Option<Match>) -> u32 {
    value.unwrap().as_str().parse().unwrap()
}

fn extract_i32(value: Option<Match>) -> i32 {
    value.unwrap().as_str().parse().unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = include_str!("fixtures/simple.txt");

        assert_eq!(most_asleep(input), 10);
    }

    #[test]
    fn parse_shift() {
        let record = Record {
            record_type: RecordType::Shift,
            ts: Utc.ymd(1518, 11, 1).and_hms(0, 0, 0),
            guard: 10,
        };
        assert_eq!(parse("[1518-11-01 00:00] Guard #10 begins shift", None), Some(record));
    }

    #[test]
    fn parse_sleep() {
        let record = Record {
            record_type: RecordType::Sleep,
            ts: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
            guard: 10,
        };
        assert_eq!(parse("[1518-11-01 00:05] falls asleep", Some(10)), Some(record));
    }

    #[test]
    fn parse_wake() {
        let record = Record {
            record_type: RecordType::Wake,
            ts: Utc.ymd(1518, 11, 1).and_hms(0, 25, 0),
            guard: 10,
        };
        assert_eq!(parse("[1518-11-01 00:25] wakes up", Some(10)), Some(record));
    }
}
