use crate::Carbono;
use chrono::prelude::*;

impl Carbono {
    pub fn year(&self) -> i32 {
        self.datetime.year()
    }

    pub fn month(&self) -> u32 {
        self.datetime.month()
    }

    pub fn day(&self) -> u32 {
        self.datetime.day()
    }

    pub fn hour(&self) -> u32 {
        self.datetime.hour()
    }

    pub fn minute(&self) -> u32 {
        self.datetime.minute()
    }

    pub fn second(&self) -> u32 {
        self.datetime.second()
    }
}
