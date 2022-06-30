use std::fmt::{Display, Result, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let sum_minus = hours * 60 + minutes;
        Clock {
            hours: sum_minus.div_euclid(60).rem_euclid(24),
            minutes: minutes.rem_euclid(60)
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Clock::new (self.hours,self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours,self.minutes)
    }
}
