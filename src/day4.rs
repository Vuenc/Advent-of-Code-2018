// use std::cmp::{max};
use regex::Regex;
use self::GuardAction::*;
use bit_vec::BitVec;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum GuardAction {
    BeginsShift,
    WakesUp,
    FallsAsleep
}

#[derive(Debug)]
struct GuardEvent {
    date: String,
    minute: i32,
    guard_id: Option<i32>,
    action: GuardAction
}

#[derive(Debug)]
struct Day {
    guard_id: i32,
    minutes: BitVec,
    minutes_awake: i32
}

impl Day {
    pub fn new(guard_id: i32) -> Day {
        let minutes = BitVec::with_capacity(60);
        Day {guard_id, minutes, minutes_awake: 0}
    }

    pub fn set_next_minute(&mut self, awake: bool) {
        self.minutes.push(awake);
        self.minutes_awake += i32::from(awake);
    }
}

impl GuardEvent {
    pub fn from_line(line: &String, regex: &Regex) -> GuardEvent {
        let captures = regex.captures(line).expect("No captures for line!");
        let action = match &captures[3] {
            "wakes" => WakesUp,
            "falls" => FallsAsleep,
            _ => BeginsShift
        };
        GuardEvent {date: captures[1].to_string(),
                minute: captures[2].replace(":", "").parse().unwrap(),
                guard_id: captures.get(4).map(|id| id.as_str().parse().unwrap()),
                action
        }
    }
}

fn initialize(lines: &Vec<String>) -> Vec<Day> {
    let regex = Regex::new(r"(\d\d-\d\d) ((?:23|00):\d\d)\] (Guard #(\d*)|wakes|falls)").expect("Building Regex failed");
    let mut events = lines.iter().map(|l| GuardEvent::from_line(l, &regex)).collect::<Vec<GuardEvent>>();
    events.sort_by(|GuardEvent {date: date1, minute: minute1, ..}, GuardEvent {date: date2, minute: minute2, ..}| {
        date1.cmp(date2).then(minute1.cmp(minute2))
    });

    let mut days = Vec::new();
    let mut events_iter = events.iter();
    let mut event_option = events_iter.next();
    while event_option.is_some() {
        let event = event_option.unwrap();
        assert_eq!(event.action, BeginsShift);
        let mut current_day = Day::new(event.guard_id.unwrap());
        let mut is_awake = true;
        event_option = events_iter.next();
        for minute in 0..60 {
            if event_option.map_or(false, |e| e.action != BeginsShift && e.minute == minute) {
                is_awake = !is_awake;
                event_option = events_iter.next();
            }
            current_day.set_next_minute(is_awake);
        }
        days.push(current_day);
    }
    days
}

pub fn star1(lines: &Vec<String>) -> String {
    let days = initialize(lines);

    let mut guard_map = HashMap::new();
    for day in days {
        guard_map.entry(day.guard_id)
            .or_insert(vec![])
            .push(day);
    }

    let (&sleepiest_guard_id, sleepiest_guard_days) = guard_map.iter()
        .max_by_key(|(_, v)| v.iter()
            .map(|day| 60 - day.minutes_awake)
            .sum::<i32>()
    ).unwrap();
    let mut sleepiest_guard_awake_by_minutes = vec![0; 60];
    for day in sleepiest_guard_days {
        // println!("Day: {:?}", day);
        for minute in 0..60 {
            sleepiest_guard_awake_by_minutes[minute] += i32::from(day.minutes[minute]);
        }
    }
    let (max_minute, _) = sleepiest_guard_awake_by_minutes.iter().enumerate().min_by_key(|(_, times)| *times).unwrap();

    println!("Min minute: {}, max guard: {}", max_minute, sleepiest_guard_id);

    (sleepiest_guard_id * max_minute as i32).to_string()
}

pub fn star2(lines: &Vec<String>) -> String {
    let days = initialize(lines);

    let mut guard_map = HashMap::new();
    for day in days {
        guard_map.entry(day.guard_id)
            .or_insert(vec![])
            .push(day);
    }

    let mut max_guard_asleep_per_minute = vec![(0, None); 60];
    for &guard_id in guard_map.keys() {
        let mut guard_asleep_by_minute = vec![0; 60];
        for day in &guard_map[&guard_id] {
            for minute in 0..60 {
                guard_asleep_by_minute[minute] += i32::from(!day.minutes[minute]);
            }
        }
        for minute in 0..60 {
            if max_guard_asleep_per_minute[minute].0 < guard_asleep_by_minute[minute] {
                max_guard_asleep_per_minute[minute] = (guard_asleep_by_minute[minute], Some(guard_id));
            }
        }
    }
    if let Some((max_minute, (_, Some(max_guard_id)))) = max_guard_asleep_per_minute.iter().enumerate().max_by_key(|(_, (times, _))| times) {
        return (max_minute as i32 * max_guard_id) .to_string();
    }
    panic!("No maximum found: Invalid input!");
}