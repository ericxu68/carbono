use crate::Carbono;
use chronoutil::RelativeDuration;

impl Carbono {
    pub fn add_month(&self) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::months(1)
        }
    }

    pub fn sub_month(&self) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::months(-1)
        }
    }

    pub fn add_months(&self, months: i32) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::months(months)
        }
    }

    pub fn sub_months(&self, months: i32) -> Self {
        Carbono {
            datetime: self.datetime + RelativeDuration::months(-months)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_a_month() {
        let carbono = Carbono::now().add_month();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 31, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_a_month() {
        let carbono = Carbono::now().sub_month();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 11, 30, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_add_months() {
        let carbono = Carbono::now().add_months(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 10, 31, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_months() {
        let carbono = Carbono::now().sub_months(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 2, 28, 23, 59, 59).unwrap());
    }
}
