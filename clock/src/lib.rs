use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,

}

impl Clock {
    const MINUTES_PER_DAY: i32 = 24 * 60;
    pub fn new(hours: i32, minutes: i32) -> Self {
        let get_postive_minutes = Self::get_postive_minutes(hours,minutes);
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let clock = Clock {
            hours: (get_postive_minutes%Clock::MINUTES_PER_DAY)/60,
            minutes: (get_postive_minutes%Clock::MINUTES_PER_DAY)%60,
        };
        return clock;
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours,self.minutes+minutes)
        
    }

    fn get_postive_minutes(hours:i32,minutes:i32) -> i32 {
        let total_minutes: i32 = hours * 60 + minutes;
        let get_postive_minutes = total_minutes.rem_euclid(Clock::MINUTES_PER_DAY);
        return ((total_minutes%Clock::MINUTES_PER_DAY)+Clock::MINUTES_PER_DAY)%Clock::MINUTES_PER_DAY;
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}