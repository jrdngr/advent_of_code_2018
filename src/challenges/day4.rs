use std::cmp::{Ordering, PartialOrd};
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
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

#[derive(Debug)]
pub enum Event {
    ShiftBegins(Date, u16),
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
            let id = captures[3].parse::<u16>()?;
            Ok(Event::ShiftBegins(date, id))
        }
    }
}

pub fn day4_1() {
    let inputs: Vec<String> = crate::utils::get_inputs(4);

    let mut events: Vec<Event> = inputs
        .iter()
        .take(20)
        .map(|i| Event::from_str(i).unwrap())
        .collect();

    events.sort_by(|e1, e2| e1.date().cmp(e2.date()));
}
pub fn day4_2() {}
