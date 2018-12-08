use std::fs;
use std::fmt;

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

#[derive(PartialEq, Eq)]
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


fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("input.txt")?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let records: Vec<Record> = lines.iter().map(Record::from_string).collect();
    for record in records {
        println!("{}", record);
    };

    Ok(())
}
