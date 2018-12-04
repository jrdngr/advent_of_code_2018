use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

pub type ID = u16;
pub type Hour = u8;
pub type Minute = u8;

pub type MinuteId = (Hour, Minute);

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: Hour,
    pub minute: Minute,
}

impl Date {
    pub fn minute_list(d1: &Date, d2: &Date) -> Vec<MinuteId> {
        let mut result = Vec::new();

        if d1 == d2 {
            return result;
        }

        let first = d1.min(d2);
        let second = d1.max(d2);

        if first.hour == 23 {
            if second.hour == 23 {
                for min in first.minute..second.minute {
                    result.push((23, min));
                }
            } else {
                for min in first.minute..60 {
                    result.push((23, min));
                }

                if second.minute > 0 {
                    for min in 0..second.minute {
                        result.push((0, min));
                    }
                }
            }
        } else {
            for min in first.minute..second.minute {
                result.push((0, min));
            }
        }

        result
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        if self.year != other.year {
            self.year.cmp(&other.year)
        } else if self.month != other.month {
            self.month.cmp(&other.month)
        } else if self.day != other.day {
            self.day.cmp(&other.day)
        } else if self.hour != other.hour {
            self.hour.cmp(&other.hour)
        } else if self.minute != other.minute {
            self.minute.cmp(&other.minute)
        } else {
            Ordering::Equal
        }
    }
}

impl FromStr for Date {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\[(\d+)\-(\d+)\-(\d+) (\d+):(\d+)\]").unwrap();
        let captures = re.captures(s).unwrap();

        Ok(Date {
            year: captures[1].parse::<u16>()?,
            month: captures[2].parse::<u8>()?,
            day: captures[3].parse::<u8>()?,
            hour: captures[4].parse::<u8>()?,
            minute: captures[5].parse::<u8>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    ShiftBegins(Date, ID),
    FallsAsleep(Date),
    WakesUp(Date),
}

impl Event {
    pub fn date(&self) -> &Date {
        match self {
            Event::ShiftBegins(d, _) => d,
            Event::FallsAsleep(d) => d,
            Event::WakesUp(d) => d,
        }
    }
}

impl FromStr for Event {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\[.+\]) (Guard #(\d+) begins shift|.+)").unwrap();
        let captures = re.captures(s).unwrap();

        let date = Date::from_str(&captures[1])?;

        if captures[2].starts_with("falls") {
            Ok(Event::FallsAsleep(date))
        } else if captures[2].starts_with("wakes") {
            Ok(Event::WakesUp(date))
        } else {
            let id = captures[3].parse::<ID>()?;
            Ok(Event::ShiftBegins(date, id))
        }
    }
}

fn minute_counts_by_id(mut events: Vec<Event>) -> HashMap<ID, HashMap<MinuteId, u64>> {
    events.sort_by(|e1, e2| e1.date().cmp(e2.date()));

    let mut result: HashMap<ID, HashMap<MinuteId, u64>> = HashMap::new();

    let mut current_id = 0;
    let mut start_date = Date::default();

    for event in events {
        match event {
            Event::ShiftBegins(_, id) => current_id = id,
            Event::FallsAsleep(date) => start_date = date.clone(),
            Event::WakesUp(date) => {
                let minutes = Date::minute_list(&date, &start_date);
                for minute in minutes {
                    let count_map = result.entry(current_id).or_insert(HashMap::new());
                    let count = count_map.entry(minute).or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    result
}

fn total_minutes(count_map: &HashMap<MinuteId, u64>) -> u64 {
    count_map.iter().map(|(_, count)| count).sum()
}

pub fn day4_1() {
    let inputs: Vec<String> = crate::utils::get_inputs(4);

    let events: Vec<Event> = inputs.iter().map(|i| Event::from_str(i).unwrap()).collect();
    let counts = minute_counts_by_id(events);

    let max_id = counts
        .iter()
        .max_by(|(_, c1), (_, c2)| {
            let min1 = total_minutes(c1);
            let min2 = total_minutes(c2);
            min1.cmp(&min2)
        })
        .unwrap()
        .0;

    let sleepy_guy_counts = &counts[max_id];

    let best_minute = sleepy_guy_counts
        .iter()
        .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
        .unwrap()
        .0;

    let answer = u64::from(*max_id) * u64::from(best_minute.1);

    println!("4-1: {}", answer);
}

pub fn day4_2() {
    let answer = 0;
    println!("4-2: {}", answer);
}
