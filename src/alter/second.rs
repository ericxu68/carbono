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

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 12, 30, 1).unwrap());
    }

    #[test]
    fn it_can_subtract_a_second() {
        let carbono = Carbono::now().sub_second();

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 12, 29, 59).unwrap());
    }

    #[test]
    fn it_can_add_seconds() {
        let carbono = Carbono::now().add_seconds(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 12, 30, 10).unwrap());
    }

    #[test]
    fn it_can_subtract_seconds() {
        let carbono = Carbono::now().sub_seconds(10);

        assert_eq!(carbono.datetime, Utc.with_ymd_and_hms(2022, 12, 15, 12, 29, 50).unwrap());
    }
}
