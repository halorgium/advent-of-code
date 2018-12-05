extern crate regex;
extern crate chrono;

use std::collections::HashMap;
use std::ops::Range;
use chrono::Duration;
use chrono::prelude::*;
use regex::{Regex, Match, Captures};
use Kind::*;

#[derive(Debug, PartialEq)]
struct Record {
    kind: Kind,
    time: DateTime<Utc>,
}

#[derive(Debug, PartialEq)]
enum Kind {
    Shift { guard: u32 },
    Sleep,
    Wake,
}

#[derive(Debug, Clone)]
struct SleepRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl SleepRange {
    fn duration(&self) -> Duration {
        self.end - self.start
    }

    fn minute_range(&self) -> Range<u32> {
        (self.start.minute())..(self.end.minute())
    }
}

#[derive(Debug)]
struct SleepLog {
    guard: u32,
    ranges: Vec<SleepRange>,
}

impl SleepLog {
    fn total_minutes(&self) -> i64 {
        self.ranges.iter().map(|r| r.duration().num_minutes()).sum()
    }

    fn most_asleep_minute(&self) -> (i64, u32) {
        let mut minutes: HashMap<u32, u32> = HashMap::new();

        for range in &self.ranges {
            for minute in range.minute_range() {
                minutes.entry(minute).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        let mut max_count = None;
        let mut max_minute = None;

        for (minute, count) in minutes.iter() {
            if max_count.is_none() || max_count.unwrap() < count {
                max_count = Some(count);
                max_minute = Some(*minute);
            }
        }

        (max_minute.unwrap().into(), *max_count.unwrap())
    }
}

fn main() {
    let input = include_str!("../input.txt");

    println!("most_asleep={}", most_asleep(input));
    println!("most_asleep2={}", most_asleep2(input));
}

fn most_asleep(input: &str) -> i64 {
    let sleep_logs = build_sleep_logs(input);

    let mut max_sleep_log = None;
    let mut max_total = None;

    for sleep_log in sleep_logs {
        let total = sleep_log.total_minutes();
        if max_total.is_none() || max_total.unwrap() < total {
            max_sleep_log = Some(sleep_log);
            max_total = Some(total);
        }
    }

    println!("max_sleep_log={:?} max_total={:?}", max_sleep_log, max_total);

    let max_sleep_log = max_sleep_log.unwrap();
    let guard: i64 = max_sleep_log.guard.into();

    let (most_asleep_minute, _) = max_sleep_log.most_asleep_minute();
    println!("most_asleep_minute={:?}", most_asleep_minute);


    guard * most_asleep_minute
}

fn most_asleep2(input: &str) -> i64 {
    let sleep_logs = build_sleep_logs(input);

    let mut max_count = None;
    let mut max_minute = None;
    let mut max_sleep_log = None;

    for sleep_log in sleep_logs {
        let (minute, count) = sleep_log.most_asleep_minute();
        if max_count.is_none() || max_count.unwrap() < count {
            max_count = Some(count);
            max_minute = Some(minute);
            max_sleep_log = Some(sleep_log);
        }
    }

    let guard: i64 = max_sleep_log.unwrap().guard.into();
    let max_minute: i64 = max_minute.unwrap().into();

    println!("guard={} max_minute={}", guard, max_minute);

    guard * max_minute
}

fn build_sleep_logs(input: &str) -> Vec<SleepLog> {
    let records = build_records(input);
    println!("records={:?}", records);

    let sleep_ranges = build_sleep_ranges(records);
    println!("sleep_ranges={:?}", sleep_ranges);

    let sleep_logs: Vec<SleepLog> = sleep_ranges.iter().map(|(g,r)| SleepLog { guard: *g, ranges: r.to_vec() }).collect();
    println!("sleep_logs={:?}", sleep_logs);

    sleep_logs
}

fn build_sleep_ranges(records: Vec<Record>) -> HashMap<u32, Vec<SleepRange>> {
    let mut sleep_ranges = HashMap::new();

    let mut current_guard = None;
    let mut sleep_time = None;

    for record in records {
        match record.kind {
            Shift { guard } => {
                assert_eq!(sleep_time, None);
                current_guard = Some(guard.clone());
            },
            Sleep => {
                assert_eq!(sleep_time, None);
                sleep_time = Some(record.time);
            },
            Wake => {
                let entry = sleep_ranges.entry(current_guard.unwrap()).or_insert(Vec::new());

                entry.push(SleepRange {
                    start: sleep_time.unwrap(),
                    end: record.time,
                });
                sleep_time = None;
            }
        }
    }

    sleep_ranges
}

fn build_records(input: &str) -> Vec<Record> {
    let mut records: Vec<Record> = input.lines().map(|line| parse(line).unwrap()).collect();
    records.sort_by_key(|r| r.time);
    records
}

// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up

fn parse(content: &str) -> Option<Record> {
    let shift_re = Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+)\] Guard #(?P<guard>\d+) begins shift").unwrap();
    let sleep_re = Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+)\] falls asleep").unwrap();
    let wake_re = Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+)\] wakes up").unwrap();

    if let Some(captures) = shift_re.captures(content) {
        return Some(Record {
            kind: Shift { guard: extract_u32(captures.name("guard")) },
            time: extract_time(&captures),
        })
    }

    if let Some(captures) = sleep_re.captures(content) {
        return Some(Record {
            kind: Sleep,
            time: extract_time(&captures),
        })
    }

    if let Some(captures) = wake_re.captures(content) {
        return Some(Record {
            kind: Wake,
            time: extract_time(&captures),
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

        assert_eq!(most_asleep(input), 240);
        assert_eq!(most_asleep2(input), 4455);
    }

    #[test]
    fn parse_shift() {
        let record = Record {
            kind: Shift { guard: 10 },
            time: Utc.ymd(1518, 11, 1).and_hms(0, 0, 0),
        };
        assert_eq!(parse("[1518-11-01 00:00] Guard #10 begins shift"), Some(record));
    }

    #[test]
    fn parse_sleep() {
        let record = Record {
            kind: Sleep,
            time: Utc.ymd(1518, 11, 1).and_hms(0, 5, 0),
        };
        assert_eq!(parse("[1518-11-01 00:05] falls asleep"), Some(record));
    }

    #[test]
    fn parse_wake() {
        let record = Record {
            kind: Wake,
            time: Utc.ymd(1518, 11, 1).and_hms(0, 25, 0),
        };
        assert_eq!(parse("[1518-11-01 00:25] wakes up"), Some(record));
    }
}
