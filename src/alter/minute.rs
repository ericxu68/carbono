use crate::Carbono;

impl Carbono {
    pub fn add_minute(&self) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::minutes(1)
        }
    }

    pub fn sub_minute(&self) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::minutes(1)
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::minutes(minutes)
        }
    }

    pub fn sub_minutes(&self, minutes: i64) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::minutes(minutes)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_a_minute() {
        let carbono = Carbono::now().add_minute();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_a_minute() {
        let carbono = Carbono::now().sub_minute();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 58, 59).unwrap());
    }

    #[test]
    fn it_can_add_minutes() {
        let carbono = Carbono::now().add_minutes(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 1, 0, 9, 59).unwrap());
    }

    #[test]
    fn it_can_subtract_minutes() {
        let carbono = Carbono::now().sub_minutes(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 49, 59).unwrap());
    }
}
