use crate::Carbono;

impl Carbono {
    pub fn add_week(&self) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::days(7)
        }
    }

    pub fn sub_week(&self) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::days(7)
        }
    }

    pub fn add_weeks(&self, days: i64) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::days(days * 7)
        }
    }

    pub fn sub_weeks(&self, days: i64) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::days(days * 7)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_a_week() {
        let carbono = Carbono::now().add_week();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 7, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_a_week() {
        let carbono = Carbono::now().sub_week();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 24, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_add_weeks() {
        let carbono = Carbono::now().add_weeks(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 3, 10, 23, 59, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_weeks() {
        let carbono = Carbono::now().sub_weeks(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 10, 22, 23, 59, 59).unwrap());
    }
}
