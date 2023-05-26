use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY);
        let hours = total_minutes.div_euclid(MINUTES_PER_HOUR);
        let minutes = total_minutes.rem_euclid(MINUTES_PER_HOUR);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
