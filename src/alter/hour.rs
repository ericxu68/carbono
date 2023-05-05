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

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 13, 30, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_an_hour() {
        let carbono = Carbono::now().sub_hour();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 11, 30, 0).unwrap());
    }

    #[test]
    fn it_can_add_hours() {
        let carbono = Carbono::now().add_hours(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 22, 30, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_hours() {
        let carbono = Carbono::now().sub_hours(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 2, 30, 0).unwrap());
    }
}
