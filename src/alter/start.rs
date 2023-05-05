use chrono::{Datelike, Timelike};

use crate::Carbono;

impl Carbono {
    pub fn start_year(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_month(1)
                .unwrap()
                .with_day(1)
                .unwrap()
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
        }
    }

    pub fn start_month(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_day(1)
                .unwrap()
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
        }
    }

    pub fn start_day(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
        }
    }

    pub fn start_hour(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
        }
    }

    pub fn start_minute(&self) -> Self {
        Carbono {
            datetime: self.datetime
                .with_second(0)
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_move_to_the_start_of_the_year() {
        let carbono = Carbono::now().start_year();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 1, 1, 0, 0, 0).unwrap());
    }

    #[test]
    fn it_can_move_to_the_start_of_the_month() {
        let carbono = Carbono::now().start_month();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 1, 0, 0, 0).unwrap());
    }

    #[test]
    fn it_can_move_to_the_start_of_the_day() {
        let carbono = Carbono::now().start_day();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 0, 0, 0).unwrap());
    }

    #[test]
    fn it_can_move_to_the_start_of_the_hour() {
        let carbono = Carbono::now().start_hour();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 0, 0).unwrap());
    }

    #[test]
    fn it_can_move_to_the_start_of_the_minute() {
        let carbono = Carbono::now().start_minute();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 0).unwrap());
    }
}
