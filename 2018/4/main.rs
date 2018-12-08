use std::fs;

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

#[derive(PartialEq, Eq)]
enum RecordKind {
    BeginsShift, FallsAsleep, WakesUp
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

        Record { timestamp, kind, guard_id }
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

    Record::from_string(&lines[0]);
    Record::from_string(&lines[1]);
    Record::from_string(&lines[10]);

    Ok(())
}
