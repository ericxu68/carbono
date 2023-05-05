use crate::Carbono;
use chronoutil::RelativeDuration;

impl Carbono {
    pub fn add_year(&self) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::years(1)
        }
    }

    pub fn sub_year(&self) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::years(-1)
        }
    }

    pub fn add_years(&self, years: i32) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::years(years)
        }
    }

    pub fn sub_years(&self, years: i32) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::years(-years)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_a_year() {
        let carbono = Carbono::now().add_year();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2023, 12, 15, 12, 30, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_a_year() {
        let carbono = Carbono::now().sub_year();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2021, 12, 15, 12, 30, 0).unwrap());
    }

    #[test]
    fn it_can_add_years() {
        let carbono = Carbono::now().add_years(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2032, 12, 15, 12, 30, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_years() {
        let carbono = Carbono::now().sub_years(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2012, 12, 15, 12, 30, 0).unwrap());
    }
}
