use std::fs;
use std::fmt;
use std::cmp;
use std::collections::HashMap;
use std::ops;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Timestamp {
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64
}

impl Timestamp {
    fn from_string(s: &String) -> Timestamp {
        let ts: Vec<i64> = s.split(|c| {
            "[- :]".chars().collect::<Vec<char>>().contains(&c)
        }).filter(|s| s != &"").map(|s| {
            match s.parse::<i64>() {
                Ok(i) => i,
                Err(_) => panic!("Couldn't parse timestamp {}", s)
            }
        }).collect();

        Timestamp {
            year: ts[0],
            month: ts[1],
            day: ts[2],
            hour: ts[3],
            minute: ts[4]
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:#04}-{:#02}-{:#02} {:#02}:{:#02}]",
               self.year, self.month, self.day,
               self.hour, self.minute)
    }
}

impl cmp::Ord for Timestamp {
    fn cmp(&self, other: &Timestamp) -> cmp::Ordering {
        self.year.cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl cmp::PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Sub for Timestamp {
    type Output = i64;
    fn sub(self, other: Timestamp) -> i64 {
        self.minute - other.minute
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum RecordKind {
    BeginsShift, FallsAsleep, WakesUp
}

impl fmt::Display for RecordKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            RecordKind::BeginsShift => "begins shift",
            RecordKind::FallsAsleep => "falls asleep",
            RecordKind::WakesUp => "wakes up"
        })
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Record {
    timestamp: Timestamp,
    guard_id: Option<i64>,
    kind: RecordKind
}

impl Record {
    fn from_string(s: &String) -> Record {
        let timestamp_tokens = s.split("]")
                      .map(|s| s.to_string())
                      .collect::<Vec<String>>();

        let tokens: Vec<String> = timestamp_tokens[1]
                                .split(|c| c == ' ' || c == '#')
                                .filter(|s| s != &"")
                                .map(|s| s.to_string())
                                .collect();

        let timestamp = Timestamp::from_string(&timestamp_tokens[0]);

        let kind = match tokens[0].as_ref() {
            "Guard" => RecordKind::BeginsShift,
            "falls" => RecordKind::FallsAsleep,
            "wakes" => RecordKind::WakesUp,
            _ => panic!("Couldn't parse {}", s)
        };

        let mut guard_id = None;
        if kind == RecordKind::BeginsShift {
            guard_id = match tokens[1].parse::<i64>() {
                Ok(i) => Some(i),
                Err(_) => None
            };
        };

        Record { timestamp, guard_id, kind }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.kind == RecordKind::BeginsShift {
            write!(f, "{} Guard #{} {}", self.timestamp, self.guard_id.unwrap(), self.kind)
        }
        else {
            write!(f, "{} {}", self.timestamp, self.kind)
        }
    }
}

impl cmp::Ord for Record {
    fn cmp(&self, other: &Record) -> cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl cmp::PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_1(records_by_guard: &HashMap<i64, Vec<Record>>) -> (i64, i64) {
    let mut laziest_guard_id: Option<i64> = None;
    let mut laziest_sleep_time: i64 = 0;
    for (id, rs) in records_by_guard.iter() {
        let mut minutes_asleep = 0;

        let (sleep, wake): (Vec<Record>, Vec<Record>) = rs
                            .iter()
                            .partition(|&r| r.kind == RecordKind::FallsAsleep);

        for (s, w) in sleep.iter().zip(wake.iter()) {
            minutes_asleep += w.timestamp - s.timestamp;
        };

        if minutes_asleep > laziest_sleep_time {
            laziest_sleep_time = minutes_asleep;
            laziest_guard_id = Some(*id);
        };
    };

    let laziest_records = records_by_guard.get(&laziest_guard_id.unwrap()).unwrap();
    let (sleep, wake): (Vec<Record>, Vec<Record>) = laziest_records
        .iter()
        .partition(|&r| r.kind == RecordKind::FallsAsleep);

    let mut asleep_by_minute: HashMap<i64, i64> = HashMap::new();
    for (s, w) in sleep.iter().zip(wake.iter()) {
        for minute in (s.timestamp.minute)..(w.timestamp.minute) {
            *asleep_by_minute.entry(minute)
                .or_insert(0)
                += 1;
        };
    };

    let mut laziest_minute = None;
    let mut laziest_count = 0;
    for (&minute, &count) in asleep_by_minute.iter() {
        if count > laziest_count {
            laziest_count = count;
            laziest_minute = Some(minute);
        };
    };

    (laziest_guard_id.unwrap(), laziest_minute.unwrap())
}

fn part_2(records_by_guard: &HashMap<i64, Vec<Record>>) -> (i64, i64) {
    let mut asleep_by_guard_and_minute: HashMap<(i64, i64), i64> = HashMap::new();

    for (&id, rs) in records_by_guard.iter() {
        let (sleep, wake): (Vec<Record>, Vec<Record>) = rs
            .iter()
            .partition(|&r| r.kind == RecordKind::FallsAsleep);

        for (s, w) in sleep.iter().zip(wake.iter()) {
            for minute in (s.timestamp.minute)..(w.timestamp.minute) {
                *asleep_by_guard_and_minute.entry((id, minute))
                    .or_insert(0)
                    += 1;
            };
        };
    };

    let mut consistentest_guard = None;
    let mut consistentest_minute = None;
    let mut consistentest_count = 0;

    for (&(id, minute), &count) in asleep_by_guard_and_minute.iter() {
        if count > consistentest_count {
            consistentest_guard = Some(id);
            consistentest_minute = Some(minute);
            consistentest_count = count;
        };
    };

    (consistentest_guard.unwrap(), consistentest_minute.unwrap())
}

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("input.txt")?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let mut records = lines.iter()
        .map(Record::from_string)
        .collect::<Vec<Record>>();

    records.sort_unstable();

    let mut records_by_guard: HashMap<i64, Vec<Record>> = HashMap::new();

    let mut current_guard_id: Option<i64> = None;
    for mut record in records {
        match record.guard_id {
            Some(id) => current_guard_id = Some(id),
            None => record.guard_id = current_guard_id
        };
        match record.kind {
            RecordKind::BeginsShift => (),
            _ => records_by_guard
                    .entry(current_guard_id.unwrap())
                    .or_insert(Vec::new())
                    .push(record)
        };
    };

    let (laziest_id, laziest_minute) = part_1(&records_by_guard);
    println!("Part 1: guard {} was the laziest, most often asleep at minute {}. Answer = {}",
             laziest_id, laziest_minute, laziest_id * laziest_minute);

    let (consistentest_id, consistentest_minute) = part_2(&records_by_guard);
    println!("Part 2: guard {} was the most consistent, most often asleep at minute {}. Answer = {}",
             consistentest_id, consistentest_minute, consistentest_id * consistentest_minute);
    Ok(())
}
