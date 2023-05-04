//! # Carbono
//!
//! Carbono Dive is a simple Rust API extension for Datetime.
//!
//! ```ini
//! [dependencies]
//! carbono = "0.1"
//! ```
//! ## Getting Started
//!
//! ```rust
//! use carbono::Carbono;
//!
//! fn main() {
//!     let carbono = Carbono::now(); // 1999-12-31 23:59:59
//!
//!     assert_eq!(carbono.year(), 1999);
//!     assert_eq!(carbono.month(), 12);
//!     assert_eq!(carbono.day(), 31);
//!     assert_eq!(carbono.hour(), 23);
//!     assert_eq!(carbono.minute(), 59);
//!     assert_eq!(carbono.second(), 59);
//! }
//! ```
//!
mod impls;
use chrono::prelude::*;

pub struct Carbono {
    pub datetime: DateTime<Utc>,
}

impl Carbono {
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
