use crate::Carbono;
use chrono::{prelude::*, IsoWeek};

impl Carbono {
    pub fn timestamp(&self) -> i64 {
        self.datetime.timestamp()
    }

    pub fn rfc3339(&self) -> String {
        self.datetime.to_rfc3339()
    }

    pub fn rfc2822(&self) -> String {
        self.datetime.to_rfc2822()
    }

    pub fn year(&self) -> i32 {
        self.datetime.year()
    }

    pub fn month(&self) -> u32 {
        self.datetime.month()
    }

    pub fn day(&self) -> u32 {
        self.datetime.day()
    }

    pub fn hour(&self) -> u32 {
        self.datetime.hour()
    }

    pub fn minute(&self) -> u32 {
        self.datetime.minute()
    }

    pub fn second(&self) -> u32 {
        self.datetime.second()
    }

    pub fn date(&self) -> String {
        self.datetime.date_naive().to_string()
    }

    pub fn time(&self) -> String {
        self.datetime.time().to_string()
    }

    pub fn weekday(&self) -> u8 {
        self.datetime.weekday() as u8
    }

    pub fn iso_week(&self) -> IsoWeek {
        self.datetime.iso_week()
    }

    pub fn is_past(&self) -> bool {
        let now = Utc::now();

        self.datetime.lt(&now)
    }

    pub fn is_future(&self) -> bool {
        let now = Utc::now();

        self.datetime.ge(&now)
    }

    #[cfg(not(any(test, feature = "testing")))]
    pub fn is_today(&self) -> bool {
        let today = Utc::now().date_naive();

        self.datetime.date_naive() == today
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn is_today(&self) -> bool {
        let today = Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 59)
            .unwrap()
            .date_naive();

        self.datetime.date_naive() == today
    }

    pub fn is_monday(&self) -> bool {
        self.weekday() == Weekday::Mon as u8
    }

    pub fn is_tuesday(&self) -> bool {
        self.weekday() == Weekday::Tue as u8
    }

    pub fn is_wednesday(&self) -> bool {
        self.weekday() == Weekday::Wed as u8
    }

    pub fn is_thursday(&self) -> bool {
        self.weekday() == Weekday::Thu as u8
    }

    pub fn is_friday(&self) -> bool {
        self.weekday() == Weekday::Fri as u8
    }

    pub fn is_saturday(&self) -> bool {
        self.weekday() == Weekday::Sat as u8
    }

    pub fn is_sunday(&self) -> bool {
        self.weekday() == Weekday::Sun as u8
    }
}
