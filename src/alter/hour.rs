use crate::Carbono;

impl Carbono {
    pub fn add_hour(&self) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::hours(1)
        }
    }

    pub fn sub_hour(&self) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::hours(1)
        }
    }

    pub fn add_hours(&self, hours: i64) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::hours(hours)
        }
    }

    pub fn sub_hours(&self, hours: i64) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::hours(hours)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_an_hour() {
        let carbono = Carbono::now().add_hour();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 1, 0, 59, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_an_hour() {
        let carbono = Carbono::now().sub_hour();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 22, 59, 59).unwrap());
    }

    #[test]
    fn it_can_add_hours() {
        let carbono = Carbono::now().add_hours(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 1, 9, 59, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_hours() {
        let carbono = Carbono::now().sub_hours(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 13, 59, 59).unwrap());
    }
}
