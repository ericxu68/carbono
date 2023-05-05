use crate::Carbono;

impl Carbono {
    pub fn add_day(&self) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::days(1)
        }
    }

    pub fn sub_day(&self) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::days(1)
        }
    }

    pub fn add_days(&self, days: i64) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::days(days)
        }
    }

    pub fn sub_days(&self, days: i64) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::days(days)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_a_day() {
        let carbono = Carbono::now().add_day();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 16, 12, 30, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_a_day() {
        let carbono = Carbono::now().sub_day();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 14, 12, 30, 0).unwrap());
    }

    #[test]
    fn it_can_add_days() {
        let carbono = Carbono::now().add_days(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 25, 12, 30, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_days() {
        let carbono = Carbono::now().sub_days(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 5, 12, 30, 0).unwrap());
    }
}
