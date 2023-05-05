//! # Carbono
//!
//! Carbono is a simple Rust API extension for chrono DateTime.
//!
//! ```ini
//! [dependencies]
//! carbono = "0.1"
//! ```
//!
//! ## Getting Started
//!
//! ```rust
//! use carbono::Carbono;
//!
//! fn main() {
//!     let carbono = Carbono::now(); // 1999-12-31 23:59:59
//!
//!     println!("{}", carbono); // 1999-12-31 23:59:59 UTC
//!
//!     assert_eq!(carbono.timestamp(), 946684799);
//!
//!     assert_eq!(carbono.rfc3339(), "1999-12-31T23:59:59+00:00");
//!     assert_eq!(carbono.rfc2822(), "Fri, 31 Dec 1999 23:59:59 +0000");
//!
//!     assert_eq!(carbono.year(), 1999);
//!     assert_eq!(carbono.month(), 12);
//!     assert_eq!(carbono.day(), 31);
//!     assert_eq!(carbono.hour(), 23);
//!     assert_eq!(carbono.minute(), 59);
//!     assert_eq!(carbono.second(), 59);
//!
//!     assert_eq!(carbono.date(), "1999-12-31");
//!     assert_eq!(carbono.time(), "23:59:59");
//!
//!     assert_eq!(format!("{:?}", carbono.iso_week()), "1999-W52");
//!
//!     assert_eq!(carbono.weekday(), 4); // Friday
//!
//!     assert_eq!(carbono.is_thursday(), false);
//!     assert_eq!(carbono.is_friday(), true);
//!     assert_eq!(carbono.is_saturday(), false);
//!
//!     assert_eq!(carbono.is_past(), true);
//!     assert_eq!(carbono.is_future(), false);
//!
//!     assert_eq!(carbono.is_today(), true); // 1999-12-31
//!
//!     assert_eq!(carbono.add_day().is_today(), false); // 2000-01-01
//!
//!     assert_eq!(carbono.add_year().add_month().date(), "2001-01-31");
//! }
//! ```
//!
//! ### Alter the date and/or time
//!
//! - `add_year()`
//! - `sub_year()`
//! - `add_years(years: i32)`
//! - `sub_years(years: i32)`
//! - `add_month()`
//! - `sub_month()`
//! - `add_months(months: i32)`
//! - `sub_months(months: i32)`
//! - `add_week()`
//! - `sub_week()`
//! - `add_weeks(weeks: i64)`
//! - `sub_weeks(weeks: i64)`
//! - `add_day()`
//! - `sub_day()`
//! - `add_days(days: i64)`
//! - `sub_days(days: i64)`
//! - `add_hour()`
//! - `sub_hour()`
//! - `add_hours(hours: i64)`
//! - `sub_hours(hours: i64)`
//! - `add_minute()`
//! - `sub_minute()`
//! - `add_minutes(minutes: i64)`
//! - `sub_minutes(minutes: i64)`
//! - `add_second()`
//! - `sub_second()`
//! - `add_seconds(seconds: i64)`
//! - `sub_seconds(seconds: i64)`

mod impls;
mod alter;

use std::fmt::Display;
use chrono::prelude::*;

pub struct Carbono {
    datetime: DateTime<Utc>,
}

impl Carbono {
    pub fn get(&self) -> DateTime<Utc> {
        self.datetime
    }

    #[cfg(not(any(test, feature = "testing")))]
    pub fn now() -> Self {
        Self {
            datetime: Utc::now(),
        }
    }

    #[cfg(any(test, feature = "testing"))]
    pub fn now() -> Self {
        Self {
            datetime: Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 59).unwrap(),
        }
    }

    pub fn create_date(year: i32, month: u32, day: u32) -> Self {
        Self {
            datetime: Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap(),
        }
    }
}

impl Display for Carbono {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.datetime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_an_object_with_the_current_datetime() {
        let carbono = Carbono::now();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_create_an_object_with_a_custom_datetime() {
        let carbono = Carbono::create_date(2023, 1, 1);
        let datetime_in_past = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        assert_eq!(carbono.datetime, datetime_in_past);
    }
}
