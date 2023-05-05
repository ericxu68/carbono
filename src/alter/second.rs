use crate::Carbono;

impl Carbono {
    pub fn add_second(&self) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::seconds(1)
        }
    }

    pub fn sub_second(&self) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::seconds(1)
        }
    }

    pub fn add_seconds(&self, seconds: i64) -> Self {
        Carbono {
            datetime: self.datetime + chrono::Duration::seconds(seconds)
        }
    }

    pub fn sub_seconds(&self, seconds: i64) -> Self {
        Carbono {
            datetime: self.datetime - chrono::Duration::seconds(seconds)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use super::*;

    #[test]
    fn it_can_add_a_second() {
        let carbono = Carbono::now().add_second();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap());
    }

    #[test]
    fn it_can_subtract_a_second() {
        let carbono = Carbono::now().sub_second();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 58).unwrap());
    }

    #[test]
    fn it_can_add_seconds() {
        let carbono = Carbono::now().add_seconds(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 9).unwrap());
    }

    #[test]
    fn it_can_subtract_seconds() {
        let carbono = Carbono::now().sub_seconds(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(1999, 12, 31, 23, 59, 49).unwrap());
    }
}
