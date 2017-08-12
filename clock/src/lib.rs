use std::fmt;

const HOURS_PER_DAY: i32 = 24;
const MINS_PER_HOUR: i32 = 60;

// this should all be unsigned, but the tests don't allow it
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let mut c = Clock {
            hours: hours,
            minutes: minutes,
        };
        c.rectify();
        c
    }

    pub fn add_minutes(&self, mins: i32) -> Clock {
        Clock::new(self.hours, self.minutes + mins)
    }

    fn rectify(&mut self) {
        self.hours += self.minutes / MINS_PER_HOUR;
        self.hours %= HOURS_PER_DAY;
        self.minutes %= MINS_PER_HOUR;

        if self.minutes < 0 {
            self.minutes += MINS_PER_HOUR;
            self.hours -= 1;
        }
        if self.hours < 0 {
            self.hours += HOURS_PER_DAY;
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
